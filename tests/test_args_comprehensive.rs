//! Comprehensive tests for argument parsing
//! Tests edge cases, empty inputs, Unicode, special characters, etc.

use zfish::Args;

#[test]
fn test_empty_args() {
    // Test with no arguments (only program name)
    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags: std::collections::HashMap::new(),
        options: std::collections::HashMap::new(),
    };

    assert_eq!(args.positional.len(), 0);
    assert!(!args.has_flag("any"));
    assert!(args.get_option("any").is_none());
}

#[test]
fn test_single_short_flag() {
    let mut flags = std::collections::HashMap::new();
    flags.insert("v".to_string(), true);

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags,
        options: std::collections::HashMap::new(),
    };

    assert!(args.has_flag("v"));
    assert!(!args.has_flag("h"));
}

#[test]
fn test_combined_short_flags() {
    let mut flags = std::collections::HashMap::new();
    flags.insert("a".to_string(), true);
    flags.insert("b".to_string(), true);
    flags.insert("c".to_string(), true);

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags,
        options: std::collections::HashMap::new(),
    };

    assert!(args.has_flag("a"));
    assert!(args.has_flag("b"));
    assert!(args.has_flag("c"));
}

#[test]
fn test_long_flags() {
    let mut flags = std::collections::HashMap::new();
    flags.insert("verbose".to_string(), true);
    flags.insert("debug".to_string(), true);

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags,
        options: std::collections::HashMap::new(),
    };

    assert!(args.has_flag("verbose"));
    assert!(args.has_flag("debug"));
}

#[test]
fn test_option_with_equals() {
    let mut options = std::collections::HashMap::new();
    options.insert("file".to_string(), "test.txt".to_string());

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags: std::collections::HashMap::new(),
        options,
    };

    assert_eq!(args.get_option("file"), Some(&"test.txt".to_string()));
}

#[test]
fn test_option_with_space() {
    let mut options = std::collections::HashMap::new();
    options.insert("output".to_string(), "result.txt".to_string());

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags: std::collections::HashMap::new(),
        options,
    };

    assert_eq!(args.get_option("output"), Some(&"result.txt".to_string()));
}

#[test]
fn test_positional_args() {
    let args = Args {
        command: "test".to_string(),
        positional: vec!["file1.txt".to_string(), "file2.txt".to_string()],
        flags: std::collections::HashMap::new(),
        options: std::collections::HashMap::new(),
    };

    assert_eq!(args.positional.len(), 2);
    assert_eq!(args.positional[0], "file1.txt");
    assert_eq!(args.positional[1], "file2.txt");
}

#[test]
fn test_mixed_args() {
    let mut flags = std::collections::HashMap::new();
    flags.insert("v".to_string(), true);
    flags.insert("debug".to_string(), true);

    let mut options = std::collections::HashMap::new();
    options.insert("file".to_string(), "test.txt".to_string());

    let args = Args {
        command: "test".to_string(),
        positional: vec!["input.txt".to_string()],
        flags,
        options,
    };

    assert!(args.has_flag("v"));
    assert!(args.has_flag("debug"));
    assert_eq!(args.get_option("file"), Some(&"test.txt".to_string()));
    assert_eq!(args.positional.len(), 1);
}

#[test]
fn test_unicode_in_options() {
    let mut options = std::collections::HashMap::new();
    options.insert("name".to_string(), "Ñoño 中文 🎉".to_string());

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags: std::collections::HashMap::new(),
        options,
    };

    assert_eq!(args.get_option("name"), Some(&"Ñoño 中文 🎉".to_string()));
}

#[test]
fn test_special_characters_in_values() {
    let mut options = std::collections::HashMap::new();
    options.insert(
        "url".to_string(),
        "https://example.com?q=test&foo=bar".to_string(),
    );

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags: std::collections::HashMap::new(),
        options,
    };

    assert_eq!(
        args.get_option("url"),
        Some(&"https://example.com?q=test&foo=bar".to_string())
    );
}

#[test]
fn test_empty_option_value() {
    let mut options = std::collections::HashMap::new();
    options.insert("empty".to_string(), "".to_string());

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags: std::collections::HashMap::new(),
        options,
    };

    assert_eq!(args.get_option("empty"), Some(&"".to_string()));
}

#[test]
fn test_flag_names_with_special_chars() {
    let mut flags = std::collections::HashMap::new();
    flags.insert("test-flag".to_string(), true);
    flags.insert("another_flag".to_string(), true);

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags,
        options: std::collections::HashMap::new(),
    };

    assert!(args.has_flag("test-flag"));
    assert!(args.has_flag("another_flag"));
}

#[test]
fn test_has_flag_returns_false_for_missing() {
    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags: std::collections::HashMap::new(),
        options: std::collections::HashMap::new(),
    };

    assert!(!args.has_flag("missing"));
    assert!(!args.has_flag(""));
}

#[test]
fn test_get_option_returns_none_for_missing() {
    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags: std::collections::HashMap::new(),
        options: std::collections::HashMap::new(),
    };

    assert!(args.get_option("missing").is_none());
    assert!(args.get_option("").is_none());
}

#[test]
fn test_long_option_values() {
    let long_value = "a".repeat(10000);
    let mut options = std::collections::HashMap::new();
    options.insert("long".to_string(), long_value.clone());

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags: std::collections::HashMap::new(),
        options,
    };

    assert_eq!(args.get_option("long"), Some(&long_value));
}

#[test]
fn test_many_positional_args() {
    let positional: Vec<String> = (0..1000).map(|i| format!("arg{}", i)).collect();

    let args = Args {
        command: "test".to_string(),
        positional: positional.clone(),
        flags: std::collections::HashMap::new(),
        options: std::collections::HashMap::new(),
    };

    assert_eq!(args.positional.len(), 1000);
    assert_eq!(args.positional[0], "arg0");
    assert_eq!(args.positional[999], "arg999");
}

#[test]
fn test_many_flags() {
    let mut flags = std::collections::HashMap::new();
    for i in 0..100 {
        flags.insert(format!("flag{}", i), true);
    }

    let args = Args {
        command: "test".to_string(),
        positional: vec![],
        flags,
        options: std::collections::HashMap::new(),
    };

    assert!(args.has_flag("flag0"));
    assert!(args.has_flag("flag50"));
    assert!(args.has_flag("flag99"));
    assert!(!args.has_flag("flag100"));
}
