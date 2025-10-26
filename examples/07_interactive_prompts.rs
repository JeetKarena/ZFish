// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example: Interactive Prompts - User input

use kite_cli::{Color, Prompt, Style};

fn main() {
    println!(
        "{}\n",
        Color::Cyan
            .paint("=== Interactive Prompts Demo ===")
            .style(Style::Bold)
    );

    // Example 1: Text input
    println!("Example 1: Text Input");
    if let Ok(name) = Prompt::input("What is your name? ") {
        println!("Hello, {}!\n", Color::Green.paint(&name).style(Style::Bold));
    }

    // Example 2: Confirmation
    println!("Example 2: Confirmation");
    if let Ok(confirmed) = Prompt::confirm("Do you like Rust? ", true) {
        if confirmed {
            println!("{}\n", Color::Green.paint("Great choice! 🦀"));
        } else {
            println!("{}\n", Color::Yellow.paint("That's okay, give it time!"));
        }
    }

    // Example 3: Text input (simple)
    println!("Example 3: Simple Text Input");
    if let Ok(lang) = Prompt::text("Favorite language? ") {
        println!(
            "You chose: {}\n",
            Color::Cyan.paint(&lang).style(Style::Bold)
        );
    }

    // Example 4: Password input
    println!("Example 4: Password Input");
    if let Ok(_password) = Prompt::password("Enter password: ") {
        println!("{}\n", Color::Green.paint("✓ Password accepted (hidden)"));
    }

    // Example 5: Build a simple survey
    println!(
        "\n{}",
        Color::Magenta
            .paint("=== Quick Survey ===")
            .style(Style::Bold)
    );

    if let Ok(email) = Prompt::input("Email address: ")
        && let Ok(age) = Prompt::input("Age: ")
        && let Ok(subscribe) = Prompt::confirm("Subscribe to newsletter? ", false)
    {
        println!(
            "\n{}",
            Color::Green
                .paint("=== Survey Results ===")
                .style(Style::Bold)
        );
        println!("Email: {}", email);
        println!("Age: {}", age);
        println!("Newsletter: {}", if subscribe { "Yes" } else { "No" });
        println!("\n{}", Color::Green.paint("✓ Thank you!"));
    }
}

// Run with:
// cargo run --example 07_interactive_prompts
