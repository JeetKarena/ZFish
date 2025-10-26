use kite::style::{Color, Style};

// Helper functions for testing the Kite library

/// Temporarily set an environment variable for the duration of a function
pub fn with_env_var<F, T>(name: &str, value: Option<&str>, f: F) -> T
where
    F: FnOnce() -> T,
{
    let original = std::env::var(name).ok();
    unsafe {
        if let Some(v) = value {
            std::env::set_var(name, v);
        } else {
            std::env::remove_var(name);
        }
    }

    let result = f();

    unsafe {
        match original {
            Some(val) => std::env::set_var(name, val),
            None => std::env::remove_var(name),
        }
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
    let expected_with_color = "\x1b[32mSuccess\x1b[0m";
    let expected_without_color = "Success";

    // Force color support off
    with_env_var("NO_COLOR", Some("1"), || {
        with_env_var("COLORTERM", None, || {
            let green_text = Color::Green.paint("Success");
            assert_eq!(format!("{}", green_text), expected_without_color);
        });
    });

    // Force color support on (remove NO_COLOR, set COLORTERM)
    with_env_var("NO_COLOR", None, || {
        with_env_var("COLORTERM", Some("1"), || {
            let green_text = Color::Green.paint("Success");
            assert_eq!(format!("{}", green_text), expected_with_color);
        });
    });
}

#[test]
fn test_styling_combinations() {
    // Test combining colors and styles
    with_env_var("NO_COLOR", None, || {
        with_env_var("COLORTERM", Some("1"), || {
            let styled_text = Color::Red.paint("Error").style(Style::Bold);
            let expected = "\x1b[31;1mError\x1b[0m";

            assert_eq!(format!("{}", styled_text), expected);
        });
    });
}

#[test]
#[ignore]
fn test_all_colors_display() {
    // This is an interactive test to visually verify color display on the terminal.
    // Run with: cargo test -- --ignored --test style_test
    // Ensure your terminal (e.g., Windows PowerShell) supports ANSI colors.

    with_env_var("COLORTERM", Some("1"), || {
        with_env_var("NO_COLOR", None, || {
            println!("Testing all colors on the terminal:");
            println!("If colors are not displayed correctly, check your terminal settings.");
            println!();

            let colors = [
                Color::Black,
                Color::Red,
                Color::Green,
                Color::Yellow,
                Color::Blue,
                Color::Magenta,
                Color::Cyan,
                Color::White,
                Color::BrightBlack,
                Color::BrightRed,
                Color::BrightGreen,
                Color::BrightYellow,
                Color::BrightBlue,
                Color::BrightMagenta,
                Color::BrightCyan,
                Color::BrightWhite,
            ];

            for color in &colors {
                println!("{}", color.paint(format!("This is {:?} color", color)));
            }

            println!();
            println!(
                "Colors should appear as expected. If not, your terminal may not support ANSI colors."
            );

            // Test runs without panicking - success
           
        });
    });
}

#[test]
fn test_custom_256_coloring() {
    // Test custom 256 colors
    with_env_var("NO_COLOR", None, || {
        with_env_var("COLORTERM", Some("1"), || {
            // Test a custom color (e.g., color 196 is a bright red)
            let custom_text = Color::Custom(196).paint("Custom Red");
            let expected = "\x1b[38;5;196mCustom Red\x1b[0m";
            assert_eq!(format!("{}", custom_text), expected);

            // Test combining custom color with style
            let styled_custom = Color::Custom(46).paint("Custom Green").style(Style::Bold);
            let expected_styled = "\x1b[38;5;46;1mCustom Green\x1b[0m";
            assert_eq!(format!("{}", styled_custom), expected_styled);
        });
    });

    // Test that custom colors are disabled with NO_COLOR
    with_env_var("NO_COLOR", Some("1"), || {
        let custom_text = Color::Custom(100).paint("No Color");
        assert_eq!(format!("{}", custom_text), "No Color");
    });
}

#[test]
#[ignore]
fn test_256_colors_display() {
    // Interactive test to visually verify all 256 colors on the terminal.
    // Run with: cargo test -- --ignored --test style_test
    // Ensure your terminal supports ANSI 256 colors.

    with_env_var("COLORTERM", Some("1"), || {
        with_env_var("NO_COLOR", None, || {
            println!("Testing all 256 colors on the terminal:");
            println!("If colors are not displayed correctly, check your terminal settings.");
            println!();

            for i in 0..=255 {
                println!("{}", Color::Custom(i).paint(format!("Color {}", i)));
            }

            println!();
            println!("All 256 colors should appear. If not, your terminal may not support ANSI 256 colors.");



            // Test runs without panicking - success
        });
    });
}

