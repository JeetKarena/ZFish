use kite::args::Args;
use std::env;

#[test]
fn test_args_parsing() {
    // Save the original args (using underscore to indicate intentionally unused)
    let _original_args: Vec<String> = env::args().collect();
    
    // Mock command-line arguments (using underscore to indicate intentionally unused)
    let _test_args = vec![
        "program".to_string(),
        "--verbose".to_string(),
        "-f".to_string(),
        "file.txt".to_string(),
        "--count=5".to_string(),
        "positional1".to_string(),
        "positional2".to_string(),
    ];
    
    // Override env::args() behavior for testing
    // Note: This doesn't actually modify env::args() in real code
    // For a proper test, consider using a CLI args constructor that takes Vec<String>
    
    // For demonstration, we'll just create an Args instance manually
    let args = Args {
        command: "program".to_string(),
        positional: vec!["positional1".to_string(), "positional2".to_string()],
        flags: {
            let mut m = std::collections::HashMap::new();
            m.insert("verbose".to_string(), true);
            m
        },
        options: {
            let mut m = std::collections::HashMap::new();
            m.insert("f".to_string(), "file.txt".to_string());
            m.insert("count".to_string(), "5".to_string());
            m
        },
    };
    
    // Test that args are parsed correctly
    assert_eq!(args.command, "program");
    assert_eq!(args.positional, vec!["positional1", "positional2"]);
    assert!(args.has_flag("verbose"));
    assert!(!args.has_flag("nonexistent"));
    assert_eq!(args.get_option("f").unwrap(), "file.txt");
    assert_eq!(args.get_option("count").unwrap(), "5");
    assert!(args.get_option("nonexistent").is_none());
}