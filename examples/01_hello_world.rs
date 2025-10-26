// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example: Hello World - Basic usage

use kite::style::Color;

fn main() {
    // Simple colored output
    println!("{}", Color::Green.paint("Hello, Kite! 🪁"));
    
    // Multiple colors
    println!(
        "{} {} {}",
        Color::Red.paint("Red"),
        Color::Yellow.paint("Yellow"),
        Color::Blue.paint("Blue")
    );
    
    // Bright colors
    println!("{}", Color::BrightCyan.paint("Bright Cyan Text"));
    
    // Custom 256 colors
    println!("{}", Color::Custom(208).paint("Orange (256-color palette)"));
}
