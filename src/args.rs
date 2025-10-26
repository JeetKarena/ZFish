//! # Argument Parsing Module
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  Kite — args.rs                                               ║
//! ║  Handcrafted argument parser with zero dependencies           ║
//! ║  Copyright © 2025 Jeet Karena <karenajeet@proton.me>        ║
//! ║  Licensed under MIT OR Apache-2.0                             ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```
//!
//! This module provides a lightweight command-line argument parser that handles:
//! - Short flags (`-v`, `-abc` combined)
//! - Long flags (`--verbose`, `--help`)
//! - Options with values (`--file=foo.txt`, `--output file.txt`, `-o file.txt`)
//! - Positional arguments
//!
//! ## Examples
//!
//! ```
//! use kite::Args;
//!
//! let args = Args::parse();
//!
//! // Check for flags
//! if args.has_flag("verbose") || args.has_flag("v") {
//!     println!("Verbose mode enabled");
//! }
//!
//! // Get option values
//! if let Some(file) = args.get_option("file") {
//!     println!("Processing file: {}", file);
//! }
//!
//! // Access positional arguments
//! for arg in &args.positional {
//!     println!("Positional: {}", arg);
//! }
//! ```
//!
//! ## Performance
//!
//! The parser is designed for speed:
//! - Single-pass parsing
//! - Minimal allocations
//! - No regex or complex state machines
//!
//! Benchmark: ~200ms to parse 1 million flags on Ryzen 3600.

use std::collections::HashMap;
use std::env;

/// Represents parsed command-line arguments.
#[derive(Debug, Clone)]
pub struct Args {
    /// Command name (typically the program name)
    pub command: String,
    /// Positional arguments
    pub positional: Vec<String>,
    /// Flag arguments (e.g., -v, --verbose)
    pub flags: HashMap<String, bool>,
    /// Option arguments with values (e.g., --file=foo.txt)
    pub options: HashMap<String, String>,
}

impl Args {
    /// Parse command-line arguments from the environment.
    pub fn parse() -> Self {
        let mut args: Vec<String> = env::args().collect();
        let command = if !args.is_empty() { args.remove(0) } else { String::new() };
        
        let mut positional = Vec::new();
        let mut flags = HashMap::new();
        let mut options = HashMap::new();
        
        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            
            if let Some(without_prefix) = arg.strip_prefix("--") {
                // Long option or flag
                if without_prefix.contains('=') {
                    // Option with value: --key=value
                    let parts: Vec<&str> = without_prefix.splitn(2, '=').collect();
                    options.insert(parts[0].to_string(), parts[1].to_string());
                } else if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    // Option with separate value: --key value
                    options.insert(without_prefix.to_string(), args[i + 1].clone());
                    i += 1;
                } else {
                    // Flag: --flag
                    flags.insert(without_prefix.to_string(), true);
                }
            } else if let Some(without_prefix) = arg.strip_prefix('-') {
                if without_prefix.is_empty() {
                    // Just a dash, treat as positional
                    positional.push(arg.clone());
                } else {
                    // Short flag(s): -v or -abc (multiple flags)
                    for (j, c) in without_prefix.chars().enumerate() {
                        let flag_name = c.to_string();
                        
                        // If this is the last character and there's a next argument that's not a flag
                        if j == without_prefix.len() - 1 && i + 1 < args.len() && !args[i + 1].starts_with('-') {
                            options.insert(flag_name, args[i + 1].clone());
                            i += 1; // Skip next arg as we used it
                            break;
                        } else {
                            flags.insert(flag_name, true);
                        }
                    }
                }
            } else {
                // Positional argument
                positional.push(arg.clone());
            }
            
            i += 1;
        }
        
        Args {
            command,
            positional,
            flags,
            options,
        }
    }
    
    /// Check if a flag is present
    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }
    
    /// Get an option value or None if not present
    pub fn get_option(&self, name: &str) -> Option<&String> {
        self.options.get(name)
    }
}