// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example: Colored Text - Advanced styling

use kite::{Color, Style};

fn main() {
    println!("=== Basic Colors ===");
    println!("{}", Color::Red.paint("Red text"));
    println!("{}", Color::Green.paint("Green text"));
    println!("{}", Color::Blue.paint("Blue text"));
    println!();

    println!("=== Bright Colors ===");
    println!("{}", Color::BrightRed.paint("Bright Red"));
    println!("{}", Color::BrightGreen.paint("Bright Green"));
    println!("{}", Color::BrightBlue.paint("Bright Blue"));
    println!();

    println!("=== Text Styles ===");
    println!("{}", Color::White.paint("Bold text").style(Style::Bold));
    println!("{}", Color::White.paint("Italic text").style(Style::Italic));
    println!(
        "{}",
        Color::White
            .paint("Underlined text")
            .style(Style::Underline)
    );
    println!("{}", Color::White.paint("Dim text").style(Style::Dim));
    println!();

    println!("=== Combined Styles ===");
    println!(
        "{}",
        Color::Cyan
            .paint("Bold + Italic + Underline")
            .style(Style::Bold)
            .style(Style::Italic)
            .style(Style::Underline)
    );
    println!();

    println!("=== 256-Color Palette ===");
    for i in [196, 202, 208, 214, 220, 226] {
        println!("{}", Color::Custom(i).paint(format!("Color {}", i)));
    }
    println!();

    println!("=== Rainbow Example ===");
    let rainbow_colors = [
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
    ];

    for color in rainbow_colors {
        print!("{}", color.paint("█████ "));
    }
    println!();
}
