//! # Terminal Styling and Color Module
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  Kite — style.rs                                              ║
//! ║  ANSI color and text styling without dependencies            ║
//! ║  Copyright © 2025 Jeet Karena <karenajeet@proton.me>        ║
//! ║  Licensed under MIT OR Apache-2.0                             ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```
//!
//! This module provides zero-dependency terminal styling with support for:
//! - 16 standard ANSI colors (8 normal + 8 bright)
//! - 256-color palette (`Color::Custom(0..=255)`)
//! - Text styles (bold, italic, underline, etc.)
//! - Automatic color detection (`NO_COLOR` env var, `COLORTERM` support)
//!
//! ## Examples
//!
//! ### Basic Colors
//!
//! ```
//! use kite::Color;
//!
//! // Standard colors
//! println!("{}", Color::Red.paint("Error!"));
//! println!("{}", Color::Green.paint("Success!"));
//! println!("{}", Color::Yellow.paint("Warning!"));
//!
//! // Bright colors
//! println!("{}", Color::BrightBlue.paint("Info"));
//! ```
//!
//! ### 256-Color Palette
//!
//! ```
//! use kite::Color;
//!
//! // Custom 256 colors (0-255)
//! println!("{}", Color::Custom(196).paint("Bright red"));
//! println!("{}", Color::Custom(46).paint("Bright green"));
//! println!("{}", Color::Custom(21).paint("Deep blue"));
//! ```
//!
//! ### Text Styling
//!
//! ```
//! use kite::{Color, Style};
//!
//! // Combine colors and styles
//! let text = Color::Red.paint("Error").style(Style::Bold);
//! println!("{}", text);
//!
//! // Multiple styles
//! let fancy = Color::Cyan.paint("Title")
//!     .style(Style::Bold)
//!     .style(Style::Underline);
//! println!("{}", fancy);
//! ```
//!
//! ## Color Detection
//!
//! The module automatically detects terminal capabilities:
//! - Respects `NO_COLOR` environment variable (disables all colors)
//! - Checks `COLORTERM` for true color support
//! - Checks `TERM` for basic ANSI support
//!
//! ## Performance
//!
//! - Zero allocations on color detection
//! - Minimal string formatting overhead
//! - Fast ANSI code generation

use std::fmt;

/// ANSI color codes for terminal output
#[derive(Debug, Clone, Copy)]
pub enum Color {
    /// Black (ANSI code 30)
    Black,
    /// Red (ANSI code 31)
    Red,
    /// Green (ANSI code 32)
    Green,
    /// Yellow (ANSI code 33)
    Yellow,
    /// Blue (ANSI code 34)
    Blue,
    /// Magenta (ANSI code 35)
    Magenta,
    /// Cyan (ANSI code 36)
    Cyan,
    /// White (ANSI code 37)
    White,
    /// Bright Black / Gray (ANSI code 90)
    BrightBlack,
    /// Bright Red (ANSI code 91)
    BrightRed,
    /// Bright Green (ANSI code 92)
    BrightGreen,
    /// Bright Yellow (ANSI code 93)
    BrightYellow,
    /// Bright Blue (ANSI code 94)
    BrightBlue,
    /// Bright Magenta (ANSI code 95)
    BrightMagenta,
    /// Bright Cyan (ANSI code 96)
    BrightCyan,
    /// Bright White (ANSI code 97)
    BrightWhite,
    /// Custom 256-color (0-255) using ANSI sequence `\x1b[38;5;Nm`
    Custom(u8),
}

/// Text styling options
#[derive(Debug, Clone, Copy)]
pub enum Style {
    /// Bold or increased intensity (ANSI code 1)
    Bold,
    /// Dim or decreased intensity (ANSI code 2)
    Dim,
    /// Italic (ANSI code 3) - not widely supported
    Italic,
    /// Underline (ANSI code 4)
    Underline,
    /// Slow blink (ANSI code 5) - not widely supported
    Blink,
    /// Reverse video / invert colors (ANSI code 7)
    Reverse,
    /// Hidden / invisible text (ANSI code 8)
    Hidden,
}

/// A styled string with color and style attributes
#[derive(Debug)]
pub struct StyledString {
    text: String,
    color: Option<Color>,
    styles: Vec<Style>,
}

impl Color {
    /// Convert color to its ANSI foreground code string
    /// For standard colors: returns the code (e.g., "31")
    /// For custom 256 colors: returns "38;5;n"
    fn to_fg_code_string(self) -> String {
        match self {
            Color::Black => "30".to_string(),
            Color::Red => "31".to_string(),
            Color::Green => "32".to_string(),
            Color::Yellow => "33".to_string(),
            Color::Blue => "34".to_string(),
            Color::Magenta => "35".to_string(),
            Color::Cyan => "36".to_string(),
            Color::White => "37".to_string(),
            Color::BrightBlack => "90".to_string(),
            Color::BrightRed => "91".to_string(),
            Color::BrightGreen => "92".to_string(),
            Color::BrightYellow => "93".to_string(),
            Color::BrightBlue => "94".to_string(),
            Color::BrightMagenta => "95".to_string(),
            Color::BrightCyan => "96".to_string(),
            Color::BrightWhite => "97".to_string(),
            Color::Custom(n) => format!("38;5;{}", n),
        }
    }

    /// Apply color to a string
    pub fn paint<T: Into<String>>(self, text: T) -> StyledString {
        StyledString {
            text: text.into(),
            color: Some(self),
            styles: Vec::new(),
        }
    }
}

impl Style {
    /// Convert style to its ANSI style code
    fn code(self) -> u8 {
        match self {
            Style::Bold => 1,
            Style::Dim => 2,
            Style::Italic => 3,
            Style::Underline => 4,
            Style::Blink => 5,
            Style::Reverse => 7,
            Style::Hidden => 8,
        }
    }

    /// Apply style to a string
    pub fn apply<T: Into<String>>(&self, text: T) -> StyledString {
        StyledString {
            text: text.into(),
            color: None,
            styles: vec![*self],
        }
    }
}

impl StyledString {
    /// Add a style to this styled string
    pub fn style(mut self, style: Style) -> Self {
        self.styles.push(style);
        self
    }

    /// Detect if terminal supports colors
    fn supports_colors() -> bool {
        // `NO_COLOR` environment variable should always disable colors.
        if std::env::var("NO_COLOR").is_ok() {
            return false;
        }

        // In a test environment, enable colors if `COLORTERM` is set,
        // which indicates explicit support.
        if cfg!(test) {
            return std::env::var("COLORTERM").is_ok();
        }

        // Standard detection for non-test environments.
        std::env::var("COLORTERM").is_ok_and(|_| true)
            || std::env::var("TERM").is_ok_and(|term| term != "dumb")
    }
}

impl fmt::Display for StyledString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !StyledString::supports_colors() {
            return write!(f, "{}", self.text);
        }

        // Start building the ANSI escape sequence
        let mut codes = Vec::new();

        // Add color code if present
        if let Some(color) = self.color {
            codes.push(color.to_fg_code_string());
        }

        // Add style codes (convert to string)
        for style in &self.styles {
            codes.push(style.code().to_string());
        }

        if codes.is_empty() {
            // No styling to apply
            write!(f, "{}", self.text)
        } else {
            // Write the styled text with ANSI escape codes
            write!(f, "\x1b[{}m{}\x1b[0m", codes.join(";"), self.text)
        }
    }
}
