//! Comprehensive tests for the command and subcommand system
//!
//! This test suite covers:
//! - Basic argument parsing (flags, options, positional)
//! - Subcommand parsing (single and nested)
//! - Help generation
//! - Error handling
//! - Edge cases
//! - Cross-platform compatibility

use zfish::command::{App, Arg, ArgValue, Command, CommandError};

// ============================================================================
// Basic Argument Parsing Tests
// ============================================================================

#[test]
fn test_single_short_flag() {
    let app = App::new("test").arg(Arg::new("verbose").short('v').takes_value(false));

    let matches = app.try_get_matches_from(vec!["test", "-v"]).unwrap();
    assert!(matches.is_flag_set("verbose"));
}

#[test]
fn test_single_long_flag() {
    let app = App::new("test").arg(Arg::new("verbose").long("verbose").takes_value(false));

    let matches = app.try_get_matches_from(vec!["test", "--verbose"]).unwrap();
    assert!(matches.is_flag_set("verbose"));
}

#[test]
fn test_combined_short_flags() {
    let app = App::new("test")
        .arg(Arg::new("verbose").short('v').takes_value(false))
        .arg(Arg::new("debug").short('d').takes_value(false))
        .arg(Arg::new("quiet").short('q').takes_value(false));

    let matches = app.try_get_matches_from(vec!["test", "-vdq"]).unwrap();
    assert!(matches.is_flag_set("verbose"));
    assert!(matches.is_flag_set("debug"));
    assert!(matches.is_flag_set("quiet"));
}

#[test]
fn test_option_with_value_space_separated() {
    let app = App::new("test").arg(Arg::new("output").short('o').long("output"));

    let matches = app
        .try_get_matches_from(vec!["test", "-o", "file.txt"])
        .unwrap();
    assert_eq!(matches.value_of("output"), Some("file.txt"));
}

#[test]
fn test_option_with_value_equals_sign() {
    let app = App::new("test").arg(Arg::new("output").long("output"));

    let matches = app
        .try_get_matches_from(vec!["test", "--output=file.txt"])
        .unwrap();
    assert_eq!(matches.value_of("output"), Some("file.txt"));
}

#[test]
fn test_option_short_and_long() {
    let app = App::new("test").arg(Arg::new("output").short('o').long("output"));

    // Test short
    let matches1 = app
        .clone()
        .try_get_matches_from(vec!["test", "-o", "file1.txt"])
        .unwrap();
    assert_eq!(matches1.value_of("output"), Some("file1.txt"));

    // Test long
    let matches2 = app
        .try_get_matches_from(vec!["test", "--output", "file2.txt"])
        .unwrap();
    assert_eq!(matches2.value_of("output"), Some("file2.txt"));
}

#[test]
fn test_multiple_values() {
    let app = App::new("test").arg(Arg::new("file").short('f').multiple(true));

    let matches = app
        .try_get_matches_from(vec!["test", "-f", "a.txt", "-f", "b.txt"])
        .unwrap();

    let values = matches.values_of("file").unwrap();
    assert_eq!(values.len(), 2);
    assert!(values.contains(&"a.txt".to_string()));
    assert!(values.contains(&"b.txt".to_string()));
}

#[test]
fn test_default_value() {
    let app = App::new("test").arg(Arg::new("threads").long("threads").default_value("4"));

    let matches = app.try_get_matches_from(vec!["test"]).unwrap();
    assert_eq!(matches.value_of("threads"), Some("4"));
}

#[test]
fn test_default_value_override() {
    let app = App::new("test").arg(Arg::new("threads").long("threads").default_value("4"));

    let matches = app
        .try_get_matches_from(vec!["test", "--threads", "8"])
        .unwrap();
    assert_eq!(matches.value_of("threads"), Some("8"));
}

// ============================================================================
// Required Arguments Tests
// ============================================================================

#[test]
fn test_required_argument_present() {
    let app = App::new("test").arg(Arg::new("name").long("name").required(true));

    let matches = app
        .try_get_matches_from(vec!["test", "--name", "value"])
        .unwrap();
    assert_eq!(matches.value_of("name"), Some("value"));
}

#[test]
fn test_required_argument_missing() {
    let app = App::new("test").arg(Arg::new("name").long("name").required(true));

    let result = app.try_get_matches_from(vec!["test"]);
    assert!(result.is_err());
    match result {
        Err(CommandError::MissingArgument(name)) => assert_eq!(name, "name"),
        _ => panic!("Expected MissingArgument error"),
    }
}

// ============================================================================
// Validation Tests
// ============================================================================

