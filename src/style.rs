//! Text styling and color functionality for terminal output.

use std::fmt;

/// ANSI color codes for terminal output
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

/// Text styling options
#[derive(Debug, Clone, Copy)]
pub enum Style {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
}

/// A styled string with color and style attributes
pub struct StyledString {
    text: String,
    color: Option<Color>,
    styles: Vec<Style>,
}

impl Color {
    /// Convert color to its ANSI color code
    fn to_fg_code(&self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::BrightBlack => 90,
            Color::BrightRed => 91,
            Color::BrightGreen => 92,
            Color::BrightYellow => 93,
            Color::BrightBlue => 94,
            Color::BrightMagenta => 95,
            Color::BrightCyan => 96,
            Color::BrightWhite => 97,
        }
    }
    
    /// Apply color to a string
    pub fn paint<T: Into<String>>(&self, text: T) -> StyledString {
        StyledString {
            text: text.into(),
            color: Some(*self),
            styles: Vec::new(),
        }
    }
}

impl Style {
    /// Convert style to its ANSI style code
    fn to_code(&self) -> u8 {
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
        // Force colors in test environment
        if cfg!(test) {
            return true;
        }
        
        // Simple detection based on environment variables
        std::env::var("NO_COLOR").is_err() && 
        (std::env::var("TERM").map(|term| term != "dumb").unwrap_or(false) ||
         std::env::var("COLORTERM").is_ok())
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
            codes.push(color.to_fg_code().to_string());
        }
        
        // Add style codes
        for style in &self.styles {
            codes.push(style.to_code().to_string());
        }
        
        if codes.is_empty() {
            // No styling to apply
            write!(f, "{}", self.text)
        } else {
            // Write the styled text with ANSI escape codes
            write!(
                f, 
                "\x1b[{}m{}\x1b[0m", 
                codes.join(";"), 
                self.text
            )
        }
    }
}