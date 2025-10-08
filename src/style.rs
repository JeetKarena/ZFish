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
    /// Custom 256-color (0-255)
    Custom(u8),
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
    /// Convert color to its ANSI foreground code string
    /// For standard colors: returns the code (e.g., "31")
    /// For custom 256 colors: returns "38;5;n"
    fn to_fg_code_string(&self) -> String {
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
        std::env::var("TERM").map_or(false, |term| term != "dumb") ||
        std::env::var("COLORTERM").is_ok()
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