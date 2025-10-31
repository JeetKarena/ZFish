//! Example: Subcommand System
//!
//! This example demonstrates ZFish's powerful subcommand system,
//! showcasing real-world CLI patterns like git, cargo, and docker.
//!
//! Run with:
//!   cargo run --example 09_subcommands -- --help
//!   cargo run --example 09_subcommands -- init my-project
//!   cargo run --example 09_subcommands -- build --release
//!   cargo run --example 09_subcommands -- deploy --env production --verbose

use zfish::Color;
use zfish::command::{App, Arg, Command};

fn main() {
    // Create a CLI application with subcommands (like cargo, git, docker)
    let app = App::new("myapp")
        .version("1.0.0")
        .about("A modern CLI application with subcommands")
        // Global flags that apply to all subcommands
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .about("Enable verbose output")
                .takes_value(false),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .about("Path to config file")
                .default_value("config.toml"),
        )
        // Subcommand: init
        .subcommand(
            Command::new("init")
                .about("Initialize a new project")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .short('n')
                        .about("Project name")
                        .required(true),
                )
                .arg(
                    Arg::new("template")
                        .long("template")
                        .about("Project template to use")
                        .possible_values(&["basic", "advanced", "minimal"])
                        .default_value("basic"),
                ),
        )
        // Subcommand: build
        .subcommand(
            Command::new("build")
                .about("Build the project")
                .arg(
                    Arg::new("release")
                        .long("release")
                        .about("Build in release mode")
                        .takes_value(false),
                )
                .arg(
                    Arg::new("target")
                        .long("target")
                        .about("Target platform")
                        .possible_values(&["x86_64", "arm64", "wasm32"]),
                )
                .arg(
                    Arg::new("jobs")
                        .short('j')
                        .long("jobs")
                        .about("Number of parallel jobs")
                        .default_value("4")
                        .validator(|s| {
                            s.parse::<u32>()
                                .map(|_| ())
                                .map_err(|_| "must be a number".to_string())
                        }),
                ),
        )
        // Subcommand: test
        .subcommand(
            Command::new("test")
                .about("Run tests")
                .arg(
                    Arg::new("filter")
                        .long("filter")
                        .short('f')
                        .about("Test name filter"),
                )
                .arg(
                    Arg::new("nocapture")
                        .long("nocapture")
                        .about("Don't capture test output")
                        .takes_value(false),
                ),
        )
        // Subcommand: deploy
        .subcommand(
            Command::new("deploy")
                .about("Deploy the application")
                .arg(
                    Arg::new("environment")
                        .short('e')
                        .long("env")
                        .about("Deployment environment")
                        .possible_values(&["development", "staging", "production"])
                        .required(true),
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .about("Simulate deployment without making changes")
                        .takes_value(false),
                ),
        )
        // Subcommand: clean
        .subcommand(
            Command::new("clean").about("Clean build artifacts").arg(
                Arg::new("all")
                    .short('a')
                    .long("all")
                    .about("Clean all caches including dependencies")
                    .takes_value(false),
            ),
        );

    // Parse arguments
    let matches = app.get_matches();

    // Handle global flags
    let verbose = matches.is_flag_set("verbose");
    let config = matches.value_of("config").unwrap();

    if verbose {
        println!(
            "{}",
            Color::Cyan.paint(format!("→ Using config: {}", config))
        );
        println!("{}", Color::Cyan.paint("→ Verbose mode enabled"));
    }

    // Handle subcommands
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            let template = sub_matches.value_of("template").unwrap();

            println!("{}", Color::Green.paint("✓ Initializing new project"));
            println!("  Project name: {}", Color::Yellow.paint(name));
            println!("  Template: {}", Color::Yellow.paint(template));

            if verbose {
                println!(
                    "\n{}",
                    Color::Cyan.paint("→ Creating directory structure...")
                );
                println!("{}", Color::Cyan.paint("→ Writing configuration files..."));
                println!("{}", Color::Cyan.paint("→ Installing dependencies..."));
            }

            println!(
                "\n{}",
                Color::Green.paint(format!("✓ Project '{}' created successfully!", name))
            );
        }

        Some(("build", sub_matches)) => {
            let release = sub_matches.is_flag_set("release");
            let target = sub_matches.value_of("target");
            let jobs = sub_matches.value_of("jobs").unwrap();

            println!("{}", Color::Green.paint("✓ Building project"));

            if release {
                println!("  Mode: {}", Color::Yellow.paint("Release (optimized)"));
            } else {
                println!("  Mode: {}", Color::Yellow.paint("Debug"));
            }

            if let Some(t) = target {
                println!("  Target: {}", Color::Yellow.paint(t));
            }

            println!("  Parallel jobs: {}", Color::Yellow.paint(jobs));

            if verbose {
                println!("\n{}", Color::Cyan.paint("→ Compiling dependencies..."));
                println!("{}", Color::Cyan.paint("→ Compiling project..."));
                println!("{}", Color::Cyan.paint("→ Linking..."));
            }

            println!(
                "\n{}",
                Color::Green.paint("✓ Build completed successfully!")
            );
        }

        Some(("test", sub_matches)) => {
            let filter = sub_matches.value_of("filter");
            let nocapture = sub_matches.is_flag_set("nocapture");

            println!("{}", Color::Green.paint("✓ Running tests"));

            if let Some(f) = filter {
                println!("  Filter: {}", Color::Yellow.paint(f));
            }

            if nocapture {
                println!("  Output capture: {}", Color::Yellow.paint("disabled"));
            }

            if verbose {
                println!("\n{}", Color::Cyan.paint("→ Running 42 tests..."));
                println!("{}", Color::Green.paint("  ✓ test_basic ... ok"));
                println!("{}", Color::Green.paint("  ✓ test_advanced ... ok"));
                println!("{}", Color::Green.paint("  ✓ test_edge_cases ... ok"));
            }

            println!("\n{}", Color::Green.paint("✓ All tests passed!"));
        }

        Some(("deploy", sub_matches)) => {
            let env = sub_matches.value_of("environment").unwrap();
            let dry_run = sub_matches.is_flag_set("dry-run");

            if dry_run {
                println!(
                    "{}",
                    Color::Yellow.paint("⚠ DRY RUN - No actual changes will be made")
                );
            }

            println!("{}", Color::Green.paint("✓ Deploying application"));
            println!("  Environment: {}", Color::Yellow.paint(env));

            if env == "production" {
                println!(
                    "\n{}",
                    Color::Red.paint("⚠ WARNING: Deploying to PRODUCTION!")
                );
            }

            if verbose {
                println!("\n{}", Color::Cyan.paint("→ Building container image..."));
                println!("{}", Color::Cyan.paint("→ Pushing to registry..."));
                println!("{}", Color::Cyan.paint("→ Updating service..."));
                println!("{}", Color::Cyan.paint("→ Running health checks..."));
            }

            if dry_run {
                println!(
                    "\n{}",
                    Color::Yellow.paint("✓ Dry run completed - no changes made")
                );
            } else {
                println!(
                    "\n{}",
                    Color::Green.paint(format!("✓ Successfully deployed to {}!", env))
                );
            }
        }

        Some(("clean", sub_matches)) => {
            let all = sub_matches.is_flag_set("all");

            println!("{}", Color::Green.paint("✓ Cleaning build artifacts"));

            if all {
                println!(
                    "  Scope: {}",
                    Color::Yellow.paint("All (including dependencies)")
                );
            } else {
                println!("  Scope: {}", Color::Yellow.paint("Build artifacts only"));
            }

            if verbose {
                println!("\n{}", Color::Cyan.paint("→ Removing target/ directory..."));
                if all {
                    println!("{}", Color::Cyan.paint("→ Clearing cargo cache..."));
                    println!("{}", Color::Cyan.paint("→ Removing node_modules/..."));
                }
            }

            println!("\n{}", Color::Green.paint("✓ Cleanup completed!"));
        }

        _ => {
            println!("{}", Color::Yellow.paint("No subcommand provided"));
            println!("\nAvailable commands:");
            println!(
                "  {} - Initialize a new project",
                Color::Green.paint("init")
            );
            println!("  {} - Build the project", Color::Green.paint("build"));
            println!("  {} - Run tests", Color::Green.paint("test"));
            println!(
                "  {} - Deploy the application",
                Color::Green.paint("deploy")
            );
            println!("  {} - Clean build artifacts", Color::Green.paint("clean"));
            println!("\nRun 'myapp --help' for more information");
        }
    }
}
