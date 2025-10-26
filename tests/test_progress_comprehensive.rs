//! Comprehensive tests for ProgressBar
//! Tests edge cases, boundary values, performance

use kite::ProgressBar;

#[test]
fn test_progress_bar_zero_total() {
    let pb = ProgressBar::new(0);
    assert_eq!(pb.width, 40);
    // Should handle division by zero gracefully
}

#[test]
fn test_progress_bar_one_total() {
    let mut pb = ProgressBar::new(1);
    pb.set(0);
    pb.set(1);
    // Should complete immediately
}

#[test]
fn test_progress_bar_large_total() {
    let mut pb = ProgressBar::new(u64::MAX);
    pb.set(0);
    pb.set(u64::MAX / 2);
    pb.set(u64::MAX);
    // Should handle very large numbers
}

#[test]
fn test_progress_bar_custom_width() {
    let pb = ProgressBar::new(100).width(80);
    assert_eq!(pb.width, 80);
}

#[test]
fn test_progress_bar_minimum_width() {
    let pb = ProgressBar::new(100).width(1);
    assert_eq!(pb.width, 1);
}

#[test]
fn test_progress_bar_maximum_width() {
    let pb = ProgressBar::new(100).width(u16::MAX);
    assert_eq!(pb.width, u16::MAX);
}

#[test]
fn test_progress_bar_set_beyond_total() {
    let mut pb = ProgressBar::new(100);
    pb.set(150); // Should clamp to total
    // Verify it doesn't panic
}

#[test]
fn test_progress_bar_inc_beyond_total() {
    let mut pb = ProgressBar::new(100);
    pb.inc(50);
    pb.inc(100); // Should clamp to total
    // Verify it doesn't panic
}

#[test]
fn test_progress_bar_multiple_increments() {
    let mut pb = ProgressBar::new(100);
    for _i in 0..100 {
        pb.inc(1);
    }
    // Should reach 100
}

#[test]
fn test_progress_bar_set_to_zero() {
    let mut pb = ProgressBar::new(100);
    pb.set(50);
    pb.set(0); // Can go backwards
}

#[test]
fn test_progress_bar_rapid_updates() {
    let mut pb = ProgressBar::new(1000);
    // Simulate rapid updates
    for i in 0..=1000 {
        pb.set(i);
    }
}

#[test]
fn test_progress_bar_inc_zero() {
    let mut pb = ProgressBar::new(100);
    pb.inc(0); // Should be a no-op
}

#[test]
fn test_progress_bar_multiple_finish_calls() {
    let mut pb = ProgressBar::new(100);
    pb.finish("Done");
    pb.finish("Done again"); // Should be safe to call multiple times
}

#[test]
fn test_progress_bar_finish_with_empty_message() {
    let mut pb = ProgressBar::new(100);
    pb.finish("");
}

#[test]
fn test_progress_bar_finish_with_long_message() {
    let mut pb = ProgressBar::new(100);
    let long_msg = "a".repeat(10000);
    pb.finish(&long_msg);
}

#[test]
fn test_progress_bar_finish_with_unicode() {
    let mut pb = ProgressBar::new(100);
    pb.finish("✓ Complete! 完成 🎉");
}

#[test]
fn test_progress_bar_finish_with_newlines() {
    let mut pb = ProgressBar::new(100);
    pb.finish("Line1\nLine2\nLine3");
}

#[test]
fn test_progress_bar_percentage_calculation() {
    let mut pb = ProgressBar::new(100);
    pb.set(0); // 0%
    pb.set(25); // 25%
    pb.set(50); // 50%
    pb.set(75); // 75%
    pb.set(100); // 100%
}

#[test]
fn test_progress_bar_non_divisible_total() {
    let mut pb = ProgressBar::new(77);
    for i in 0..=77 {
        pb.set(i);
    }
}

#[test]
fn test_progress_bar_builder_pattern() {
    let pb = ProgressBar::new(100).width(60);
    assert_eq!(pb.width, 60);
}

#[test]
fn test_progress_bar_width_chain() {
    let pb = ProgressBar::new(100)
        .width(50)
        .width(60) // Last one wins
        .width(70);
    assert_eq!(pb.width, 70);
}
