//! # User Input Prompting Module
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  Kite — prompt.rs                                             ║
//! ║  Interactive user input without dependencies                  ║
//! ║  Copyright © 2025 Jeet Karena <karenajeet@proton.me>        ║
//! ║  Licensed under MIT OR Apache-2.0                             ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```
//!
//! This module provides utilities for interactive user input in CLI applications.

use std::io::{self, Write};

/// Utilities for prompting user input
#[derive(Debug)]
pub struct Prompt;

impl Prompt {
    /// Prompt for a yes/no confirmation
    pub fn confirm(prompt: &str, default: bool) -> io::Result<bool> {
        let yes_no = if default { "[Y/n]" } else { "[y/N]" };
        let full_prompt = format!("{} {} ", prompt, yes_no);

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(full_prompt.as_bytes())?;
        handle.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();

        Ok(match input.as_str() {
            "y" | "yes" => true,
            "n" | "no" => false,
            "" => default,
            _ => false,
        })
    }

    /// Prompt for a line of text input
    pub fn input(prompt: &str) -> io::Result<String> {
        let full_prompt = format!("{} ", prompt);

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(full_prompt.as_bytes())?;
        handle.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }

    /// Prompt for a password with hidden input
    pub fn password(prompt: &str) -> io::Result<String> {
        let full_prompt = format!("{} ", prompt);

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(full_prompt.as_bytes())?;
        handle.flush()?;

        // Use platform-specific implementation from os module
        let password = crate::os::read_password()?;

        // Print a newline since we don't echo during password input
        println!();

        Ok(password)
    }

    /// Alias for `input` - prompt for text input
    pub fn text(prompt: &str) -> io::Result<String> {
        Self::input(prompt)
    }
}