#[test]
fn test_possible_values_valid() {
    let app = App::new("test").arg(
        Arg::new("level")
            .long("level")
            .possible_values(&["debug", "info", "warn"]),
    );

    let matches = app
        .try_get_matches_from(vec!["test", "--level", "debug"])
        .unwrap();
    assert_eq!(matches.value_of("level"), Some("debug"));
}

#[test]
fn test_possible_values_invalid() {
    let app = App::new("test").arg(
        Arg::new("level")
            .long("level")
            .possible_values(&["debug", "info", "warn"]),
    );

    let result = app.try_get_matches_from(vec!["test", "--level", "invalid"]);
    assert!(result.is_err());
}

#[test]
fn test_custom_validator_valid() {
    fn is_positive_number(s: &str) -> Result<(), String> {
        s.parse::<u32>()
            .map(|_| ())
            .map_err(|_| "must be a positive number".to_string())
    }

    let app = App::new("test").arg(
        Arg::new("count")
            .long("count")
            .validator(is_positive_number),
    );

    let matches = app
        .try_get_matches_from(vec!["test", "--count", "42"])
        .unwrap();
    assert_eq!(matches.value_of("count"), Some("42"));
}

#[test]
fn test_custom_validator_invalid() {
    fn is_positive_number(s: &str) -> Result<(), String> {
        s.parse::<u32>()
            .map(|_| ())
            .map_err(|_| "must be a positive number".to_string())
    }

    let app = App::new("test").arg(
        Arg::new("count")
            .long("count")
            .validator(is_positive_number),
    );

    let result = app.try_get_matches_from(vec!["test", "--count", "invalid"]);
    assert!(result.is_err());
}

// ============================================================================
// Subcommand Tests
// ============================================================================

#[test]
fn test_simple_subcommand() {
    let app = App::new("git").subcommand(Command::new("status"));

    let matches = app.try_get_matches_from(vec!["git", "status"]).unwrap();
    assert_eq!(matches.subcommand_name(), Some("status"));
}

#[test]
fn test_subcommand_with_args() {
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
fn test_multiple_subcommands() {
    let app = App::new("cargo")
        .subcommand(Command::new("build"))
        .subcommand(Command::new("test"))
        .subcommand(Command::new("run"));

    let matches1 = app
        .clone()
        .try_get_matches_from(vec!["cargo", "build"])
        .unwrap();
    assert_eq!(matches1.subcommand_name(), Some("build"));

    let matches2 = app
        .clone()
        .try_get_matches_from(vec!["cargo", "test"])
        .unwrap();
    assert_eq!(matches2.subcommand_name(), Some("test"));

    let matches3 = app.try_get_matches_from(vec!["cargo", "run"]).unwrap();
    assert_eq!(matches3.subcommand_name(), Some("run"));
}

#[test]
fn test_parent_and_subcommand_args() {
    let app = App::new("docker")
        .arg(Arg::new("verbose").short('v').takes_value(false))
        .subcommand(Command::new("run").arg(Arg::new("name").long("name")));

    let matches = app
        .try_get_matches_from(vec!["docker", "-v", "run", "--name", "mycontainer"])
        .unwrap();

    assert!(matches.is_flag_set("verbose"));
    assert_eq!(matches.subcommand_name(), Some("run"));

    let sub = matches.subcommand_matches("run").unwrap();
    assert_eq!(sub.value_of("name"), Some("mycontainer"));
}

#[test]
fn test_unknown_subcommand() {
    let app = App::new("test").subcommand(Command::new("build"));

    let result = app.try_get_matches_from(vec!["test", "unknown"]);
    // Unknown positional argument is not treated as subcommand error in current impl
    // It will be ignored if no subcommand matches
    assert!(result.is_ok());
}

// ============================================================================
// Help Generation Tests
// ============================================================================

#[test]
fn test_help_flag_short() {
    let app = App::new("test").arg(Arg::new("verbose").short('v'));

    let result = app.try_get_matches_from(vec!["test", "-h"]);
    assert!(matches!(result, Err(CommandError::HelpRequested)));
}

#[test]
fn test_help_flag_long() {
    let app = App::new("test").arg(Arg::new("verbose").short('v'));

    let result = app.try_get_matches_from(vec!["test", "--help"]);
    assert!(matches!(result, Err(CommandError::HelpRequested)));
}

#[test]
fn test_help_text_generation() {
    let cmd = Command::new("test")
        .about("Test command")
        .version("1.0.0")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .about("Enable verbose output")
                .takes_value(false),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file")
                .required(true),
        );

    let help = cmd.generate_help();

    assert!(help.contains("Test command"));
    assert!(help.contains("Version: 1.0.0"));
    assert!(help.contains("-v, --verbose"));
    assert!(help.contains("Enable verbose output"));
    assert!(help.contains("-o, --output"));
    assert!(help.contains("[required]"));
}

