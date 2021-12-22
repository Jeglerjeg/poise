use crate::serenity_prelude as serenity;

// Returns tuple of stripped prefix and rest of the message, if any prefix matches
async fn strip_prefix<'a, U, E>(
    framework: &'a crate::Framework<U, E>,
    ctx: &'a serenity::Context,
    msg: &'a serenity::Message,
) -> Option<(&'a str, &'a str)> {
    if let Some(dynamic_prefix) = framework.options.prefix_options.dynamic_prefix {
        let partial_ctx = crate::PartialContext {
            guild_id: msg.guild_id,
            channel_id: msg.channel_id,
            author: &msg.author,
            discord: ctx,
            framework,
            data: framework.get_user_data().await,
        };
        if let Some(prefix) = dynamic_prefix(partial_ctx).await {
            if msg.content.starts_with(&prefix) {
                return Some(msg.content.split_at(prefix.len()));
            }
        }
    }

    if let Some(prefix) = &framework.options.prefix_options.prefix {
        if let Some(content) = msg.content.strip_prefix(prefix) {
            return Some((prefix, content));
        }
    }

    if let Some((prefix, content)) = framework
        .options
        .prefix_options
        .additional_prefixes
        .iter()
        .find_map(|prefix| match prefix {
            &crate::Prefix::Literal(prefix) => Some((prefix, msg.content.strip_prefix(prefix)?)),
            crate::Prefix::Regex(prefix) => {
                let regex_match = prefix.find(&msg.content)?;
                if regex_match.start() == 0 {
                    Some(msg.content.split_at(regex_match.end()))
                } else {
                    None
                }
            }
        })
    {
        return Some((prefix, content));
    }

    if let Some(dynamic_prefix) = framework.options.prefix_options.stripped_dynamic_prefix {
        if let Some((prefix, content)) =
            dynamic_prefix(ctx, msg, framework.get_user_data().await).await
        {
            return Some((prefix, content));
        }
    }

    if framework.options.prefix_options.mention_as_prefix {
        // Mentions are either <@USER_ID> or <@!USER_ID>
        if let Some(stripped_content) = (|| {
            msg.content
                .strip_prefix("<@")?
                .trim_start_matches('!')
                .strip_prefix(&ctx.cache.current_user_id().0.to_string())?
                .strip_prefix('>')
        })() {
            let mention_prefix = &msg.content[..(msg.content.len() - stripped_content.len())];
            return Some((mention_prefix, stripped_content));
        }
    }

    None
}

/// Find a command within nested Command's by the user message string. Also returns
/// the arguments, i.e. the remaining string.
fn find_command<'a, U, E>(
    framework: &'a crate::Framework<U, E>,
    user_data: &'a U,
    ctx: &'a serenity::Context,
    msg: &'a serenity::Message,
    prefix: &'a str,
    commands: &'a [crate::Command<U, E>],
    remaining_message: &'a str,
) -> Option<(
    &'a crate::Command<U, E>,
    for<'b> fn(
        crate::PrefixContext<'b, U, E>,
        args: &'b str,
    ) -> crate::BoxFuture<'b, Result<(), crate::FrameworkError<'b, U, E>>>,
    &'a str,
)>
where
    U: Send + Sync,
{
    let considered_equal = if framework.options.prefix_options.case_insensitive_commands {
        |a: &str, b: &str| a.eq_ignore_ascii_case(b)
    } else {
        |a: &str, b: &str| a == b
    };

    let (command_name, remaining_message) = {
        let mut iter = remaining_message.splitn(2, char::is_whitespace);
        (iter.next().unwrap(), iter.next().unwrap_or("").trim_start())
    };

    for command in commands {
        let primary_name_matches = considered_equal(command.name, command_name);
        let alias_matches = command
            .aliases
            .iter()
            .any(|alias| considered_equal(alias, command_name));
        if !primary_name_matches && !alias_matches {
            continue;
        }

        let ctx = crate::PrefixContext {
            discord: ctx,
            msg,
            prefix,
            framework,
            data: user_data,
            command,
        };

        return Some(
            match find_command(
                framework,
                user_data,
                ctx.discord,
                msg,
                prefix,
                &command.subcommands,
                remaining_message,
            ) {
                Some(subcommand_result) => subcommand_result,
                None => match command.prefix_action {
                    Some(action) => (command, action, remaining_message),
                    None => continue,
                },
            },
        );
    }

    None
}

/// Manually dispatches a message with the prefix framework.
///
/// Returns:
/// - Ok(()) if a command was successfully dispatched and run
/// - Err(None) if no command was dispatched, for example if the message didn't contain a command or
///   the cooldown limits were reached
/// - Err(Some(error: UserError)) if any user code yielded an error
pub async fn dispatch_message<'a, U, E>(
    framework: &'a crate::Framework<U, E>,
    ctx: &'a serenity::Context,
    msg: &'a serenity::Message,
    triggered_by_edit: bool,
    previously_tracked: bool,
) -> Result<(), Option<(crate::FrameworkError<'a, U, E>, &'a crate::Command<U, E>)>>
where
    U: Send + Sync,
{
    // Strip prefix and whitespace between prefix and command
    let (prefix, msg_content) = strip_prefix(framework, ctx, msg).await.ok_or(None)?;
    let msg_content = msg_content.trim_start();

    // Check if we're allowed to execute our own messages
    let bot_id = ctx.cache.current_user_id();
    let execute_self_messages = framework.options.prefix_options.execute_self_messages;
    if bot_id == msg.author.id && !execute_self_messages {
        return Err(None);
    }

    let (command, action, args) = find_command(
        framework,
        framework.get_user_data().await,
        ctx,
        msg,
        prefix,
        &framework.options.commands,
        msg_content,
    )
    .ok_or(None)?;

    // Check if we should disregard this invocation if it was triggered by an edit
    let should_execute_if_triggered_by_edit = command.track_edits
        || (!previously_tracked && framework.options.prefix_options.execute_untracked_edits);
    if triggered_by_edit && !should_execute_if_triggered_by_edit {
        return Err(None);
    }

    let ctx = crate::PrefixContext {
        discord: ctx,
        msg,
        prefix,
        framework,
        data: framework.get_user_data().await,
        command,
    };

    super::common::check_permissions_and_cooldown(ctx.into(), command)
        .await
        .map_err(|e| Some((e, command)))?;

    // Typing is broadcasted as long as this object is alive
    let _typing_broadcaster = if command.broadcast_typing {
        msg.channel_id.start_typing(&ctx.discord.http).ok()
    } else {
        None
    };

    (framework.options.pre_command)(crate::Context::Prefix(ctx)).await;

    // Execute command
    let res = (action)(ctx, args).await.map_err(|e| Some((e, command)));

    (framework.options.post_command)(crate::Context::Prefix(ctx)).await;

    res
}