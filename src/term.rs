//! Terminal manipulation utilities.

use std::io::{self, Write};

/// Terminal utilities for cursor manipulation and screen clearing.
#[derive(Debug)]
pub struct Terminal;

impl Terminal {
    /// Clear the entire screen.
    pub fn clear_screen() -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\x1b[2J")?;
        handle.flush()
    }
    
    /// Move the cursor to the specified position (1-based coordinates).
    pub fn move_cursor(row: u16, col: u16) -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        write!(handle, "\x1b[{};{}H", row, col)?;
        handle.flush()
    }
    
    /// Get the terminal size (width, height) if available.
    pub fn size() -> Option<(u16, u16)> {
        // This is a simplified version. A real implementation would
        // use platform-specific code to detect terminal size.
        // For now, we'll just return a default size.
        Some((80, 24))
    }
    
    /// Print text at the specified position.
    pub fn print_at(row: u16, col: u16, text: &str) -> io::Result<()> {
        Self::move_cursor(row, col)?;
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(text.as_bytes())?;
        handle.flush()
    }
}