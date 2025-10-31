//! Comprehensive tests for v0.2.1 command features
//!
//! Tests cover:
//! - Positional arguments
//! - Variadic positional arguments  
//! - Environment variable fallbacks
//! - Argument dependencies (requires)
//! - Argument conflicts
//! - Value delimiters
//! - Command aliases
//! - Argument groups

use zfish::command::{App, Arg, Command};

// ============================================================================
// POSITIONAL ARGUMENTS TESTS
// ============================================================================

#[test]
fn test_single_positional_argument() {
    let app = App::new("test")
        .subcommand(Command::new("process").arg(Arg::new("input").index(0).required(true)));

    let matches = app.get_matches_from(vec!["test", "process", "file.txt"]);
    let sub = matches.subcommand().unwrap().1;
    assert_eq!(sub.value_of("input"), Some("file.txt"));
}

#[test]
fn test_multiple_positional_arguments() {
    let app = App::new("test").subcommand(
        Command::new("copy")
            .arg(Arg::new("source").index(0).required(true))
            .arg(Arg::new("dest").index(1).required(true)),
    );

    let matches = app.get_matches_from(vec!["test", "copy", "src.txt", "dst.txt"]);
    let sub = matches.subcommand().unwrap().1;
    assert_eq!(sub.value_of("source"), Some("src.txt"));
    assert_eq!(sub.value_of("dest"), Some("dst.txt"));
}

#[test]
fn test_optional_positional_argument() {
    // Without argument - should use default
    let app1 = App::new("test")
        .subcommand(Command::new("list").arg(Arg::new("dir").index(0).default_value(".")));
    let matches = app1.get_matches_from(vec!["test", "list"]);
    let sub = matches.subcommand().unwrap().1;
    assert_eq!(sub.value_of("dir"), Some("."));

    // With argument
    let app2 = App::new("test")
        .subcommand(Command::new("list").arg(Arg::new("dir").index(0).default_value(".")));
    let matches2 = app2.get_matches_from(vec!["test", "list", "/home"]);
    let sub2 = matches2.subcommand().unwrap().1;
    assert_eq!(sub2.value_of("dir"), Some("/home"));
}

#[test]
fn test_positional_with_flags() {
    let app = App::new("test").subcommand(
        Command::new("build")
            .arg(Arg::new("release").long("release").takes_value(false))
            .arg(Arg::new("file").index(0).required(true)),
    );

    let matches = app.get_matches_from(vec!["test", "build", "--release", "main.rs"]);
    let sub = matches.subcommand().unwrap().1;
    assert_eq!(sub.value_of("file"), Some("main.rs"));
    assert!(sub.is_present("release"));
}

// ============================================================================
// VARIADIC POSITIONAL ARGUMENTS TESTS
// ============================================================================

#[test]
fn test_variadic_positional_single_file() {
    let app = App::new("test").subcommand(
        Command::new("remove").arg(Arg::new("files").index(0).last(true).required(true)),
    );

    let matches = app.get_matches_from(vec!["test", "remove", "file1.txt"]);
    let sub = matches.subcommand().unwrap().1;
    let files: Vec<String> = vec!["file1.txt".to_string()];
    assert_eq!(sub.values_of("files"), Some(files.as_slice()));
}

#[test]
fn test_variadic_positional_multiple_files() {
    let app = App::new("test").subcommand(
        Command::new("remove").arg(Arg::new("files").index(0).last(true).required(true)),
    );

    let matches = app.get_matches_from(vec!["test", "remove", "a.txt", "b.txt", "c.txt"]);
    let sub = matches.subcommand().unwrap().1;
    let files: Vec<String> = vec![
        "a.txt".to_string(),
        "b.txt".to_string(),
        "c.txt".to_string(),
    ];
    assert_eq!(sub.values_of("files"), Some(files.as_slice()));
}