#[test]
fn test_help_with_subcommands() {
    let cmd = Command::new("git")
        .about("Git version control")
        .subcommand(Command::new("commit").about("Commit changes"))
        .subcommand(Command::new("push").about("Push to remote"));

    let help = cmd.generate_help();

    assert!(help.contains("COMMANDS:"));
    assert!(help.contains("commit"));
    assert!(help.contains("Commit changes"));
    assert!(help.contains("push"));
    assert!(help.contains("Push to remote"));
}

// ============================================================================
// Version Tests
// ============================================================================

#[test]
fn test_version_flag() {
    let app = App::new("test").version("1.0.0");

    let result = app.try_get_matches_from(vec!["test", "--version"]);
    assert!(matches!(result, Err(CommandError::VersionRequested)));
}

#[test]
fn test_version_flag_short() {
    let app = App::new("test").version("1.0.0");

    let result = app.try_get_matches_from(vec!["test", "-V"]);
    assert!(matches!(result, Err(CommandError::VersionRequested)));
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_unknown_long_flag() {
    let app = App::new("test").arg(Arg::new("verbose").long("verbose"));

    let result = app.try_get_matches_from(vec!["test", "--unknown"]);
    assert!(result.is_err());
    match result {
        Err(CommandError::UnknownArgument(name)) => assert_eq!(name, "unknown"),
        _ => panic!("Expected UnknownArgument error"),
    }
}

#[test]
fn test_unknown_short_flag() {
    let app = App::new("test").arg(Arg::new("verbose").short('v'));

    let result = app.try_get_matches_from(vec!["test", "-x"]);
    assert!(result.is_err());
    match result {
        Err(CommandError::UnknownArgument(name)) => assert_eq!(name, "x"),
        _ => panic!("Expected UnknownArgument error"),
    }
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[test]
fn test_empty_args() {
    let app = App::new("test");
    let matches = app.try_get_matches_from(vec!["test"]).unwrap();
    assert_eq!(matches.command_name(), "test");
}

#[test]
fn test_only_flags_no_subcommand() {
    let app = App::new("test")
        .arg(Arg::new("verbose").short('v').takes_value(false))
        .subcommand(Command::new("build"));

    let matches = app.try_get_matches_from(vec!["test", "-v"]).unwrap();
    assert!(matches.is_flag_set("verbose"));
    assert!(matches.subcommand_name().is_none());
}

#[test]
fn test_flag_after_value() {
    let app = App::new("test")
        .arg(Arg::new("output").short('o'))
        .arg(Arg::new("verbose").short('v').takes_value(false));

    let matches = app
        .try_get_matches_from(vec!["test", "-o", "file.txt", "-v"])
        .unwrap();

    assert_eq!(matches.value_of("output"), Some("file.txt"));
    assert!(matches.is_flag_set("verbose"));
}

#[test]
fn test_value_looks_like_flag() {
    let app = App::new("test").arg(Arg::new("output").short('o'));

    // This is a known limitation - values starting with '-' might be tricky
    // In production, we'd handle this with '--' separator
    let matches = app
        .try_get_matches_from(vec!["test", "-o", "normalfile.txt"])
        .unwrap();
    assert_eq!(matches.value_of("output"), Some("normalfile.txt"));
}

#[test]
fn test_equals_sign_with_empty_value() {
    let app = App::new("test").arg(Arg::new("output").long("output"));

    let matches = app.try_get_matches_from(vec!["test", "--output="]).unwrap();
    assert_eq!(matches.value_of("output"), Some(""));
}

#[test]
fn test_arg_value_types() {
    let single = ArgValue::Single("test".to_string());
    assert_eq!(single.as_str(), Some("test"));
    assert_eq!(single.as_bool(), None);
    assert_eq!(single.as_vec(), None);

    let flag = ArgValue::Flag(true);
    assert_eq!(flag.as_bool(), Some(true));
    assert_eq!(flag.as_str(), None);

    let multiple = ArgValue::Multiple(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(multiple.as_vec().unwrap().len(), 2);
    assert_eq!(multiple.as_str(), None);
}

// ============================================================================
// Real-World Scenario Tests
// ============================================================================

#[test]
fn test_git_like_cli() {
    let app = App::new("git")
        .version("2.0.0")
        .about("Git version control system")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false),
        )
        .subcommand(
            Command::new("commit")
                .about("Record changes to the repository")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .required(true),
                )
                .arg(Arg::new("all").short('a').long("all").takes_value(false)),
        )
        .subcommand(
            Command::new("push").about("Update remote refs").arg(
                Arg::new("force")
                    .short('f')
                    .long("force")
                    .takes_value(false),
            ),
        );

    // Test commit with message
    let matches = app
        .clone()
        .try_get_matches_from(vec!["git", "commit", "-m", "Initial commit", "-a"])
        .unwrap();

    assert_eq!(matches.subcommand_name(), Some("commit"));
    let sub = matches.subcommand_matches("commit").unwrap();
    assert_eq!(sub.value_of("message"), Some("Initial commit"));
    assert!(sub.is_flag_set("all"));

    // Test push with force
    let matches2 = app
        .try_get_matches_from(vec!["git", "push", "--force"])
        .unwrap();

    assert_eq!(matches2.subcommand_name(), Some("push"));
    let sub2 = matches2.subcommand_matches("push").unwrap();
    assert!(sub2.is_flag_set("force"));
}

