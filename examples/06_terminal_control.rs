// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example: Terminal Control - Cursor and screen manipulation

use std::thread;
use std::time::Duration;
use zfish::term::Terminal;

fn main() {
    println!("Example 1: Terminal size");
    if let Some((rows, cols)) = Terminal::size() {
        println!("Terminal size: {} rows × {} columns", rows, cols);
    } else {
        println!("Could not detect terminal size");
    }

    println!("\nExample 2: Clear screen (in 3 seconds...)");
    thread::sleep(Duration::from_secs(3));
    let _ = Terminal::clear_screen();

    println!("Screen cleared!");
    thread::sleep(Duration::from_secs(2));

    println!("\nExample 3: Cursor movement");
    let _ = Terminal::move_cursor(5, 10);
    println!("At position (5, 10)");

    let _ = Terminal::move_cursor(10, 20);
    println!("At position (10, 20)");

    thread::sleep(Duration::from_secs(2));

    println!("\nExample 4: Drawing a box");
    let _ = Terminal::clear_screen();

    let _ = Terminal::print_at(2, 5, "┌─────────────────┐");
    let _ = Terminal::print_at(3, 5, "│   zfish Demo     │");
    let _ = Terminal::print_at(4, 5, "│                 │");
    let _ = Terminal::print_at(5, 5, "│   Hello World!  │");
    let _ = Terminal::print_at(6, 5, "└─────────────────┘");

    let _ = Terminal::move_cursor(8, 0);
    println!("\nBox drawn using cursor positioning!");
}
