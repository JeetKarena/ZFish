//! Kite: A secure, dependency-free Rust library for building CLI tools.
//! 
//! This library provides utilities for CLI development without any external dependencies,
//! using only Rust's standard library.

pub mod args;
pub mod style;
pub mod progress;
pub mod prompt;
pub mod log;
pub mod term;

// Re-export main components for easier access
pub use args::Args;
pub use style::Color;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
