//! # Operating System Platform Abstraction
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  Kite — os/mod.rs                                             ║
//! ║  Platform-specific code isolation                             ║
//! ║  Copyright © 2025 Jeet Karena <karenajeet@proton.me>        ║
//! ║  Licensed under MIT OR Apache-2.0                             ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```
//!
//! This module contains platform-specific implementations.
//! Unsafe code is allowed here but must have SAFETY comments.

// Allow unsafe code in this module and submodules
#![allow(unsafe_code)]

#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub mod unix;

/// Read a password with echo disabled (platform-specific implementation)
pub fn read_password() -> std::io::Result<String> {
    #[cfg(windows)]
    {
        windows::read_password()
    }

    #[cfg(unix)]
    {
        unix::read_password()
    }

    #[cfg(not(any(windows, unix)))]
    {
        // Fallback for other platforms
        use std::io::{self, Write};
        print!("Password: ");
        io::stdout().flush()?;
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;
        password.pop(); // Remove newline
        Ok(password)
    }
}
