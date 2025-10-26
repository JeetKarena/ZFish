//! Progress bar and spinner utilities for CLI applications.

use std::io::{self, Write};
use std::time::Instant;

/// A progress bar for displaying progress of operations.
#[derive(Debug)]
pub struct ProgressBar {
    total: u64,
    current: u64,
    /// The width of the progress bar in characters (default: 40).
    pub width: u16,
    start_time: Instant,
}

impl ProgressBar {
    /// Create a new progress bar with the given total steps.
    pub fn new(total: u64) -> Self {
        ProgressBar {
            total,
            current: 0,
            width: 40, // Default width
            start_time: Instant::now(),
        }
    }

    /// Set the width of the progress bar.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Update the progress bar to the given position.
    pub fn set(&mut self, position: u64) {
        self.current = position;
        self.render();
    }

    /// Increment the progress bar by the given amount.
    pub fn inc(&mut self, amount: u64) {
        self.current = std::cmp::min(self.current + amount, self.total);
        self.render();
    }

    /// Finish the progress bar with an optional message.
    pub fn finish(&mut self, message: &str) {
        self.current = self.total;
        self.render();

        // Move to the next line and display completion message
        println!("\n{}", message);
    }

    /// Render the progress bar to stdout.
    fn render(&self) {
        // Cap current at total to prevent overflow
        let current = self.current.min(self.total);

        let percent = (current as f64 / self.total as f64) * 100.0;
        let filled_width = (self.width as f64 * (current as f64 / self.total as f64)) as u16;
        let empty_width = self.width - filled_width;

        // Calculate elapsed time and estimate remaining time
        let elapsed = self.start_time.elapsed();
        let elapsed_secs = elapsed.as_secs_f64();
        let items_per_sec = if elapsed_secs > 0.0 {
            current as f64 / elapsed_secs
        } else {
            0.0
        };
        let remaining_secs = if items_per_sec > 0.0 && current < self.total {
            (self.total - current) as f64 / items_per_sec
        } else {
            0.0
        };

        // Format the progress bar
        let mut output = format!(
            "\r[{}{}] {:.1}% ({}/{}) {:.1}/s ETA: {:.1}s",
            "=".repeat(filled_width as usize),
            " ".repeat(empty_width as usize),
            percent,
            current,
            self.total,
            items_per_sec,
            remaining_secs
        );

        // Truncate if too long for terminal
        if let Some((width, _)) = terminal_size() {
            let max_len = width as usize;
            if output.len() > max_len {
                output.truncate(max_len);
            }
        }

        // Print the progress bar (without newline)
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let _ = handle.write_all(output.as_bytes());
        let _ = handle.flush();
    }
}

/// Get the terminal size (width, height) if available.
fn terminal_size() -> Option<(u16, u16)> {
    // This is a simplified version. A real implementation would
    // use platform-specific code to detect terminal size.
    // For now, we'll just return a default size.
    Some((80, 24))
}
