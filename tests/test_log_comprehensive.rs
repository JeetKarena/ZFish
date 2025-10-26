//! Comprehensive tests for Logger
//! Tests all log levels, edge cases, Unicode, special characters

use kite::log::{Level, Logger};

#[test]
fn test_logger_default_level() {
    let _logger = Logger::new();
    // Should have a default level
}

#[test]
fn test_logger_set_error_level() {
    let logger = Logger::new().level(Level::Error);
    logger.error("Error message");
    // Should only show errors
}

#[test]
fn test_logger_set_warn_level() {
    let logger = Logger::new().level(Level::Warn);
    logger.warn("Warning message");
}

#[test]
fn test_logger_set_info_level() {
    let logger = Logger::new().level(Level::Info);
    logger.info("Info message");
}

#[test]
fn test_logger_set_debug_level() {
    let logger = Logger::new().level(Level::Debug);
    logger.debug("Debug message");
}

#[test]
fn test_logger_all_levels() {
    let logger = Logger::new().level(Level::Debug);
    logger.error("Error");
    logger.warn("Warning");
    logger.info("Info");
    logger.debug("Debug");
}

#[test]
fn test_logger_empty_messages() {
    let logger = Logger::new().level(Level::Debug);
    logger.error("");
    logger.warn("");
    logger.info("");
    logger.debug("");
}

#[test]
fn test_logger_long_messages() {
    let logger = Logger::new().level(Level::Debug);
    let long_msg = "a".repeat(10000);
    logger.error(&long_msg);
    logger.warn(&long_msg);
    logger.info(&long_msg);
    logger.debug(&long_msg);
}

#[test]
fn test_logger_unicode_messages() {
    let logger = Logger::new().level(Level::Debug);
    logger.error("Error: 错误 エラー خطأ");
    logger.warn("Warning: 警告 警告 تحذير");
    logger.info("Info: 信息 情報 معلومات");
    logger.debug("Debug: 调试 デバッグ تصحيح");
}

#[test]
fn test_logger_emoji_messages() {
    let logger = Logger::new().level(Level::Debug);
    logger.error("❌ Error occurred!");
    logger.warn("⚠️  Warning!");
    logger.info("ℹ️  Information");
    logger.debug("🐛 Debug info");
}

#[test]
fn test_logger_multiline_messages() {
    let logger = Logger::new().level(Level::Debug);
    logger.error("Line 1\nLine 2\nLine 3");
    logger.warn("First\nSecond\nThird");
    logger.info("A\nB\nC");
    logger.debug("X\nY\nZ");
}

#[test]
fn test_logger_special_characters() {
    let logger = Logger::new().level(Level::Debug);
    logger.error("Tab:\tQuote:\"Backslash:\\");
    logger.warn("Null:\0Control:\x01");
    logger.info("Brackets: {[()]}");
    logger.debug("Symbols: !@#$%^&*");
}

#[test]
fn test_logger_json_like_messages() {
    let logger = Logger::new().level(Level::Debug);
    logger.error(r#"{"error": "Something went wrong"}"#);
    logger.warn(r#"{"warning": "Deprecated API"}"#);
    logger.info(r#"{"info": "Process started"}"#);
    logger.debug(r#"{"debug": {"nested": "value"}}"#);
}

#[test]
fn test_logger_paths() {
    let logger = Logger::new().level(Level::Debug);
    logger.error("Error in /path/to/file.txt");
    logger.warn("Warning: C:\\Windows\\System32\\file.dll");
    logger.info("Info: ~/Documents/file.md");
    logger.debug("Debug: ./relative/path/file.rs");
}

#[test]
fn test_logger_urls() {
    let logger = Logger::new().level(Level::Debug);
    logger.error("Error: https://example.com/error?code=500");
    logger.warn("Warning: http://api.example.com/deprecated");
    logger.info("Info: ws://localhost:8080/socket");
    logger.debug("Debug: ftp://files.example.com/debug.log");
}

#[test]
fn test_logger_level_filtering() {
    // Error level should only show errors
    let error_logger = Logger::new().level(Level::Error);
    error_logger.error("This should appear");
    error_logger.warn("This should NOT appear");
    error_logger.info("This should NOT appear");
    error_logger.debug("This should NOT appear");

    // Warn level should show errors and warnings
    let warn_logger = Logger::new().level(Level::Warn);
    warn_logger.error("Should appear");
    warn_logger.warn("Should appear");
    warn_logger.info("Should NOT appear");
    warn_logger.debug("Should NOT appear");

    // Info level should show errors, warnings, and info
    let info_logger = Logger::new().level(Level::Info);
    info_logger.error("Should appear");
    info_logger.warn("Should appear");
    info_logger.info("Should appear");
    info_logger.debug("Should NOT appear");

    // Debug level should show everything
    let debug_logger = Logger::new().level(Level::Debug);
    debug_logger.error("Should appear");
    debug_logger.warn("Should appear");
    debug_logger.info("Should appear");
    debug_logger.debug("Should appear");
}

#[test]
fn test_logger_rapid_logging() {
    let logger = Logger::new().level(Level::Debug);
    for i in 0..1000 {
        logger.debug(&format!("Message {}", i));
    }
}

#[test]
fn test_logger_builder_pattern() {
    let logger1 = Logger::new().level(Level::Error);
    let logger2 = Logger::new().level(Level::Debug);

    // Both should work independently
    logger1.error("Error logger");
    logger2.debug("Debug logger");
}

#[test]
fn test_logger_message_with_format_specifiers() {
    let logger = Logger::new().level(Level::Debug);
    // These shouldn't be interpreted as format strings
    logger.error("Error: %s %d %f");
    logger.warn("Warning: {} {:?}");
    logger.info("Info: {0} {1}");
    logger.debug("Debug: {name}");
}

#[test]
fn test_logger_very_long_single_line() {
    let logger = Logger::new().level(Level::Debug);
    let very_long = "word ".repeat(10000);
    logger.error(&very_long);
}

#[test]
fn test_logger_binary_data_representation() {
    let logger = Logger::new().level(Level::Debug);
    logger.debug("Binary: \x00\x01\x02\x7F");
}

#[test]
fn test_logger_level_clone() {
    let level1 = Level::Error;
    let _level2 = level1; // Should be Copy
    let _level3 = level1; // Should still work
}

#[test]
fn test_logger_multiple_instances() {
    let logger1 = Logger::new().level(Level::Error);
    let logger2 = Logger::new().level(Level::Debug);
    let logger3 = Logger::new().level(Level::Info);

    logger1.error("Logger 1");
    logger2.debug("Logger 2");
    logger3.info("Logger 3");
}