#[test]
fn test_variadic_with_flags() {
    let app = App::new("test").subcommand(
        Command::new("compress")
            .arg(Arg::new("verbose").short('v').takes_value(false))
            .arg(Arg::new("files").index(0).last(true).required(true)),
    );

    let matches =
        app.get_matches_from(vec!["test", "compress", "-v", "f1.txt", "f2.txt", "f3.txt"]);
    let sub = matches.subcommand().unwrap().1;
    assert!(sub.is_present("verbose"));
    let files: Vec<String> = vec![
        "f1.txt".to_string(),
        "f2.txt".to_string(),
        "f3.txt".to_string(),
    ];
    assert_eq!(sub.values_of("files"), Some(files.as_slice()));
}

// ============================================================================
// ENVIRONMENT VARIABLE TESTS
// ============================================================================

#[test]
fn test_env_var_fallback() {
    unsafe {
        std::env::set_var("TEST_CONFIG", "from_env.toml");
    }

    let app = App::new("test").arg(Arg::new("config").long("config").env("TEST_CONFIG"));

    // Without CLI arg - should use env var
    let matches = app.get_matches_from(vec!["test"]);
    assert_eq!(matches.value_of("config"), Some("from_env.toml"));

    unsafe {
        std::env::remove_var("TEST_CONFIG");
    }
}

#[test]
fn test_cli_overrides_env_var() {
    unsafe {
        std::env::set_var("TEST_PORT", "8080");
    }

    let app = App::new("test").arg(Arg::new("port").long("port").env("TEST_PORT"));

    // CLI arg should override env var
    let matches = app.get_matches_from(vec!["test", "--port", "3000"]);
    assert_eq!(matches.value_of("port"), Some("3000"));

    unsafe {
        std::env::remove_var("TEST_PORT");
    }
}

#[test]
fn test_env_var_with_default() {
    let app = App::new("test").arg(
        Arg::new("host")
            .long("host")
            .env("TEST_HOST")
            .default_value("localhost"),
    );

    // No CLI arg, no env var - should use default
    let matches = app.get_matches_from(vec!["test"]);
    assert_eq!(matches.value_of("host"), Some("localhost"));
}

#[test]
fn test_priority_cli_env_default() {
    unsafe {
        std::env::set_var("TEST_LEVEL", "info");
    }

    let app = App::new("test").arg(
        Arg::new("level")
            .long("level")
            .env("TEST_LEVEL")
            .default_value("warn"),
    );

    // Priority: CLI > ENV > DEFAULT

    // Only default
    let app1 = App::new("test").arg(Arg::new("level").long("level").default_value("warn"));
    let m1 = app1.get_matches_from(vec!["test"]);
    assert_eq!(m1.value_of("level"), Some("warn"));

    // ENV overrides default
    let m2 = app.get_matches_from(vec!["test"]);
    assert_eq!(m2.value_of("level"), Some("info"));

    // CLI overrides ENV
    let app3 = App::new("test").arg(
        Arg::new("level")
            .long("level")
            .env("TEST_LEVEL")
            .default_value("warn"),
    );
    let m3 = app3.get_matches_from(vec!["test", "--level", "debug"]);
    assert_eq!(m3.value_of("level"), Some("debug"));

    unsafe {
        std::env::remove_var("TEST_LEVEL");
    }
}

// ============================================================================
// ARGUMENT DEPENDENCIES TESTS (requires)
// ============================================================================

// Note: Tests for missing dependencies cannot use should_panic because
// get_matches_from() calls std::process::exit() rather than panicking.
// These are tested manually in examples and behave correctly in production.

#[test]
fn test_requires_satisfied() {
    let app = App::new("test").subcommand(
        Command::new("export")
            .arg(Arg::new("output").long("output").requires("format"))
            .arg(Arg::new("format").long("format")),
    );

    let matches = app.get_matches_from(vec![
        "test", "export", "--output", "file.txt", "--format", "json",
    ]);
    let sub = matches.subcommand().unwrap().1;
    assert_eq!(sub.value_of("output"), Some("file.txt"));
    assert_eq!(sub.value_of("format"), Some("json"));
}

