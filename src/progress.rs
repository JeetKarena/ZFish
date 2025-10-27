//! Progress bar and spinner utilities for CLI applications.

use std::io::{self, Write};
use std::time::Instant;

/// Progress bar style (visual appearance)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressStyle {
    /// Classic style: [==========          ] (default)
    #[default]
    Classic,
    /// Arrow style: [=========>            ]
    Arrow,
    /// Dots style: [**********            ]
    Dots,
    /// Spinner style: [/|/|/|/|            ]
    Spinner,
}

/// A progress bar for displaying progress of operations.
#[derive(Debug)]
pub struct ProgressBar {
    total: u64,
    current: u64,
    /// The width of the progress bar in characters (default: 40).
    pub width: u16,
    /// The style of the progress bar
    pub style: ProgressStyle,
    start_time: Instant,
    spinner_frame: usize,
}

impl ProgressBar {
    /// Create a new progress bar with the given total steps.
    pub fn new(total: u64) -> Self {
        ProgressBar {
            total,
            current: 0,
            width: 40, // Default width
            style: ProgressStyle::default(),
            start_time: Instant::now(),
            spinner_frame: 0,
        }
    }

    /// Set the width of the progress bar.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set the style of the progress bar.
    pub fn with_style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
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
    fn render(&mut self) {
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

        // Build the progress bar based on style
        let bar = self.build_bar(filled_width, empty_width);

        // Format the progress bar
        let mut output = format!(
            "\r[{}] {:.1}% ({}/{}) {:.1}/s ETA: {:.1}s",
            bar, percent, current, self.total, items_per_sec, remaining_secs
        );

        // Truncate if too long for terminal
        if let Some((width, _)) = crate::term::Terminal::size() {
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

    /// Build the progress bar string based on the selected style
    fn build_bar(&mut self, filled_width: u16, empty_width: u16) -> String {
        match self.style {
            ProgressStyle::Classic => {
                // [==========          ]
                format!(
                    "{}{}",
                    "=".repeat(filled_width as usize),
                    " ".repeat(empty_width as usize)
                )
            }
            ProgressStyle::Arrow => {
                // [=========>          ]
                if filled_width == 0 {
                    " ".repeat(self.width as usize)
                } else if filled_width >= self.width {
                    "=".repeat(self.width as usize)
                } else {
                    format!(
                        "{}>{}",
                        "=".repeat((filled_width - 1) as usize),
                        " ".repeat(empty_width as usize)
                    )
                }
            }
            ProgressStyle::Dots => {
                // [**********          ]
                format!(
                    "{}{}",
                    "*".repeat(filled_width as usize),
                    " ".repeat(empty_width as usize)
                )
            }
            ProgressStyle::Spinner => {
                // [/|/|/|/|            ]  (animated)
                const SPINNER_CHARS: &[char] = &['/', '|', '\\', '|'];
                self.spinner_frame = (self.spinner_frame + 1) % SPINNER_CHARS.len();
                let spinner_char = SPINNER_CHARS[self.spinner_frame];

                let mut bar = String::with_capacity(self.width as usize);
                for i in 0..self.width {
                    if i < filled_width {
                        bar.push(spinner_char);
                    } else {
                        bar.push(' ');
                    }
                }
                bar
            }
        }
    }
}
