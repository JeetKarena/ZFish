// Copyright (c) 2025 Jeet Karena <karenajeet@proton.me>
// Example: Argument Parsing - CLI arguments

use kite::Args;

fn main() {
    // Parse command-line arguments
    let args = Args::parse();
    
    println!("Command: {}", args.command);
    
    // Check for flags
    if args.has_flag("verbose") || args.has_flag("v") {
        println!("Verbose mode enabled");
    }
    
    if args.has_flag("help") || args.has_flag("h") {
        print_help();
        return;
    }
    
    // Get option values
    if let Some(output) = args.get_option("output").or_else(|| args.get_option("o")) {
        println!("Output file: {}", output);
    }
    
    if let Some(count) = args.get_option("count") {
        println!("Count: {}", count);
    }
    
    // Get positional arguments
    if !args.positional.is_empty() {
        println!("Files: {:?}", args.positional);
    }
}

fn print_help() {
    println!("Usage: {} [OPTIONS] [FILES...]", std::env::args().next().unwrap());
    println!();
    println!("Options:");
    println!("  -h, --help              Show this help message");
    println!("  -v, --verbose           Enable verbose output");
    println!("  -o, --output <FILE>     Output file path");
    println!("  --count <N>             Number of iterations");
}

// Example usage:
// cargo run --example 02_argument_parsing -- --verbose -o output.txt file1.txt file2.txt
// cargo run --example 02_argument_parsing -- --help