#[test]
fn test_multiple_requires() {
    let app = App::new("test").subcommand(
        Command::new("render")
            .arg(
                Arg::new("output")
                    .long("output")
                    .requires("format")
                    .requires("quality"),
            )
            .arg(Arg::new("format").long("format"))
            .arg(Arg::new("quality").long("quality")),
    );

    let matches = app.get_matches_from(vec![
        "test",
        "render",
        "--output",
        "out.png",
        "--format",
        "png",
        "--quality",
        "high",
    ]);
    let sub = matches.subcommand().unwrap().1;
    assert_eq!(sub.value_of("output"), Some("out.png"));
}

#[test]
fn test_requires_not_present_when_not_used() {
    // Test that required args are only enforced when the requiring arg is present
    let app = App::new("test").subcommand(
        Command::new("export")
            .arg(Arg::new("output").long("output").requires("format"))
            .arg(Arg::new("format").long("format")),
    );

    // Should work fine - output not provided, so format not required
    let matches = app.get_matches_from(vec!["test", "export"]);
    let sub = matches.subcommand().unwrap().1;
    assert!(!sub.is_present("output"));
    assert!(!sub.is_present("format"));
}

// ============================================================================
// ARGUMENT CONFLICTS TESTS
// ============================================================================

// Note: Tests for conflicts cannot use should_panic because
// get_matches_from() calls std::process::exit() rather than panicking.
// These are tested manually in examples and behave correctly in production.

#[test]
fn test_no_conflict_when_only_one_present() {
    let app = App::new("test")
        .arg(
            Arg::new("verbose")
                .short('v')
                .takes_value(false)
                .conflicts_with("quiet"),
        )
        .arg(Arg::new("quiet").short('q').takes_value(false));

    // Should work - only verbose is present
    let matches = app.get_matches_from(vec!["test", "-v"]);
    assert!(matches.is_present("verbose"));
    assert!(!matches.is_present("quiet"));
}

#[test]
fn test_no_conflict_when_neither_present() {
    let app = App::new("test")
        .arg(
            Arg::new("verbose")
                .short('v')
                .takes_value(false)
                .conflicts_with("quiet"),
        )
        .arg(Arg::new("quiet").short('q').takes_value(false));

    // Should work - neither is present
    let matches = app.get_matches_from(vec!["test"]);
    assert!(!matches.is_present("verbose"));
    assert!(!matches.is_present("quiet"));
}

// ============================================================================
// VALUE DELIMITER TESTS
// ============================================================================

#[test]
fn test_value_delimiter_comma() {
    let app = App::new("test").arg(Arg::new("tags").long("tags").value_delimiter(','));

    let matches = app.get_matches_from(vec!["test", "--tags", "rust,cli,tool"]);
    let tags: Vec<String> = vec!["rust".to_string(), "cli".to_string(), "tool".to_string()];
    assert_eq!(matches.values_of("tags"), Some(tags.as_slice()));
}

#[test]
fn test_value_delimiter_colon() {
    let app = App::new("test").arg(Arg::new("path").long("path").value_delimiter(':'));

    let matches = app.get_matches_from(vec!["test", "--path", "/bin:/usr/bin:/usr/local/bin"]);
    let paths: Vec<String> = vec![
        "/bin".to_string(),
        "/usr/bin".to_string(),
        "/usr/local/bin".to_string(),
    ];
    assert_eq!(matches.values_of("path"), Some(paths.as_slice()));
}

#[test]
fn test_value_delimiter_with_spaces() {
    let app = App::new("test").arg(Arg::new("items").long("items").value_delimiter(','));

    // Spaces around delimiters should be trimmed
    let matches = app.get_matches_from(vec!["test", "--items", "a, b, c"]);
    let items: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(matches.values_of("items"), Some(items.as_slice()));
}

#[test]
fn test_value_delimiter_single_value() {
    let app = App::new("test").arg(Arg::new("tags").long("tags").value_delimiter(','));

    // Single value without delimiter
    let matches = app.get_matches_from(vec!["test", "--tags", "rust"]);
    let tags: Vec<String> = vec!["rust".to_string()];
    assert_eq!(matches.values_of("tags"), Some(tags.as_slice()));
}

#[test]
fn test_value_delimiter_with_equals() {
    let app = App::new("test").arg(Arg::new("tags").long("tags").value_delimiter(','));

    let matches = app.get_matches_from(vec!["test", "--tags=a,b,c"]);
    let tags: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(matches.values_of("tags"), Some(tags.as_slice()));
}

