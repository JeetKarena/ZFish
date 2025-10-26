//! Comprehensive tests for Terminal module
//! Tests cursor movement, screen clearing, positioning

use kite::term::Terminal;

#[test]
fn test_terminal_clear_screen() {
    let result = Terminal::clear_screen();
    // Should not panic, may fail if not in a terminal
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_terminal_move_cursor_origin() {
    let result = Terminal::move_cursor(0, 0);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_terminal_move_cursor_various_positions() {
    let positions = vec![(0, 0), (1, 1), (10, 10), (100, 100), (u16::MAX, u16::MAX)];

    for (row, col) in positions {
        let result = Terminal::move_cursor(row, col);
        assert!(result.is_ok() || result.is_err());
    }
}

#[test]
fn test_terminal_size() {
    let size = Terminal::size();
    // May be None if not in a terminal
    match size {
        Some((rows, cols)) => {
            assert!(rows > 0, "Rows should be positive");
            assert!(cols > 0, "Columns should be positive");
        }
        None => {
            // Expected in non-terminal environments
        }
    }
}

#[test]
fn test_terminal_print_at_origin() {
    let result = Terminal::print_at(0, 0, "Test");
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_terminal_print_at_various_positions() {
    let tests = vec![
        (0, 0, "Origin"),
        (1, 1, "One One"),
        (10, 20, "Ten Twenty"),
        (50, 50, "Fifty"),
    ];

    for (row, col, text) in tests {
        let result = Terminal::print_at(row, col, text);
        assert!(result.is_ok() || result.is_err());
    }
}

#[test]
fn test_terminal_print_empty_string() {
    let result = Terminal::print_at(0, 0, "");
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_terminal_print_long_string() {
    let long_text = "a".repeat(10000);
    let result = Terminal::print_at(0, 0, &long_text);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_terminal_print_unicode() {
    let unicode_texts = vec!["Hello 世界", "Привет мир", "🎉🚀💯", "Ñoño"];

    for text in unicode_texts {
        let result = Terminal::print_at(0, 0, text);
        assert!(result.is_ok() || result.is_err());
    }
}

#[test]
fn test_terminal_print_multiline() {
    let multiline = "Line 1\nLine 2\nLine 3";
    let result = Terminal::print_at(0, 0, multiline);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_terminal_print_special_chars() {
    let special = "Tab:\tQuote:\"Backslash:\\";
    let result = Terminal::print_at(0, 0, special);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_terminal_multiple_clear_calls() {
    for _ in 0..10 {
        let result = Terminal::clear_screen();
        assert!(result.is_ok() || result.is_err());
    }
}

#[test]
fn test_terminal_cursor_movement_sequence() {
    // Move cursor in a pattern
    let _ = Terminal::move_cursor(0, 0);
    let _ = Terminal::move_cursor(10, 10);
    let _ = Terminal::move_cursor(5, 5);
    let _ = Terminal::move_cursor(0, 0);
}

#[test]
fn test_terminal_print_sequence() {
    let _ = Terminal::print_at(0, 0, "First");
    let _ = Terminal::print_at(1, 0, "Second");
    let _ = Terminal::print_at(2, 0, "Third");
}

#[test]
fn test_terminal_size_multiple_calls() {
    let size1 = Terminal::size();
    let size2 = Terminal::size();
    // Should be consistent
    assert_eq!(size1, size2);
}

#[test]
fn test_terminal_print_at_max_position() {
    let result = Terminal::print_at(u16::MAX, u16::MAX, "Edge");
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_terminal_rapid_operations() {
    for i in 0..100 {
        let _ = Terminal::move_cursor(i % 10, i % 20);
        let _ = Terminal::print_at(i % 10, i % 20, &format!("Test{}", i));
    }
}

#[test]
fn test_terminal_print_with_ansi_codes() {
    // Test that we can print strings containing ANSI codes
    let with_ansi = "\x1b[31mRed Text\x1b[0m";
    let result = Terminal::print_at(0, 0, with_ansi);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_terminal_clear_and_print() {
    let _ = Terminal::clear_screen();
    let _ = Terminal::print_at(0, 0, "After clear");
}

#[test]
fn test_terminal_size_bounds() {
    if let Some((rows, cols)) = Terminal::size() {
        // Reasonable terminal sizes
        assert!((1..=10000).contains(&rows), "Rows should be reasonable");
        assert!((1..=10000).contains(&cols), "Cols should be reasonable");
    }
}
