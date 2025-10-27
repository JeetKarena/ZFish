// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example: Complete CLI - Putting it all together

use zfish::{Args, Color, Level, Logger, ProgressBar, Style};
use std::thread;
use std::time::Duration;

fn main() {
    let args = Args::parse();

    // Setup logger based on verbosity
    let log_level = if args.has_flag("verbose") || args.has_flag("v") {
        Level::Debug
    } else {
        Level::Info
    };
    let logger = Logger::new().level(log_level);

    // Show help
    if args.has_flag("help") || args.has_flag("h") {
        show_help();
        return;
    }

    // Show version
    if args.has_flag("version") {
        println!(
            "{}",
            Color::Cyan
                .paint("zfish Example CLI v0.1.0")
                .style(Style::Bold)
        );
        return;
    }

    // Get configuration from arguments
    let iterations = args
        .get_option("iterations")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(50);

    let output_file = args
        .get_option("output")
        .map(|s| s.as_str())
        .unwrap_or("output.txt");

    // Start the application
    logger.info(&format!(
        "{}",
        Color::Green
            .paint("Starting application...")
            .style(Style::Bold)
    ));
    logger.debug(&format!("Iterations: {}", iterations));
    logger.debug(&format!("Output file: {}", output_file));

    // Process with progress bar
    logger.info("Processing data...");
    let mut pb = ProgressBar::new(iterations);

    for i in 0..iterations {
        thread::sleep(Duration::from_millis(50));

        if i % 10 == 0 {
            logger.debug(&format!("Processed {} items", i));
        }

        pb.set(i + 1);
    }

    pb.finish(&format!("{}", Color::Green.paint("✓ Processing complete!")));

    // Final output
    logger.info(&format!(
        "Results saved to: {}",
        Color::Cyan.paint(output_file)
    ));
    logger.info(&format!(
        "{}",
        Color::Green.paint("✓ Done!").style(Style::Bold)
    ));

    // Summary
    println!(
        "\n{}",
        Color::Magenta.paint("=== Summary ===").style(Style::Bold)
    );
    println!("Processed: {} items", iterations);
    println!("Output: {}", output_file);
    println!("Status: {}", Color::Green.paint("Success"));
}

fn show_help() {
    println!(
        "{}",
        Color::Cyan.paint("zfish Example CLI").style(Style::Bold)
    );
    println!();
    println!("USAGE:");
    println!("    {} [OPTIONS]", std::env::args().next().unwrap());
    println!();
    println!("OPTIONS:");
    println!("    -h, --help                 Print help information");
    println!("    --version                  Print version");
    println!("    -v, --verbose              Enable verbose logging");
    println!("    --iterations <N>           Number of iterations [default: 50]");
    println!("    -o, --output <FILE>        Output file path [default: output.txt]");
    println!();
    println!("EXAMPLES:");
    println!("    {} --iterations 100", std::env::args().next().unwrap());
    println!(
        "    {} --verbose -o results.txt",
        std::env::args().next().unwrap()
    );
}

// Run with:
// cargo run --example 08_complete_cli -- --help
// cargo run --example 08_complete_cli -- --verbose --iterations 30
// cargo run --example 08_complete_cli -- -o myfile.txt
