use zfish::log::{Level, Logger};

#[test]
fn test_logger_levels() {
    // Create a logger with the default level (Info)
    let logger = Logger::new();

    // These shouldn't panic
    logger.error("This is an error");
    logger.warn("This is a warning");
    logger.info("This is info");

    // Debug messages won't be shown with default level
    logger.debug("This debug message shouldn't be visible");

    // Test with explicit debug level
    let debug_logger = Logger::new().level(Level::Debug);
    debug_logger.debug("This debug message should be visible");

    // Test level comparison
    assert!(Level::Error < Level::Warn);
    assert!(Level::Warn < Level::Info);
    assert!(Level::Info < Level::Debug);
}
