//! # Command and Subcommand System
//!
//! This module provides a production-grade subcommand system for building complex CLI applications.
//! It supports subcommands (like `git commit`, `cargo build`), positional and flag arguments,
//! automatic help generation, validation, and advanced features like environment variable fallbacks.
//!
//! ## Features (v0.2.1)
//!
//! - **Subcommands**: Multi-level command hierarchies with aliases
//! - **Positional Arguments**: Required and optional positional args (`<FILE>`, `[OUTPUT]`)
//! - **Variadic Arguments**: Capture multiple values (`[FILES]...`)
//! - **Flags & Options**: Short/long flags with values (`-v`, `--verbose`, `--output file.txt`)
//! - **Environment Variables**: Automatic fallback to env vars (`.env("APP_CONFIG")`)
//! - **Value Delimiters**: Parse comma-separated values (`--tags rust,cli,tool`)
//! - **Argument Dependencies**: Require other arguments (`.requires("format")`)
//! - **Conflict Detection**: Prevent incompatible args (`.conflicts_with("quiet")`)
//! - **Argument Groups**: Mutually exclusive argument sets
//! - **Auto Help**: Automatic `--help` generation for all commands
//! - **Validation**: Required arguments, possible values, custom validators
//! - **Error Handling**: Detailed error messages for debugging
//! - **Cross-Platform**: Works on Linux, macOS, Windows
//!
//! ## Example
//!
//! ```rust
//! use zfish::command::{App, Command, Arg};
//!
//! let app = App::new("myapp")
//!     .version("1.0.0")
//!     .about("My awesome CLI")
//!     .arg(Arg::new("verbose").short('v').long("verbose"))
//!     .subcommand(
//!         Command::new("init")
//!             .about("Initialize a new project")
//!             .arg(Arg::new("name").required(true))
//!     )
//!     .subcommand(
//!         Command::new("build")
//!             .about("Build the project")
//!             .arg(Arg::new("release").long("release"))
//!     );
//!
//! let matches = app.get_matches();
//!
//! match matches.subcommand() {
//!     Some(("init", sub_matches)) => {
//!         let name = sub_matches.value_of("name").unwrap();
//!         println!("Initializing project: {}", name);
//!     }
//!     Some(("build", sub_matches)) => {
//!         if sub_matches.is_present("release") {
//!             println!("Building in release mode");
//!         }
//!     }
//!     _ => println!("No subcommand provided"),
//! }
//! ```

use std::collections::HashMap;
use std::fmt;

/// Represents a parsed command-line argument value
#[derive(Debug, Clone, PartialEq)]
pub enum ArgValue {
    /// A single string value
    Single(String),
    /// Multiple string values (for repeated arguments)
    Multiple(Vec<String>),
    /// A flag (present/absent)
    Flag(bool),
}

impl ArgValue {
    /// Returns the value as a string reference, if it's a single value
    pub fn as_str(&self) -> Option<&str> {
        match self {
            ArgValue::Single(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the value as a boolean, if it's a flag
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ArgValue::Flag(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the values as a slice, if multiple
    pub fn as_vec(&self) -> Option<&[String]> {
        match self {
            ArgValue::Multiple(v) => Some(v),
            _ => None,
        }
    }
}

/// Errors that can occur during command parsing
#[derive(Debug, Clone, PartialEq)]
pub enum CommandError {
    /// An argument is missing
    MissingArgument(String),
    /// An unknown argument was provided
    UnknownArgument(String),
    /// An unknown subcommand was provided
    UnknownSubcommand(String),
    /// An argument validation failed
    ValidationError(String, String), // (arg_name, error_message)
    /// Invalid value for an argument
    InvalidValue(String, String), // (arg_name, value)
    /// Help was requested
    HelpRequested,
    /// Version was requested
    VersionRequested,
    /// Arguments conflict with each other
    ArgumentConflict(String, String), // (arg1, arg2)
    /// Required dependency is missing
    MissingDependency(String, String), // (arg, required_arg)
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::MissingArgument(name) => {
                write!(f, "error: the argument '{}' is required", name)
            }
            CommandError::UnknownArgument(name) => {
                write!(f, "error: unknown argument '{}'", name)
            }
            CommandError::UnknownSubcommand(name) => {
                write!(f, "error: unknown subcommand '{}'", name)
            }
            CommandError::ValidationError(name, msg) => {
                write!(f, "error: validation failed for '{}': {}", name, msg)
            }
            CommandError::InvalidValue(name, value) => {
                write!(f, "error: invalid value '{}' for '{}'", value, name)
            }
            CommandError::HelpRequested => write!(f, "help requested"),
            CommandError::VersionRequested => write!(f, "version requested"),
            CommandError::ArgumentConflict(arg1, arg2) => {
                write!(
                    f,
                    "error: the argument '{}' cannot be used with '{}'",
                    arg1, arg2
                )
            }
            CommandError::MissingDependency(arg, required) => {
                write!(f, "error: the argument '{}' requires '{}'", arg, required)
            }
        }
    }
}

impl std::error::Error for CommandError {}

/// Result type for command operations
pub type CommandResult<T> = Result<T, CommandError>;

/// Represents a group of mutually exclusive arguments
#[derive(Debug, Clone)]
pub struct ArgGroup {
    name: String,
    args: Vec<String>,
    required: bool,
}

impl ArgGroup {
    /// Creates a new argument group
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            args: Vec::new(),
            required: false,
        }
    }

