//! # zfish — Ultra-Light Zero-Dependency CLI Framework
//!
//! ```text
//! ╭─╮
//! │ ╰─╮  zfish — Soar above the complexity
//! ╰─╯
//! ```
//!
//! **zfish** is a zero-dependency Rust library for building beautiful, fast, and reliable
//! command-line applications. Built with only Rust's standard library, zfish provides
//! everything you need for modern CLI development without the bloat.
//!
//! ## Philosophy
//!
//! - **Zero Dependencies**: Build on `std` alone — no supply-chain risk, minimal compile times
//! - **Cross-Platform**: Works seamlessly on Linux, macOS, and Windows
//! - **Lightweight**: Cold start under 5ms, parse millions of flags in milliseconds
//! - **Safe**: `#![forbid(unsafe_code)]` in public API (platform-specific code isolated)
//!
//! ## Features
//!
//! - **Argument Parsing**: Handcrafted lexer for `--flags`, `-abc` combos, subcommands
//! - **Terminal Styling**: ANSI colors (16 + 256-color palette), bold, italic, underline
//! - **Progress Bars**: Multi-bar support with throughput display
//! - **Interactive Prompts**: Text input, password entry, confirmation dialogs
//! - **Logging**: Leveled output with timestamp support
//! - **Terminal Control**: Size detection, cursor movement, screen clearing
//!
//! ## Quick Start
//!
//! ```rust
//! use zfish::{Args, Color};
//!
//! let args = Args::parse();
//! if args.has_flag("verbose") {
//!     println!("{}", Color::Green.paint("✓ Verbose mode enabled"));
//! }
//! ```
//!
//! ## Feature Flags
//!
//! - `colour` (default): Enable ANSI color support
//! - `raw`: Raw terminal mode for interactive apps
//! - `progress`: Progress bars and spinners (requires `raw`)
//! - `interactive`: Interactive prompts (requires `raw`)
//!
//! ## Project Status
//!
//! **Current Version**: 0.1.10 (Active Development)
//!
//! See [ROADMAP.md](../ROADMAP.md) for future plans.
//!
//! ## Documentation
//!
//! - **[Developer Docs](https://zfish-devdocs.vercel.app)** — Interactive guides and tutorials
//! - **[API Reference](https://docs.rs/zfish)** — Complete API documentation
//! - **[Roadmap](https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232)** — Feature status
//!
//! ## License
//!
//! Licensed under the [MIT License](https://opensource.org/licenses/MIT).
//!
//! ## Credits
//!
//! Created by **Jeet Karena** with ❤️ for the Rust community.
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  zfish v0.1.10                                                ║
//! ║  Copyright © 2025 Jeet Karena                                 ║
//! ║  Licensed under MIT License                                   ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```

#![deny(unsafe_code)] // Deny unsafe code by default (can be overridden in os module)
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/zfish/0.1.0")]

//! Core modules

pub mod args;
pub mod command;
pub mod log;
pub mod progress;
pub mod prompt;
pub mod style;
pub mod table;
pub mod term;
pub mod unicode;

// Platform-specific code (unsafe allowed here)
pub(crate) mod os;

// Re-export main components for easier access
pub use args::Args;
pub use command::{App, Arg, ArgMatches, Command, CommandError, CommandResult};
pub use log::{Level, Logger};
pub use progress::{ProgressBar, ProgressStyle};
pub use prompt::Prompt;
pub use style::{Color, Style};
pub use table::{Alignment, BoxStyle, Table, draw_box, draw_separator};
pub use term::Terminal;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// ASCII art logo
pub const LOGO: &str = r#"
╭─╮
│ ╰─╮  zfish — Ultra-light CLI framework
╰─╯";
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        // Verify VERSION matches Cargo.toml version
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
        // Ensure it's a valid semver format (X.Y.Z)
        assert!(
            VERSION.chars().filter(|c| *c == '.').count() == 2,
            "Version should have format X.Y.Z"
        );
    }

    #[test]
    fn test_logo() {
        assert!(LOGO.contains("zfish"));
    }
}
