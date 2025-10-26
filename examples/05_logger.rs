// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example: Logger - Leveled logging

use kite::log::{Level, Logger};

fn main() {
    println!("Example 1: Default logger (Info level)");
    let logger = Logger::new();
    logger.error("This is an error message");
    logger.warn("This is a warning");
    logger.info("This is an info message");
    logger.debug("This won't be shown (below Info level)");

    println!("\nExample 2: Debug level logger");
    let logger = Logger::new().level(Level::Debug);
    logger.error("Error message");
    logger.warn("Warning message");
    logger.info("Info message");
    logger.debug("Debug message - now visible!");

    println!("\nExample 3: Error level only");
    let logger = Logger::new().level(Level::Error);
    logger.error("Only errors will be shown");
    logger.warn("This won't be shown");
    logger.info("This won't be shown");
    logger.debug("This won't be shown");

    println!("\nExample 4: Application logging");
    let logger = Logger::new().level(Level::Info);

    logger.info("Application starting...");

    // Simulate some work
    logger.debug("Loading configuration");
    logger.info("Configuration loaded");

    logger.debug("Connecting to database");
    logger.info("Database connected");

    logger.warn("Cache miss - using fallback");

    logger.info("Processing request");
    logger.debug("Request processed in 42ms");

    logger.error("Failed to send email notification");
    logger.warn("Retrying email send");

    logger.info("Application shutting down");
}
