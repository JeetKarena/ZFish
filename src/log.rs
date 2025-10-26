//! Logging utilities for CLI applications.

use crate::style::Color;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH}; // Removed unused Style import

/// Log levels for different types of messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Error level - highest priority.
    Error,
    /// Warning level.
    Warn,
    /// Informational level.
    Info,
    /// Debug level - lowest priority.
    Debug,
}

/// A simple logger for CLI applications.
#[derive(Debug)]
pub struct Logger {
    level: Level,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    /// Create a new logger with the default level (Info).
    pub fn new() -> Self {
        Logger { level: Level::Info }
    }

    /// Set the maximum log level.
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Log an error message.
    pub fn error(&self, message: &str) {
        self.log(Level::Error, message);
    }

    /// Log a warning message.
    pub fn warn(&self, message: &str) {
        self.log(Level::Warn, message);
    }

    /// Log an info message.
    pub fn info(&self, message: &str) {
        self.log(Level::Info, message);
    }

    /// Log a debug message.
    pub fn debug(&self, message: &str) {
        self.log(Level::Debug, message);
    }

    /// Log a message with the given level.
    fn log(&self, level: Level, message: &str) {
        if level > self.level {
            return;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Format the log message with color based on level
        let level_str = match level {
            Level::Error => Color::BrightRed.paint("ERROR"),
            Level::Warn => Color::BrightYellow.paint("WARN "),
            Level::Info => Color::BrightBlue.paint("INFO "),
            Level::Debug => Color::BrightBlack.paint("DEBUG"),
        };

        let output = format!("[{}] {} {}\n", timestamp, level_str, message);

        // Write to stderr
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        let _ = handle.write_all(output.as_bytes());
        let _ = handle.flush();
    }
}
