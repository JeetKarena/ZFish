use kite::term::Terminal;

#[test]
fn test_terminal_size() {
    let size = Terminal::size();
    assert!(size.is_some());

    if let Some((width, height)) = size {
        assert!(width > 0);
        assert!(height > 0);
        println!("Terminal size: {}x{}", width, height);
    }
}

#[test]
#[ignore]
fn test_terminal_clear() {
    // This is a visual test
    println!("The screen will clear in 2 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    Terminal::clear_screen().unwrap();
    println!("Screen should be cleared now");
}

#[test]
#[ignore]
fn test_cursor_movement() {
    Terminal::clear_screen().unwrap();

    // Print something at the top
    println!("This is at the default position");

    // Move cursor and print elsewhere
    Terminal::print_at(10, 20, "This text should be at row 10, column 20").unwrap();

    // Move back to bottom
    Terminal::move_cursor(20, 1).unwrap();
    println!("Back at the bottom");
}
