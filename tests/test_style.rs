use kite::style::{Color, Style};

// Helper functions for testing the Kite library

use std::env;

/// Temporarily set an environment variable for the duration of a function
pub fn with_env_var<F, T>(name: &str, value: Option<&str>, f: F) -> T
where
    F: FnOnce() -> T,
{
    let original = env::var(name).ok();
    if let Some(v) = value {
        // environment API in this workspace requires explicit call context
        unsafe { env::set_var(name, v); }
    } else {
        unsafe { env::remove_var(name); }
    }

    let result = f();

    match original {
        Some(val) => unsafe { env::set_var(name, &val) },
        None => unsafe { env::remove_var(name) },
    }

    result
}

/// Capture stdout during a function's execution (stubbed)
pub fn capture_stdout<F>(f: F) -> std::io::Result<String>
where
    F: FnOnce() -> std::io::Result<()>,
{
    f()?;
    Ok("Output captured (placeholder)".to_string())
}

#[test]
fn test_basic_coloring() {
    // Test that coloring produces expected ANSI sequences
    let green_text = Color::Green.paint("Success");
    let expected_with_color = "\x1b[32mSuccess\x1b[0m";
    let expected_without_color = "Success";
    
    // The display output depends on whether colors are supported
    // For testing, we'll ensure this works both ways
    
    // Force color support off
    unsafe { env::set_var("NO_COLOR", "1") };
    assert_eq!(format!("{}", green_text), expected_without_color);
    
    // Force color support on
    unsafe { env::remove_var("NO_COLOR") };
    unsafe { env::set_var("COLORTERM", "1") };
    
    // Now we need to create a new styled string since the previous one 
    // cached the color support detection result
    let green_text = Color::Green.paint("Success");
    assert_eq!(format!("{}", green_text), expected_with_color);
    
    // Clean up environment
    unsafe { env::remove_var("COLORTERM") };
}

#[test]
fn test_styling_combinations() {
    // Test combining colors and styles
    unsafe { env::set_var("COLORTERM", "1") };
    unsafe { env::remove_var("NO_COLOR") };
    
    let styled_text = Color::Red.paint("Error").style(Style::Bold);
    let expected = "\x1b[31;1mError\x1b[0m";
    
    assert_eq!(format!("{}", styled_text), expected);
    
    // Clean up environment
    unsafe { env::remove_var("COLORTERM") };
}