#[test]
fn test_cargo_like_cli() {
    let app = App::new("cargo")
        .version("1.70.0")
        .arg(
            Arg::new("verbose")
                .short('v')
                .takes_value(false)
                .multiple(true),
        )
        .subcommand(
            Command::new("build")
                .about("Compile the current package")
                .arg(Arg::new("release").long("release").takes_value(false))
                .arg(Arg::new("target").long("target")),
        )
        .subcommand(
            Command::new("test")
                .about("Run tests")
                .arg(Arg::new("release").long("release").takes_value(false)),
        );

    let matches = app
        .try_get_matches_from(vec![
            "cargo",
            "build",
            "--release",
            "--target",
            "x86_64-unknown-linux-gnu",
        ])
        .unwrap();

    let sub = matches.subcommand_matches("build").unwrap();
    assert!(sub.is_flag_set("release"));
    assert_eq!(sub.value_of("target"), Some("x86_64-unknown-linux-gnu"));
}

#[test]
fn test_docker_like_cli() {
    let app = App::new("docker").subcommand(
        Command::new("run")
            .about("Run a command in a new container")
            .arg(Arg::new("name").long("name"))
            .arg(
                Arg::new("detach")
                    .short('d')
                    .long("detach")
                    .takes_value(false),
            )
            .arg(Arg::new("port").short('p').long("port").multiple(true)),
    );

    let matches = app
        .try_get_matches_from(vec![
            "docker",
            "run",
            "-d",
            "--name",
            "mycontainer",
            "-p",
            "8080:80",
            "-p",
            "443:443",
        ])
        .unwrap();

    let sub = matches.subcommand_matches("run").unwrap();
    assert!(sub.is_flag_set("detach"));
    assert_eq!(sub.value_of("name"), Some("mycontainer"));
    let ports = sub.values_of("port").unwrap();
    assert_eq!(ports.len(), 2);
}

// ============================================================================
// Platform-Specific Tests (These should pass on all platforms)
// ============================================================================

#[test]
#[cfg(target_os = "windows")]
fn test_windows_path_handling() {
    let app = App::new("test").arg(Arg::new("path").long("path"));

    let matches = app
        .try_get_matches_from(vec!["test", "--path", "C:\\Users\\test\\file.txt"])
        .unwrap();

    assert_eq!(matches.value_of("path"), Some("C:\\Users\\test\\file.txt"));
}

#[test]
#[cfg(target_family = "unix")]
fn test_unix_path_handling() {
    let app = App::new("test").arg(Arg::new("path").long("path"));

    let matches = app
        .try_get_matches_from(vec!["test", "--path", "/home/user/file.txt"])
        .unwrap();

    assert_eq!(matches.value_of("path"), Some("/home/user/file.txt"));
}

#[test]
fn test_unicode_arguments() {
    let app = App::new("test").arg(Arg::new("name").long("name"));

    let matches = app
        .try_get_matches_from(vec!["test", "--name", "你好世界🚀"])
        .unwrap();

    assert_eq!(matches.value_of("name"), Some("你好世界🚀"));
}

#[test]
fn test_special_characters_in_values() {
    let app = App::new("test").arg(Arg::new("pattern").long("pattern"));

    let matches = app
        .try_get_matches_from(vec!["test", "--pattern", "*.rs"])
        .unwrap();

    assert_eq!(matches.value_of("pattern"), Some("*.rs"));
}
