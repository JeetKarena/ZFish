//! # Argument Features v0.2.1 Example
//!
//! Demonstrates all v0.2.1 advanced argument parsing features:
//! - Positional arguments (`<FILE>`, `[OUTPUT]`)
//! - Variadic positional arguments (`[FILES]...`)
//! - Environment variable fallbacks (`.env("VAR_NAME")`)
//! - Argument dependencies (`.requires("other")`)
//! - Argument conflicts (`.conflicts_with("other")`)
//! - Value delimiters (`.value_delimiter(',')`)
//! - Command aliases (`.alias("short")`)
//!
//! Run with `--help` to see all options.

use zfish::Color;
use zfish::command::{App, Arg, Command};

fn main() {
    let app = App::new("myapp")
        .version("2.0.0")
        .about("Advanced CLI with v0.2.1 features")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .about("Configuration file path")
                .env("MYAPP_CONFIG") // Falls back to MYAPP_CONFIG environment variable
                .default_value("config.toml"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .about("Enable verbose output")
                .takes_value(false)
                .conflicts_with("quiet"), // Cannot be used with --quiet
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .about("Suppress output")
                .takes_value(false)
                .conflicts_with("verbose"), // Cannot be used with --verbose
        )
        .subcommand(
            Command::new("process")
                .alias("proc") // Alias: can use "proc" instead of "process"
                .alias("p")
                .about("Process files with advanced options")
                .arg(
                    Arg::new("input")
                        .index(0) // Positional argument at position 0
                        .about("Input file to process")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .about("Output file")
                        .requires("format"), // Requires --format to be specified
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .about("Output format")
                        .possible_values(&["json", "xml", "yaml"]),
                )
                .arg(
                    Arg::new("tags")
                        .short('t')
                        .long("tags")
                        .about("Comma-separated tags")
                        .value_delimiter(','), // Parses "rust,cli,tool" into ["rust", "cli", "tool"]
                ),
        )
        .subcommand(
            Command::new("convert")
                .alias("cv")
                .about("Convert files with multiple inputs")
                .arg(
                    Arg::new("files")
                        .index(0)
                        .last(true) // Variadic positional: captures all remaining args
                        .about("Files to convert")
                        .required(true),
                )
                .arg(
                    Arg::new("target")
                        .long("target")
                        .about("Target format")
                        .required(true)
                        .possible_values(&["pdf", "png", "svg"]),
                ),
        );

    let matches = app.get_matches();

    // Check config (from CLI, env var, or default)
    if let Some(config) = matches.value_of("config") {
        println!("→ Using config: {}", Color::Cyan.paint(config));
    }

    // Check verbose/quiet
    if matches.is_present("verbose") {
        println!("→ Verbose mode enabled");
    } else if matches.is_present("quiet") {
        println!("→ Quiet mode enabled");
    }

    // Handle subcommands
    match matches.subcommand() {
        Some(("process", sub_matches)) | Some(("proc", sub_matches)) | Some(("p", sub_matches)) => {
            println!("\n{} Processing file", Color::Green.paint("✓"));

            if let Some(input) = sub_matches.value_of("input") {
                println!("  Input: {}", Color::Yellow.paint(input));
            }

            if let Some(output) = sub_matches.value_of("output") {
                println!("  Output: {}", Color::Yellow.paint(output));

                if let Some(format) = sub_matches.value_of("format") {
                    println!("  Format: {}", Color::Yellow.paint(format));
                }
            }

            if let Some(tags) = sub_matches.values_of("tags") {
                println!("  Tags: {}", tags.join(", "));
            }
        }
        Some(("convert", sub_matches)) | Some(("cv", sub_matches)) => {
            println!("\n{} Converting files", Color::Green.paint("✓"));

            if let Some(files) = sub_matches.values_of("files") {
                println!("  Files: {}", files.join(", "));
            }

            if let Some(target) = sub_matches.value_of("target") {
                println!("  Target format: {}", Color::Yellow.paint(target));
            }
        }
        _ => {
            println!("\n{} No subcommand specified", Color::Yellow.paint("⚠"));
            println!("Run with --help for usage information");
        }
    }
}