    /// Adds an argument to this group
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Adds multiple arguments to this group
    pub fn args(mut self, args: &[&str]) -> Self {
        for arg in args {
            self.args.push(arg.to_string());
        }
        self
    }

    /// Makes this group required (at least one argument must be present)
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

/// Represents a single command-line argument definition
#[derive(Debug, Clone)]
pub struct Arg {
    name: String,
    short: Option<char>,
    long: Option<String>,
    help: Option<String>,
    required: bool,
    takes_value: bool,
    multiple: bool,
    default_value: Option<String>,
    possible_values: Option<Vec<String>>,
    #[allow(clippy::type_complexity)]
    validator: Option<fn(&str) -> Result<(), String>>,
    // New fields for v0.2.1+
    index: Option<usize>,          // Position for positional arguments
    env: Option<String>,           // Environment variable name
    requires: Vec<String>,         // Arguments this arg depends on
    conflicts_with: Vec<String>,   // Arguments this arg conflicts with
    value_delimiter: Option<char>, // Delimiter for splitting values (e.g., ',')
    last: bool,                    // Variadic positional (FILES...)
}

impl Arg {
    /// Creates a new argument with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            short: None,
            long: None,
            help: None,
            required: false,
            takes_value: true,
            multiple: false,
            default_value: None,
            possible_values: Some(Vec::new()),
            validator: None,
            index: None,
            env: None,
            requires: Vec::new(),
            conflicts_with: Vec::new(),
            value_delimiter: None,
            last: false,
        }
    }

    /// Sets the short flag (e.g., `-v`)
    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    /// Sets the long flag (e.g., `--verbose`)
    pub fn long(mut self, long: impl Into<String>) -> Self {
        self.long = Some(long.into());
        self
    }

    /// Sets the help text for this argument
    pub fn about(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Marks this argument as required
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Sets whether this argument takes a value
    pub fn takes_value(mut self, takes_value: bool) -> Self {
        self.takes_value = takes_value;
        self
    }

    /// Allows this argument to be specified multiple times
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Sets the default value for this argument
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Sets the possible values for this argument
    pub fn possible_values(mut self, values: &[&str]) -> Self {
        self.possible_values = Some(values.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Sets a custom validator function
    pub fn validator(mut self, validator: fn(&str) -> Result<(), String>) -> Self {
        self.validator = Some(validator);
        self
    }

    /// Makes this a positional argument at the given index (0-based)
    /// Example: `Arg::new("file").index(0)` for `<FILE>`
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Sets an environment variable to read from if the argument is not provided
    /// Example: `Arg::new("config").env("APP_CONFIG")`
    pub fn env(mut self, env: impl Into<String>) -> Self {
        self.env = Some(env.into());
        self
    }

    /// Specifies arguments that this argument requires
    /// Example: `Arg::new("output").requires("format")`
    pub fn requires(mut self, arg: impl Into<String>) -> Self {
        self.requires.push(arg.into());
        self
    }

    /// Specifies arguments that conflict with this argument
    /// Example: `Arg::new("quiet").conflicts_with("verbose")`
    pub fn conflicts_with(mut self, arg: impl Into<String>) -> Self {
        self.conflicts_with.push(arg.into());
        self
    }

    /// Sets a delimiter for parsing multiple values from a single input
    /// Example: `Arg::new("tags").value_delimiter(',')` parses "rust,cli,tool"
    pub fn value_delimiter(mut self, delimiter: char) -> Self {
        self.value_delimiter = Some(delimiter);
        self.multiple = true; // Automatically enable multiple
        self
    }

    /// Makes this a variadic positional argument (captures all remaining args)
    /// Example: `Arg::new("files").last(true)` for `[FILES]...`
    pub fn last(mut self, last: bool) -> Self {
        self.last = last;
        if last {
            self.multiple = true;
            self.index = Some(usize::MAX); // Marker for last position
        }
        self
    }

    /// Gets the name of this argument
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Checks if this argument matches a short flag
    fn matches_short(&self, c: char) -> bool {
        self.short == Some(c)
    }

    /// Validates a value against this argument's constraints
    fn validate(&self, value: &str) -> Result<(), String> {
        // Check possible values
        if let Some(ref possible) = self.possible_values
            && !possible.is_empty()
            && !possible.contains(&value.to_string())
        {
            return Err(format!(
                "invalid value '{}', expected one of: {}",
                value,
                possible.join(", ")
            ));
        }

        // Run custom validator
        if let Some(validator) = self.validator {
            validator(value)?;
        }

        Ok(())
    }
}

/// Represents the result of parsing a command
#[derive(Debug, Clone)]
pub struct ArgMatches {
    command_name: String,
    args: HashMap<String, ArgValue>,
    subcommand: Option<Box<(String, ArgMatches)>>,
}

impl ArgMatches {
    /// Creates a new empty ArgMatches
    fn new(command_name: impl Into<String>) -> Self {
        Self {
            command_name: command_name.into(),
            args: HashMap::new(),
            subcommand: None,
        }
    }

    /// Gets the name of this command
    pub fn command_name(&self) -> &str {
        &self.command_name
    }

    /// Checks if an argument is present
    pub fn is_present(&self, name: &str) -> bool {
        self.args.contains_key(name)
    }

    /// Gets the value of an argument as a string
    pub fn value_of(&self, name: &str) -> Option<&str> {
        self.args.get(name).and_then(|v| v.as_str())
    }

    /// Gets the value of an argument as a boolean
    pub fn is_flag_set(&self, name: &str) -> bool {
        self.args
            .get(name)
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Gets multiple values for an argument
    pub fn values_of(&self, name: &str) -> Option<&[String]> {
        self.args.get(name).and_then(|v| v.as_vec())
    }

    /// Gets the subcommand, if any
    pub fn subcommand(&self) -> Option<(&str, &ArgMatches)> {
        self.subcommand
            .as_ref()
            .map(|boxed| (boxed.0.as_str(), &boxed.1))
    }

    /// Gets the subcommand name, if any
    pub fn subcommand_name(&self) -> Option<&str> {
        self.subcommand.as_ref().map(|boxed| boxed.0.as_str())
    }

    /// Gets the subcommand matches, if any
    pub fn subcommand_matches(&self, name: &str) -> Option<&ArgMatches> {
        self.subcommand.as_ref().and_then(|boxed| {
            if boxed.0 == name {
                Some(&boxed.1)
            } else {
                None
            }
        })
    }

    /// Inserts an argument value
    fn insert(&mut self, name: String, value: ArgValue) {
        self.args.insert(name, value);
    }

    /// Sets the subcommand
    fn set_subcommand(&mut self, name: String, matches: ArgMatches) {
        self.subcommand = Some(Box::new((name, matches)));
    }
}

/// Represents a command or subcommand
#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    about: Option<String>,
    long_about: Option<String>,
    version: Option<String>,
    args: Vec<Arg>,
    subcommands: Vec<Command>,
    groups: Vec<ArgGroup>,
    aliases: Vec<String>,
}

impl Command {
    /// Creates a new command with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            about: None,
            long_about: None,
            version: None,
            args: Vec::new(),
            subcommands: Vec::new(),
            groups: Vec::new(),
            aliases: Vec::new(),
        }
    }

    /// Sets the short description for this command
    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.about = Some(about.into());
        self
    }

    /// Sets the long description for this command
    pub fn long_about(mut self, long_about: impl Into<String>) -> Self {
        self.long_about = Some(long_about.into());
        self
    }

    /// Sets the version for this command
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Adds an argument to this command
    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    /// Adds multiple arguments to this command
    pub fn args(mut self, args: &[Arg]) -> Self {
        self.args.extend_from_slice(args);
        self
    }

    /// Adds a subcommand to this command
    pub fn subcommand(mut self, subcommand: Command) -> Self {
        self.subcommands.push(subcommand);
        self
    }

    /// Adds multiple subcommands to this command
    pub fn subcommands(mut self, subcommands: &[Command]) -> Self {
        self.subcommands.extend_from_slice(subcommands);
        self
    }

    /// Adds an argument group to this command
    pub fn group(mut self, group: ArgGroup) -> Self {
        self.groups.push(group);
        self
    }

    /// Adds an alias for this command
    /// Example: `Command::new("build").alias("b")`
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Adds multiple aliases for this command
    pub fn aliases(mut self, aliases: &[&str]) -> Self {
        for alias in aliases {
            self.aliases.push(alias.to_string());
        }
        self
    }

    /// Gets the name of this command
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Finds an argument by name, short, or long flag
    fn find_arg(&self, identifier: &str) -> Option<&Arg> {
        self.args.iter().find(|arg| {
            arg.name == identifier
                || arg.short.map(|c| format!("{}", c)) == Some(identifier.to_string())
                || arg.long.as_deref() == Some(identifier)
        })
    }

    /// Finds a subcommand by name or alias
    fn find_subcommand(&self, name: &str) -> Option<&Command> {
        self.subcommands
            .iter()
            .find(|cmd| cmd.name == name || cmd.aliases.contains(&name.to_string()))
    }

    /// Generates help text for this command
    pub fn generate_help(&self) -> String {
        let mut help = String::new();

        self.generate_header(&mut help);
        self.generate_usage(&mut help);
        self.generate_args_section(&mut help);
        self.generate_options_section(&mut help);
        self.generate_subcommands_section(&mut help);

        help
    }

    /// Generate header section (about and version)
    fn generate_header(&self, help: &mut String) {
        if let Some(ref about) = self.about {
            help.push_str(&format!("{}\n", about));
        }

        if let Some(ref version) = self.version {
            help.push_str(&format!("\nVersion: {}\n", version));
        }
    }

    /// Generate usage line
    fn generate_usage(&self, help: &mut String) {
        help.push_str(&format!("\nUSAGE:\n    {}", self.name));

        let mut positional_args: Vec<&Arg> =
            self.args.iter().filter(|a| a.index.is_some()).collect();
        positional_args.sort_by_key(|a| a.index.unwrap());

        if self.args.iter().any(|a| a.index.is_none()) {
            help.push_str(" [OPTIONS]");
        }

        for arg in &positional_args {
            self.append_positional_usage(arg, help);
        }

        if !self.subcommands.is_empty() {
            help.push_str(" <COMMAND>");
        }

        help.push('\n');
    }

    /// Append single positional arg to usage line
    fn append_positional_usage(&self, arg: &Arg, help: &mut String) {
        if arg.last {
            help.push_str(&format!(" [{}]...", arg.name.to_uppercase()));
        } else if arg.required {
            help.push_str(&format!(" <{}>", arg.name.to_uppercase()));
        } else {
            help.push_str(&format!(" [{}]", arg.name.to_uppercase()));
        }
    }

    /// Generate ARGS section for positional arguments
    fn generate_args_section(&self, help: &mut String) {
        let mut positional_args: Vec<&Arg> =
            self.args.iter().filter(|a| a.index.is_some()).collect();
        positional_args.sort_by_key(|a| a.index.unwrap());

        if positional_args.is_empty() {
            return;
        }

        help.push_str("\nARGS:\n");
        for arg in &positional_args {
            self.format_arg_line(arg, help);
        }
    }

    /// Format a single positional argument line
    fn format_arg_line(&self, arg: &Arg, help: &mut String) {
        let mut arg_line = format!("    <{}>", arg.name.to_uppercase());

        while arg_line.len() < 30 {
            arg_line.push(' ');
        }

        if let Some(ref help_text) = arg.help {
            arg_line.push_str(help_text);
        }

        if arg.required {
            arg_line.push_str(" [required]");
        }

        help.push_str(&format!("{}\n", arg_line));
    }

    /// Generate OPTIONS section for flags and options
    fn generate_options_section(&self, help: &mut String) {
        let option_args: Vec<&Arg> = self.args.iter().filter(|a| a.index.is_none()).collect();

        if option_args.is_empty() {
            return;
        }

        help.push_str("\nOPTIONS:\n");
        for arg in &option_args {
            self.format_option_line(arg, help);
        }
    }

    /// Format a single option/flag line
    fn format_option_line(&self, arg: &Arg, help: &mut String) {
        let mut arg_line = String::from("    ");

        if let Some(short) = arg.short {
            arg_line.push_str(&format!("-{}", short));
            if arg.long.is_some() {
                arg_line.push_str(", ");
            }
        }

        if let Some(ref long) = arg.long {
            arg_line.push_str(&format!("--{}", long));
        }

        if arg.takes_value {
            arg_line.push_str(&format!(" <{}>", arg.name.to_uppercase()));
        }

        while arg_line.len() < 30 {
            arg_line.push(' ');
        }

        self.append_option_metadata(arg, &mut arg_line);
        help.push_str(&format!("{}\n", arg_line));
    }

    /// Append help text, required flag, and default value to option line
    fn append_option_metadata(&self, arg: &Arg, arg_line: &mut String) {
        if let Some(ref help_text) = arg.help {
            arg_line.push_str(help_text);
        }

        if arg.required {
            arg_line.push_str(" [required]");
        }

        if let Some(ref default) = arg.default_value {
            arg_line.push_str(&format!(" [default: {}]", default));
        }
    }

    /// Generate COMMANDS section for subcommands
    fn generate_subcommands_section(&self, help: &mut String) {
        if self.subcommands.is_empty() {
            return;
        }

        help.push_str("\nCOMMANDS:\n");
        for subcmd in &self.subcommands {
            self.format_subcommand_line(subcmd, help);
        }

        help.push_str("\nRun '<COMMAND> --help' for more information on a specific command.\n");
    }

    /// Format a single subcommand line
    fn format_subcommand_line(&self, subcmd: &Command, help: &mut String) {
        let mut cmd_line = format!("    {}", subcmd.name);

        if !subcmd.aliases.is_empty() {
            cmd_line.push_str(&format!(" ({})", subcmd.aliases.join(", ")));
        }

        while cmd_line.len() < 30 {
            cmd_line.push(' ');
        }

        if let Some(ref about) = subcmd.about {
            cmd_line.push_str(about);
        }

        help.push_str(&format!("{}\n", cmd_line));
    }

    /// Helper: Process a value with delimiter support
    fn process_value(&self, arg: &Arg, value: &str, matches: &mut ArgMatches) -> CommandResult<()> {
        if let Some(delimiter) = arg.value_delimiter {
            // Split by delimiter
            let values: Vec<String> = value
                .split(delimiter)
                .map(|s| s.trim().to_string())
                .collect();
            // Validate each value
            for v in &values {
                arg.validate(v)
                    .map_err(|err| CommandError::ValidationError(arg.name.clone(), err))?;
            }
            matches.insert(arg.name.clone(), ArgValue::Multiple(values));
        } else if arg.multiple {
            // Accumulate multiple values
            let current = matches
                .args
                .entry(arg.name.clone())
                .or_insert(ArgValue::Multiple(Vec::new()));
            if let ArgValue::Multiple(vec) = current {
                arg.validate(value)
                    .map_err(|err| CommandError::ValidationError(arg.name.clone(), err))?;
                vec.push(value.to_string());
            }
        } else {
            // Single value
            arg.validate(value)
                .map_err(|err| CommandError::ValidationError(arg.name.clone(), err))?;
            matches.insert(arg.name.clone(), ArgValue::Single(value.to_string()));
        }
        Ok(())
    }

    /// Parses command-line arguments
    fn parse_args(&self, args: &[String]) -> CommandResult<ArgMatches> {
        let mut matches = ArgMatches::new(&self.name);
        let mut positional_values: Vec<String> = Vec::new();

        self.parse_command_line(args, &mut matches, &mut positional_values)?;
        self.process_positional_args(&positional_values, &mut matches);
        self.validate_matches(&mut matches)?;

        Ok(matches)
    }

    /// Parse command line arguments (flags, options, subcommands)
    fn parse_command_line(
        &self,
        args: &[String],
        matches: &mut ArgMatches,
        positional_values: &mut Vec<String>,
    ) -> CommandResult<()> {
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            // Check for special flags
            if self.handle_special_flags(arg)? {
                return Ok(());
            }

            // Check for subcommand
            if !arg.starts_with('-') {
                if let Some(subcmd) = self.find_subcommand(arg) {
                    let sub_args = &args[i + 1..];
                    let sub_matches = subcmd.parse_args(sub_args)?;
                    matches.set_subcommand(arg.clone(), sub_matches);
                    return Ok(());
                }
                positional_values.push(arg.clone());
                i += 1;
                continue;
            }

            i += self.parse_flag_or_option(arg, args, i, matches)?;
        }

        Ok(())
    }

    /// Handle special flags like --help and --version
    fn handle_special_flags(&self, arg: &str) -> CommandResult<bool> {
        if arg == "--help" || arg == "-h" {
            return Err(CommandError::HelpRequested);
        }
        if (arg == "--version" || arg == "-V") && self.version.is_some() {
            return Err(CommandError::VersionRequested);
        }
        Ok(false)
    }

    /// Parse a single flag or option and return number of args consumed
    fn parse_flag_or_option(
        &self,
        arg: &str,
        args: &[String],
        index: usize,
        matches: &mut ArgMatches,
    ) -> CommandResult<usize> {
        if arg.starts_with("--") && arg.contains('=') {
            self.parse_long_flag_with_equals(arg, matches)?;
            Ok(1)
        } else if arg.starts_with("--") {
            self.parse_long_flag(arg, args, index, matches)
        } else if arg.starts_with('-') && arg.len() > 1 {
            self.parse_short_flags(arg, args, index, matches)
        } else {
            Ok(1)
        }
    }

    /// Parse long flag with = (--flag=value)
    fn parse_long_flag_with_equals(
        &self,
        arg: &str,
        matches: &mut ArgMatches,
    ) -> CommandResult<()> {
        let parts: Vec<&str> = arg.splitn(2, '=').collect();
        let flag_name = parts[0].trim_start_matches("--");
        let value = parts[1];

        if let Some(found_arg) = self.find_arg(flag_name) {
            self.process_value(found_arg, value, matches)?;
        } else {
            return Err(CommandError::UnknownArgument(flag_name.to_string()));
        }
        Ok(())
    }

    /// Parse long flag (--flag value or --flag)
    fn parse_long_flag(
        &self,
        arg: &str,
        args: &[String],
        index: usize,
        matches: &mut ArgMatches,
    ) -> CommandResult<usize> {
        let flag_name = arg.trim_start_matches("--");
        let found_arg = self
            .find_arg(flag_name)
            .ok_or_else(|| CommandError::UnknownArgument(flag_name.to_string()))?;

        if found_arg.takes_value {
            if index + 1 < args.len() && !args[index + 1].starts_with('-') {
                self.process_value(found_arg, &args[index + 1], matches)?;
                Ok(2) // Consumed current + next
            } else if let Some(ref default) = found_arg.default_value {
                matches.insert(found_arg.name.clone(), ArgValue::Single(default.clone()));
                Ok(1)
            } else {
                Ok(1)
            }
        } else {
            matches.insert(found_arg.name.clone(), ArgValue::Flag(true));
            Ok(1)
        }
    }

    /// Parse short flag(s) (-v or -abc)
    fn parse_short_flags(
        &self,
        arg: &str,
        args: &[String],
        index: usize,
        matches: &mut ArgMatches,
    ) -> CommandResult<usize> {
        let flags = arg.trim_start_matches('-');
        let mut consumed = 1;

        for (idx, c) in flags.chars().enumerate() {
            let found_arg = self
                .args
                .iter()
                .find(|a| a.matches_short(c))
                .ok_or_else(|| CommandError::UnknownArgument(c.to_string()))?;

            if found_arg.takes_value && idx == flags.len() - 1 {
                // Last flag can take value from next arg
                if index + 1 < args.len() && !args[index + 1].starts_with('-') {
                    self.process_value(found_arg, &args[index + 1], matches)?;
                    consumed = 2;
                } else if let Some(ref default) = found_arg.default_value {
                    matches.insert(found_arg.name.clone(), ArgValue::Single(default.clone()));
                }
            } else {
                matches.insert(found_arg.name.clone(), ArgValue::Flag(true));
            }
        }

        Ok(consumed)
    }

    /// Process positional arguments
    fn process_positional_args(&self, positional_values: &[String], matches: &mut ArgMatches) {
        let mut positional_args: Vec<&Arg> =
            self.args.iter().filter(|a| a.index.is_some()).collect();
        positional_args.sort_by_key(|a| a.index.unwrap());

        for (idx, arg) in positional_args.iter().enumerate() {
            if arg.last {
                let remaining: Vec<String> = positional_values.iter().skip(idx).cloned().collect();
                if !remaining.is_empty() {
                    matches.insert(arg.name.clone(), ArgValue::Multiple(remaining));
                }
            } else if idx < positional_values.len() {
                matches.insert(
                    arg.name.clone(),
                    ArgValue::Single(positional_values[idx].clone()),
                );
            }
        }
    }

    /// Validate matches: check required args, apply defaults, check dependencies
    fn validate_matches(&self, matches: &mut ArgMatches) -> CommandResult<()> {
        self.check_required_args(matches)?;
        self.apply_defaults_and_env(matches)?;
        self.check_dependencies(matches)?;
        self.check_conflicts(matches)?;
        self.validate_groups(matches)?;
        Ok(())
    }

    /// Check for required arguments
    fn check_required_args(&self, matches: &ArgMatches) -> CommandResult<()> {
        for arg in &self.args {
            if arg.required && !matches.is_present(&arg.name) {
                return Err(CommandError::MissingArgument(arg.name.clone()));
            }
        }
        Ok(())
    }

    /// Apply default values and environment variables
    fn apply_defaults_and_env(&self, matches: &mut ArgMatches) -> CommandResult<()> {
        for arg in &self.args {
            if !matches.is_present(&arg.name) {
                if let Some(ref env_var) = arg.env
                    && let Ok(value) = std::env::var(env_var)
                {
                    matches.insert(arg.name.clone(), ArgValue::Single(value));
                    continue;
                }
                if let Some(ref default) = arg.default_value {
                    matches.insert(arg.name.clone(), ArgValue::Single(default.clone()));
                }
            }
        }
        Ok(())
    }

    /// Check argument dependencies
    fn check_dependencies(&self, matches: &ArgMatches) -> CommandResult<()> {
        for arg in &self.args {
            if matches.is_present(&arg.name) {
                for required in &arg.requires {
                    if !matches.is_present(required) {
                        return Err(CommandError::MissingDependency(
                            arg.name.clone(),
                            required.clone(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Check argument conflicts
    fn check_conflicts(&self, matches: &ArgMatches) -> CommandResult<()> {
        for arg in &self.args {
            if matches.is_present(&arg.name) {
                for conflict in &arg.conflicts_with {
                    if matches.is_present(conflict) {
                        return Err(CommandError::ArgumentConflict(
                            arg.name.clone(),
                            conflict.clone(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate argument groups
    fn validate_groups(&self, matches: &ArgMatches) -> CommandResult<()> {
        for group in &self.groups {
            let present_count = group.args.iter().filter(|a| matches.is_present(a)).count();

            if group.required && present_count == 0 {
                return Err(CommandError::MissingArgument(format!(
                    "{} (one of: {})",
                    group.name,
                    group.args.join(", ")
                )));
            }

            // Groups are mutually exclusive by default
            if present_count > 1 {
                let present: Vec<&String> = group
                    .args
                    .iter()
                    .filter(|a| matches.is_present(a))
                    .collect();
                return Err(CommandError::ArgumentConflict(
                    present[0].clone(),
                    present[1].clone(),
                ));
            }
        }

        Ok(())
    }
}

/// Represents the main application
#[derive(Debug, Clone)]
pub struct App {
    command: Command,
}

impl App {
    /// Creates a new application with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            command: Command::new(name),
        }
    }

    /// Sets the version for this application
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.command = self.command.version(version);
        self
    }

    /// Sets the description for this application
    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.command = self.command.about(about);
        self
    }

    /// Adds an argument to this application
    pub fn arg(mut self, arg: Arg) -> Self {
        self.command = self.command.arg(arg);
        self
    }

    /// Adds a subcommand to this application
    pub fn subcommand(mut self, subcommand: Command) -> Self {
        self.command = self.command.subcommand(subcommand);
        self
    }

    /// Parses command-line arguments from `std::env::args()`
    pub fn get_matches(self) -> ArgMatches {
        self.get_matches_from(std::env::args())
    }

    /// Parses command-line arguments from an iterator
    pub fn get_matches_from<I, T>(self, args: I) -> ArgMatches
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let args: Vec<String> = args.into_iter().map(|a| a.into()).collect();
        let args_slice = if args.len() > 1 { &args[1..] } else { &[] };

        match self.command.parse_args(args_slice) {
            Ok(matches) => matches,
            Err(CommandError::HelpRequested) => {
                println!("{}", self.command.generate_help());
                std::process::exit(0);
            }
            Err(CommandError::VersionRequested) => {
                if let Some(version) = self.command.version {
                    println!("{} {}", self.command.name, version);
                } else {
                    println!("{}", self.command.name);
                }
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("\nFor more information try --help");
                std::process::exit(1);
            }
        }
    }

    /// Tries to parse arguments and returns a Result instead of exiting
    pub fn try_get_matches(self) -> CommandResult<ArgMatches> {
        self.try_get_matches_from(std::env::args())
    }

    /// Tries to parse arguments from an iterator
    pub fn try_get_matches_from<I, T>(self, args: I) -> CommandResult<ArgMatches>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let args: Vec<String> = args.into_iter().map(|a| a.into()).collect();
        let args_slice = if args.len() > 1 { &args[1..] } else { &[] };
        self.command.parse_args(args_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_creation() {
        let arg = Arg::new("test")
            .short('t')
            .long("test")
            .about("Test argument")
            .required(true);

        assert_eq!(arg.name(), "test");
        assert_eq!(arg.short, Some('t'));
        assert_eq!(arg.long, Some("test".to_string()));
    }

    #[test]
    fn test_simple_command() {
        let app = App::new("test").version("1.0.0").about("Test app").arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false),
        );

        let matches = app.try_get_matches_from(vec!["test", "--verbose"]).unwrap();

        assert!(matches.is_flag_set("verbose"));
    }

    #[test]
    fn test_subcommand_parsing() {
        let app = App::new("git").subcommand(
            Command::new("commit").arg(
                Arg::new("message")
                    .short('m')
                    .long("message")
                    .required(true),
            ),
        );

        let matches = app
            .try_get_matches_from(vec!["git", "commit", "-m", "Initial commit"])
            .unwrap();

        assert_eq!(matches.subcommand_name(), Some("commit"));
        let sub = matches.subcommand_matches("commit").unwrap();
        assert_eq!(sub.value_of("message"), Some("Initial commit"));
    }

    #[test]
    fn test_help_generation() {
        let cmd = Command::new("test")
            .about("Test command")
            .version("1.0.0")
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .about("Verbose output"),
            );

        let help = cmd.generate_help();
        assert!(help.contains("Test command"));
        assert!(help.contains("--verbose"));
        assert!(help.contains("Verbose output"));
    }
}
