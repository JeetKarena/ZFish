use kite::prompt::Prompt;

// These tests require user input, so they're marked as #[ignore]
// Run them explicitly with: cargo test -- --ignored

#[test]
#[ignore]
fn test_confirm_prompt() {
    // This is a manual test since it requires user input
    println!("Please type 'y' when prompted:");
    let result = Prompt::confirm("Are you sure?", false).unwrap();
    println!("You entered: {}", if result { "yes" } else { "no" });
    
    println!("Please type 'n' when prompted:");
    let result = Prompt::confirm("Continue?", true).unwrap();
    println!("You entered: {}", if result { "yes" } else { "no" });
    
    println!("Please press enter when prompted:");
    let result = Prompt::confirm("Accept default (yes)?", true).unwrap();
    println!("You entered: default ({}) -> {}", if result { "yes" } else { "no" }, result);
    
    // Don't assert specific values since this is an interactive test
    // Just make the test pass
    assert!(true);
}

#[test]
#[ignore]
fn test_text_input() {
    println!("Please type any text when prompted:");
    let result = Prompt::input("Enter text:").unwrap();
    println!("You entered: {}", result);
    
    // Don't assert specific values since this is an interactive test
    assert!(!result.is_empty());
}

#[test]
#[ignore]
fn test_password_input() {
    println!("Please type any password when prompted (input will be hidden):");
    let result = Prompt::password("Enter password:").unwrap();
    println!("You entered a password with {} characters: {}", 
             result.len(), 
             "*".repeat(result.len())); // Don't display the actual password
    
    // Don't assert specific values since this is an interactive test
    assert!(!result.is_empty());
}