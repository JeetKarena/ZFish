use kite_cli::progress::ProgressBar;

#[test]
fn test_progress_bar_creation() {
    let pb = ProgressBar::new(100);

    // Test that we can create a progress bar without errors
    // Default width should be 40
    assert_eq!(pb.width, 40);
}

#[test]
fn test_progress_bar_updates() {
    let mut pb = ProgressBar::new(100);

    // Ensure we can update progress
    pb.set(50);

    // Increment and make sure it stays within bounds
    pb.inc(30);
    pb.inc(100); // Should cap at 100

    // Finish should work without panicking
    pb.finish("Done!");
}
