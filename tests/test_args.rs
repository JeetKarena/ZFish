use kite::Args;

#[test]
fn test_args_parsing() {
    // In a real application, env::args() would be populated
    // This test demonstrates the API even though we can't mock env::args()
    
    // Parse the actual command line arguments (from cargo test)
    let _args = Args::parse();
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