initSidebarItems({"attr":[["command","This macro transforms plain functions into poise bot commands."]],"derive":[["SlashChoiceParameter","Use this derive macro on an enum to easily generate a choice parameter type. A choice parameter is mainly useful in slash commands. It allows you to constrain input to a fixed set of choices."]],"enum":[["ApplicationCommandOrAutocompleteInteraction","Abstracts over a refernce to an application command interaction or autocomplete interaction"],["Context","Wrapper around either [`crate::ApplicationContext`] or [`crate::PrefixContext`]"],["ContextMenuCommandAction","Possible actions that a context menu entry can have"],["Event","This enum stores every possible event that a [`serenity::EventHandler`] can receive."],["FrameworkError","Any error that can occur while the bot runs. Either thrown by user code (those variants will have an `error` field with your error type `E` in it), or originating from within the framework."],["Prefix","Possible ways to define a command prefix"],["ReplyHandle","Returned from [`send_reply`] to retrieve the sent message object."],["SlashArgError","Possible errors when parsing slash command arguments"]],"fn":[["dispatch_message","Manually dispatches a message with the prefix framework."],["find_command","Find a command or subcommand within `&[Command]`, given a command invocation without a prefix. Returns the verbatim command name string as well as the command arguments (i.e. the remaining string)."],["say_reply","Shorthand of [`send_reply`] for text-only messages"],["send_application_reply","Send a response to an interaction (slash command or context menu command invocation)."],["send_prefix_reply","Prefix-specific reply function. For more details, see [`crate::send_reply`]."],["send_reply","Send a message in the given context: normal message if prefix command, interaction response if application command."]],"macro":[["autocomplete_argument_into_json","Full version of [`crate::Autocompletable::into_json`]."],["autocomplete_argument_into_json","Full version of [`crate::Autocompletable::into_json`]."],["create_slash_argument","Full version of [`crate::SlashArgument::create`]."],["create_slash_argument","Full version of [`crate::SlashArgument::create`]."],["extract_autocomplete_argument","Full version of [`crate::Autocompletable::extract_partial`]."],["extract_autocomplete_argument","Full version of [`crate::Autocompletable::extract_partial`]."],["extract_slash_argument","Full version of [`crate::SlashArgument::extract`]."],["extract_slash_argument","Full version of [`crate::SlashArgument::extract`]."],["parse_prefix_args","Macro for parsing an argument string into multiple parameter types."],["parse_prefix_args","Macro for parsing an argument string into multiple parameter types."],["parse_slash_args","Macro for extracting and parsing slash command arguments out of an array of [`serenity::ApplicationCommandInteractionDataOption`]."],["parse_slash_args","Macro for extracting and parsing slash command arguments out of an array of [`serenity::ApplicationCommandInteractionDataOption`]."],["pop_prefix_argument","Full version of [`crate::PopArgument::pop_from`]."],["pop_prefix_argument","Full version of [`crate::PopArgument::pop_from`]."]],"mod":[["builtins","Building blocks for common commands like help commands or application command registration"],["samples","See [`builtins`]"],["serenity_prelude","This module re-exports a bunch of items from all over serenity. Useful if you can’t remember the full paths of serenity items."]],"struct":[["ApplicationContext","Application command specific context passed to command invocations."],["AutocompleteChoice","A single autocomplete choice, displayed in Discord UI"],["CodeBlock","A command parameter type for Discord code blocks"],["CodeBlockError","Error thrown when parsing a malformed [`CodeBlock`] ([`CodeBlock::pop_from`])"],["Command","Type returned from `#[poise::command]` annotated functions, which contains all of the generated prefix and application commands"],["CommandParameter","A single parameter of a [`crate::Command`]"],["CooldownConfig","Configuration struct for [`Cooldowns`]"],["Cooldowns","Handles cooldowns for a single command"],["CreateReply","Message builder that abstracts over prefix and application command responses"],["EditTracker","Stores messages and the associated bot responses in order to implement poise’s edit tracking feature."],["EventWrapper","A [`serenity::EventHandler`] implementation that wraps every received event into the [`Event`] enum and propagates it to a callback."],["Framework","The main framework struct which stores all data and handles message and interaction dispatch."],["FrameworkBuilder","A builder to configure and run a framework."],["FrameworkOptions","Framework configuration"],["InvalidBool","Error thrown when the user enters a string that is not recognized as a boolean"],["InvalidChoice","Error thrown when the user enters a string that is not recognized by a SlashChoiceParameter-derived enum"],["KeyValueArgs","A command parameter type for key-value args"],["MissingAttachment","Error thrown in prefix invocation when there’s too few attachments"],["PartialContext","Trimmed down, more general version of [`Context`]"],["PrefixContext","Prefix-specific context passed to command invocations."],["PrefixFrameworkOptions","Prefix-specific framework configuration"],["TooFewArguments","Error thrown if user passes too few arguments to a command"],["TooManyArguments","Error thrown if user passes too many arguments to a command"]],"trait":[["Autocompletable","Types that can be marked autocompletable in a slash command parameter."],["ContextMenuParameter","Implemented for all types that can be used in a context menu command"],["PopArgument","Parse a value out of a string by popping off the front of the string. Discord message context is available for parsing, and IO may be done as part of the parsing."],["SlashArgument","Implement this trait on types that you want to use as a slash command parameter."]],"type":[["BoxFuture","Shorthand for a wrapped async future with a lifetime, used by many parts of this framework."]]});