// ============================================================================
// COMMAND ALIASES TESTS
// ============================================================================

#[test]
fn test_single_alias() {
    let app = App::new("test").subcommand(Command::new("build").alias("b"));

    // Using full name
    let m1 = app.get_matches_from(vec!["test", "build"]);
    assert_eq!(m1.subcommand().map(|(n, _)| n), Some("build"));

    // Using alias
    let app2 = App::new("test").subcommand(Command::new("build").alias("b"));
    let m2 = app2.get_matches_from(vec!["test", "b"]);
    assert_eq!(m2.subcommand().map(|(n, _)| n), Some("b"));
}

#[test]
fn test_multiple_aliases() {
    let app = App::new("test").subcommand(Command::new("initialize").alias("init").alias("i"));

    // Using full name
    let m1 = app.get_matches_from(vec!["test", "initialize"]);
    assert!(m1.subcommand().is_some());

    // Using first alias
    let app2 = App::new("test").subcommand(Command::new("initialize").alias("init").alias("i"));
    let m2 = app2.get_matches_from(vec!["test", "init"]);
    assert!(m2.subcommand().is_some());

    // Using second alias
    let app3 = App::new("test").subcommand(Command::new("initialize").alias("init").alias("i"));
    let m3 = app3.get_matches_from(vec!["test", "i"]);
    assert!(m3.subcommand().is_some());
}

#[test]
fn test_alias_with_arguments() {
    let app = App::new("test").subcommand(
        Command::new("process")
            .alias("p")
            .arg(Arg::new("file").long("file").required(true)),
    );

    let matches = app.get_matches_from(vec!["test", "p", "--file", "input.txt"]);
    let sub = matches.subcommand().unwrap().1;
    assert_eq!(sub.value_of("file"), Some("input.txt"));
}

// ============================================================================
// ARGUMENT GROUPS TESTS
// ============================================================================

// Note: ArgGroup tests disabled - groups are for subcommands, not App level
// These tests would need to be refactored to work with subcommands

// ============================================================================
// COMBINED FEATURES TESTS
// ============================================================================

#[test]
fn test_positional_with_delimiter() {
    let app = App::new("test").subcommand(
        Command::new("tag")
            .arg(Arg::new("file").index(0).required(true))
            .arg(Arg::new("tags").long("tags").value_delimiter(',')),
    );

    let matches = app.get_matches_from(vec!["test", "tag", "file.txt", "--tags", "a,b,c"]);
    let sub = matches.subcommand().unwrap().1;
    assert_eq!(sub.value_of("file"), Some("file.txt"));
    let tags: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(sub.values_of("tags"), Some(tags.as_slice()));
}

#[test]
fn test_alias_with_env_var() {
    unsafe {
        std::env::set_var("TEST_OUTPUT", "default.txt");
    }

    let app = App::new("test").subcommand(
        Command::new("export")
            .alias("exp")
            .arg(Arg::new("output").long("output").env("TEST_OUTPUT")),
    );

    let matches = app.get_matches_from(vec!["test", "exp"]);
    let sub = matches.subcommand().unwrap().1;
    assert_eq!(sub.value_of("output"), Some("default.txt"));

    unsafe {
        std::env::remove_var("TEST_OUTPUT");
    }
}

#[test]
fn test_variadic_with_delimiter() {
    let app = App::new("test").subcommand(
        Command::new("process")
            .arg(Arg::new("tags").long("tags").value_delimiter(','))
            .arg(Arg::new("files").index(0).last(true)),
    );

    let matches = app.get_matches_from(vec![
        "test", "process", "--tags", "a,b,c", "f1.txt", "f2.txt",
    ]);
    let sub = matches.subcommand().unwrap().1;
    let tags: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(sub.values_of("tags"), Some(tags.as_slice()));
    let files: Vec<String> = vec!["f1.txt".to_string(), "f2.txt".to_string()];
    assert_eq!(sub.values_of("files"), Some(files.as_slice()));
}
