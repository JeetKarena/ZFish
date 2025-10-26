// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example: Progress Bar - Track long-running operations

use kite::progress::ProgressBar;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Example 1: Basic progress bar");
    let mut pb = ProgressBar::new(100);
    for i in 0..100 {
        thread::sleep(Duration::from_millis(20));
        pb.set(i + 1);
    }
    pb.finish("✓ Complete!");

    println!("\nExample 2: File processing simulation");
    let files = [
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "file4.txt",
        "file5.txt",
    ];
    let mut pb = ProgressBar::new(files.len() as u64);
    for (i, file) in files.iter().enumerate() {
        thread::sleep(Duration::from_millis(500));
        pb.set((i + 1) as u64);
        println!("\nProcessed: {}", file);
    }
    pb.finish("✓ All files processed!");

    println!("\nExample 3: Incremental progress");
    let mut pb = ProgressBar::new(50);
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(50));
        pb.inc(1);
    }
    pb.finish("✓ Done!");

    println!("\nExample 4: Custom width");
    let mut pb = ProgressBar::new(100).width(60);
    for i in 0..100 {
        thread::sleep(Duration::from_millis(10));
        pb.set(i + 1);
    }
    pb.finish("✓ Custom width complete!");
}
