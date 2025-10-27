// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example: Progress Bar - Track long-running operations with different styles

use zfish::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== zfish Progress Bar Styles Demo ===\n");

    // Example 1: Classic Style (default)
    println!("Example 1: Classic style [==========          ]");
    let mut pb = ProgressBar::new(100);
    for i in 0..=100 {
        thread::sleep(Duration::from_millis(20));
        pb.set(i);
    }
    pb.finish("✓ Classic complete!");

    // Example 2: Arrow Style
    println!("\nExample 2: Arrow style [=========>          ]");
    let mut pb = ProgressBar::new(100).with_style(ProgressStyle::Arrow);
    for i in 0..=100 {
        thread::sleep(Duration::from_millis(20));
        pb.set(i);
    }
    pb.finish("✓ Arrow complete!");

    // Example 3: Dots Style
    println!("\nExample 3: Dots style [**********          ]");
    let mut pb = ProgressBar::new(100).with_style(ProgressStyle::Dots);
    for i in 0..=100 {
        thread::sleep(Duration::from_millis(20));
        pb.set(i);
    }
    pb.finish("✓ Dots complete!");

    // Example 4: Spinner Style (animated)
    println!("\nExample 4: Spinner style [/|/|/|/|          ]");
    let mut pb = ProgressBar::new(100).with_style(ProgressStyle::Spinner);
    for i in 0..=100 {
        thread::sleep(Duration::from_millis(20));
        pb.set(i);
    }
    pb.finish("✓ Spinner complete!");

    // Example 5: Incremental progress with Arrow
    println!("\nExample 5: Incremental progress (Arrow style)");
    let mut pb = ProgressBar::new(50).with_style(ProgressStyle::Arrow);
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(30));
        pb.inc(1);
    }
    pb.finish("✓ Incremental done!");

    // Example 6: Custom width with Spinner
    println!("\nExample 6: Custom width (60 chars, Spinner)");
    let mut pb = ProgressBar::new(80).width(60).with_style(ProgressStyle::Spinner);
    for i in 0..=80 {
        thread::sleep(Duration::from_millis(15));
        pb.set(i);
    }
    pb.finish("✓ Custom width complete!");

    println!("\n=== All examples complete! ===");
}
