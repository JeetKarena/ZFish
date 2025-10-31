//! Demonstration of Core Features
//!
//! This example demonstrates the three core features requested:
//! 1. Auto-generated --help / -h output
//! 2. Argument validation (required vs optional)
//! 3. Default values for options

use zfish::command::{App, Arg, Command};

fn main() {
    let app = App::new("myapp")
        .version("1.0.0")
        .about("Demo of core ZFish features")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .about("Configuration file path")
                .default_value("config.toml"), // ✅ FEATURE 3: Default value
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file path")
                .required(true), // ✅ FEATURE 2: Required validation
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .about("Enable verbose output")
                .takes_value(false),
        )
        .subcommand(
            Command::new("process")
                .about("Process data files")
                .arg(
                    Arg::new("input")
                        .index(0)
                        .about("Input file to process")
                        .required(true),
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .about("Output format")
                        .default_value("json"),
                ),
        );

    // ✅ FEATURE 1: Auto-generated help
    // Try running with: cargo run --example 11_core_features_demo -- --help
    // Or: cargo run --example 11_core_features_demo -- process --help

    let matches = app.get_matches();

    // Demonstrate the features
    println!("\n=== Core Features Demo ===\n");

    // FEATURE 3: Default values
    if let Some(config) = matches.value_of("config") {
        println!("✓ Config file: {} (default value)", config);
    }

    // FEATURE 2: Required validation (will error if missing)
    if let Some(output) = matches.value_of("output") {
        println!("✓ Output file: {} (required argument)", output);
    }

    // Optional argument
    if matches.is_present("verbose") {
        println!("✓ Verbose mode enabled");
    }

    // Handle subcommand
    if let Some((name, sub_matches)) = matches.subcommand() {
        println!("\n=== Subcommand: {} ===\n", name);

        if let Some(input) = sub_matches.value_of("input") {
            println!("✓ Processing input: {}", input);
        }

        if let Some(format) = sub_matches.value_of("format") {
            println!("✓ Using format: {} (default value)", format);
        }
    }

    println!("\n=== Feature Verification ===\n");
    println!("✅ Help generation: Try --help or -h");
    println!("✅ Required validation: Try removing --output");
    println!("✅ Default values: Config defaults to 'config.toml'");
    println!("\nAll three features are working! 🎉\n");
}
