// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example 1: Hello World - The simplest Kite program

use zfish::style::Color;

// Simple colored output
// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example 1: Hello World - The simplest zfish program

fn main() {
    println!("{}", Color::Green.paint("Hello, zfish! 🐟"));

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
