//! Argument parsing functionality for CLI applications.

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
            
            if arg.starts_with("--") {
                // Long option or flag
                let without_prefix = &arg[2..];
                
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
            } else if arg.starts_with('-') && arg.len() > 1 {
                // Short flag(s): -v or -abc (multiple flags)
                for (j, c) in arg[1..].chars().enumerate() {
                    let flag_name = c.to_string();
                    
                    // If this is the last character and there's a next argument that's not a flag
                    if j == arg[1..].len() - 1 && i + 1 < args.len() && !args[i + 1].starts_with('-') {
                        options.insert(flag_name, args[i + 1].clone());
                        i += 1;
                    } else {
                        flags.insert(flag_name, true);
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