/* //! Helper functions for testing the Kite library

use std::io::{self, Read, Write};
use std::env;

/// Temporarily set an environment variable for the duration of a function
pub fn with_env_var<F, T>(name: &str, value: Option<&str>, f: F) -> T
where
    F: FnOnce() -> T,
{
    let original = env::var(name).ok();
    match value {
        Some(val) => unsafe { env::set_var(name, val) },
        None => unsafe { env::remove_var(name) },
    };

    let result = f();

    match original {
        Some(val) => unsafe { env::set_var(name, val) },
        None => unsafe { env::remove_var(name) },
    }

    result
}

/// Capture stdout during a function's execution
pub fn capture_stdout<F>(f: F) -> io::Result<String>
where
    F: FnOnce() -> io::Result<()>
{
    // This is a simplified stub - in real code, you would need to
    // redirect stdout which is more complex and platform-specific
    // For now, we just return a placeholder

    f()?;
    Ok("Output captured (placeholder)".to_string())
}

use kite::style::{Color, Style};

#[test]
fn test_basic_coloring() {
    let expected_with_color = "\x1b[32mSuccess\x1b[0m";
    let expected_without_color = "Success";

    // Force color support off
    helpers::with_env_var("NO_COLOR", Some("1"), || {
        let green_text = Color::Green.paint("Success");
        assert_eq!(format!("{}", green_text), expected_without_color);
    });

    // Force color support on (remove NO_COLOR, set COLORTERM)
    helpers::with_env_var("NO_COLOR", None, || {
        helpers::with_env_var("COLORTERM", Some("1"), || {
            let green_text = Color::Green.paint("Success");
            assert_eq!(format!("{}", green_text), expected_with_color);
        });
    });
}

#[test]
fn test_styling_combinations() {
    let expected = "\x1b[31;1mError\x1b[0m";

    helpers::with_env_var("COLORTERM", Some("1"), || {
        helpers::with_env_var("NO_COLOR", None, || {
            let styled_text = Color::Red.paint("Error").style(Style::Bold);
            assert_eq!(format!("{}", styled_text), expected);
        });
    });
} */
