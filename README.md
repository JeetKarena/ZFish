# 🐟 zfish — Ultra-Light CLI Framework for Rust

<p align="center">
  <img src="Logo.svg" alt="zfish Logo" width="200"/>
</p>

<p align="center">
  <strong>Soar above the complexity</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/zfish"><img src="https://img.shields.io/crates/v/zfish.svg" alt="Crates.io"/></a>
  <a href="https://docs.rs/zfish"><img src="https://docs.rs/zfish/badge.svg" alt="Documentation"/></a>
  <a href="https://zfish-devdocs.vercel.app"><img src="https://img.shields.io/badge/dev%20docs-vercel-black.svg" alt="Developer Docs"/></a>
  <a href="https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232"><img src="https://img.shields.io/badge/roadmap-notion-000000.svg" alt="Roadmap"/></a>
  <a href="https://github.com/JeetKarena/ZFish/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"/></a>
  <a href="https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html"><img src="https://img.shields.io/badge/MSRV-1.90-blue.svg" alt="MSRV"/></a>
  <a href="https://github.com/JeetKarena/ZFish/actions"><img src="https://github.com/JeetKarena/ZFish/workflows/CI/badge.svg" alt="CI Status"/></a>
</p>

<p align="center">
  <a href="https://sonarcloud.io/dashboard?id=JeetKarena_ZFish"><img src="https://sonarcloud.io/api/project_badges/measure?project=JeetKarena_ZFish&metric=alert_status" alt="Quality Gate"/></a>
  <a href="https://sonarcloud.io/dashboard?id=JeetKarena_ZFish"><img src="https://sonarcloud.io/api/project_badges/measure?project=JeetKarena_ZFish&metric=security_rating" alt="Security Rating"/></a>
  <a href="https://sonarcloud.io/dashboard?id=JeetKarena_ZFish"><img src="https://sonarcloud.io/api/project_badges/measure?project=JeetKarena_ZFish&metric=sqale_rating" alt="Maintainability Rating"/></a>
  <a href="https://sonarcloud.io/dashboard?id=JeetKarena_ZFish"><img src="https://sonarcloud.io/api/project_badges/measure?project=JeetKarena_ZFish&metric=coverage" alt="Coverage"/></a>
  <a href="https://sonarcloud.io/dashboard?id=JeetKarena_ZFish"><img src="https://sonarcloud.io/api/project_badges/measure?project=JeetKarena_ZFish&metric=bugs" alt="Bugs"/></a>
</p>

---

> 📋 **[View the full roadmap and feature status on Notion →](https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232)**

---

## ✨ Features

- 🚀 **Zero Dependencies** — No third-party crates, only `std`
- 🎨 **Rich Styling** — 16 ANSI colors + 256-color palette
- ⚡ **Blazing Fast** — Cold start <5ms, parse 1M flags in ~200ms
- 🔒 **Memory Safe** — `#![forbid(unsafe_code)]` in public API
- 🌍 **Cross-Platform** — Linux, macOS, Windows (tier-1 support)
- 📦 **Lightweight** — Minimal binary size, fast compile times
- 🎯 **Intuitive API** — Ergonomic design, great docs
- 🔮 **Edition 2024** — Built with the latest Rust features

### 🎁 What's Included

| Component | Description | Examples |
|-----------|-------------|----------|
| **Colors** | 16 ANSI + 256-color palette, 8 text styles | `Color::Red.paint("text")` |
| **Args** | Flag parsing, options, positional arguments | `--verbose`, `-abc`, `file.txt` |
| **Commands** | Git-style subcommands with auto-help | `app init`, `app build --release` |
| **Progress** | 4 bar styles (bar, spinner, dots, arrows) | Loading, downloading, processing |
| **Tables** | 5 box styles, alignment, Unicode-aware | Data display, reports, grids |
| **Prompts** | Text input, password, confirm dialogs | Interactive CLIs, wizards |
| **Logging** | 5 levels (error → trace), timestamps | Debug output, application logs |
| **Terminal** | Clear, cursor control, size detection | TUI helpers, screen management |

> 📖 **See the [Feature Matrix](#-feature-matrix) below for detailed capabilities**

---

## 🚀 Quick Start

### Installation

Add zfish to your `Cargo.toml`:

```toml
[dependencies]
zfish = "0.1"
```

**Alternative: Install from GitHub Packages**

Download the `.crate` file from [GitHub Releases](https://github.com/JeetKarena/ZFish/releases):

```bash
# Download the latest release
wget https://github.com/JeetKarena/ZFish/releases/download/v0.1.10/zfish-0.1.10.crate

# Verify checksum (optional)
wget https://github.com/JeetKarena/ZFish/releases/download/v0.1.10/zfish-0.1.10.crate.sha256
sha256sum -c zfish-0.1.10.crate.sha256

# Install from local crate
cargo install --path zfish-0.1.10.crate
```

### Hello, Colorful World!

```rust
use zfish::Color;

fn main() {
    println!("{}", Color::Green.paint("✓ Success!"));
    println!("{}", Color::Red.paint("✗ Error!"));
    println!("{}", Color::Yellow.paint("⚠ Warning!"));
}
```

### Argument Parsing

```rust
use zfish::Args;

fn main() {
    let args = Args::parse();
    
    if args.has_flag("verbose") || args.has_flag("v") {
        println!("Verbose mode enabled");
    }
    
    if let Some(file) = args.get_option("file") {
        println!("Processing: {}", file);
    }
    
    for arg in &args.positional {
        println!("Positional argument: {}", arg);
    }
}
```

### Progress Bar

```rust
use zfish::ProgressBar;
use std::thread;
use std::time::Duration;

fn main() {
    let mut pb = ProgressBar::new(100);
    
    for i in 0..=100 {
        pb.set(i);
        thread::sleep(Duration::from_millis(50));
    }
    
    pb.finish("✓ Complete!");
}
```

### Subcommands (Git-Style CLI)

```rust
use zfish::command::{App, Command, Arg};

fn main() {
    let app = App::new("myapp")
        .version("1.0.0")
        .about("My awesome CLI")
        .arg(Arg::new("verbose").short('v').long("verbose"))
        .subcommand(
            Command::new("init")
                .about("Initialize a new project")
                .arg(Arg::new("name").required(true))
        )
        .subcommand(
            Command::new("build")
                .about("Build the project")
                .arg(Arg::new("release").long("release"))
        );

    let matches = app.get_matches();
    
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            println!("Initializing: {}", name);
        }
        Some(("build", sub_matches)) => {
            if sub_matches.is_present("release") {
                println!("Building in release mode");
            }
        }
        _ => println!("Use --help for usage"),
    }
}
```

### Interactive Prompts

```rust
use zfish::Prompt;

fn main() {
    let name = Prompt::text("What's your name?");
    println!("Hello, {}!", name);
    
    if Prompt::confirm("Continue?") {
        println!("Let's go!");
    }
    
    let password = Prompt::password("Enter password:");
    println!("Password length: {}", password.len());
}
```

### 256-Color Palette

```rust
use zfish::Color;

fn main() {
    // Use any color from 0-255
    println!("{}", Color::Custom(196).paint("Bright red"));
    println!("{}", Color::Custom(46).paint("Bright green"));
    println!("{}", Color::Custom(21).paint("Deep blue"));
    
    // Show all 256 colors
    for i in 0..=255 {
        print!("{} ", Color::Custom(i).paint(format!("{:3}", i)));
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
}
```

### Beautiful Tables

```rust
use zfish::table::{Table, BoxStyle, Alignment};
use zfish::Color;

fn main() {
    let mut table = Table::new(vec!["Name", "Language", "Stars"]);
    table.set_box_style(BoxStyle::Rounded);
    table.set_column_alignment(2, Alignment::Right);
    
    table.add_row(vec!["zfish", "Rust", "⭐⭐⭐⭐⭐"]);
    table.add_row(vec!["clap", "Rust", "⭐⭐⭐⭐"]);
    table.add_row(vec!["cobra", "Go", "⭐⭐⭐"]);
    
    table.print();
    // Output:
    // ╭──────────┬──────────┬───────╮
    // │ Name     │ Language │ Stars │
    // ├──────────┼──────────┼───────┤
    // │ zfish    │ Rust     │ ⭐⭐⭐⭐⭐ │
    // │ clap     │ Rust     │ ⭐⭐⭐⭐ │
    // │ cobra    │ Go       │ ⭐⭐⭐ │
    // ╰──────────┴──────────┴───────╯
}
```

---

## 📚 Documentation

- **Developer Docs**: [zfish-devdocs.vercel.app](https://zfish-devdocs.vercel.app) — Interactive guides and tutorials
- **API Docs**: [docs.rs/zfish](https://docs.rs/zfish) — Generated Rust documentation
- **Roadmap**: [ROADMAP.md](./ROADMAP.md) — Detailed feature roadmap
- **Public Roadmap**: [Notion Roadmap](https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232) — Live feature status & timeline
- **Installation Guide**: [PACKAGES.md](./PACKAGES.md) — Multiple installation methods
- **Release Process**: [.github/RELEASE_PROCESS.md](./.github/RELEASE_PROCESS.md) — How we create releases
- **Examples**: [examples/](./examples/) — 18 comprehensive examples covering all features
  - `01_hello_world.rs` — Basic usage
  - `02_argument_parsing.rs` — CLI argument handling
  - `03_colored_text.rs` — 16 + 256 color palette
  - `04_progress_bar.rs` — 4 progress bar styles
  - `05_logger.rs` — Leveled logging
  - `06_terminal_control.rs` — Terminal manipulation
  - `07_interactive_prompts.rs` — User input
  - `08_complete_cli.rs` — Full-featured CLI app
  - `09_subcommands.rs` — Nested command structures
  - `10_arg_features_v2.rs` — Advanced argument features
  - `11_core_features_demo.rs` — Core functionality showcase
  - `12_beautiful_reports.rs` — Styled report generation
  - `13_table_examples.rs` — **12 automated table examples** with all box styles
  - `14_alignment_test.rs` — Table emoji alignment verification
  - `15_debug_emoji_width.rs` — Unicode width debugging
  - `16_comprehensive_unicode_test.rs` — Full Unicode support test
  - `17_unicode_edge_cases.rs` — Complex emoji sequences
  - `18_manual_table_drawing.rs` — **Manual table fallback** for custom layouts

---

## 🎯 Design Philosophy

### Why zfish?

Most CLI frameworks pull in dozens of dependencies, increasing:
- **Compile times** (minutes instead of seconds)
- **Binary size** (MBs instead of KBs)
- **Supply chain risk** (trust dozens of crates)
- **Maintenance burden** (breaking changes cascade)

zfish takes a different approach:

```text
┌─────────────────────────────────────────────────────┐
│  Other CLI Frameworks       │  zfish                 │
├─────────────────────────────────────────────────────┤
│  50+ dependencies           │  0 dependencies       │
│  ~10 second compile         │  ~1 second compile    │
│  ~2 MB binary               │  ~200 KB binary       │
│  Complex API                │  Intuitive API        │
│  ANSI escape codes manual   │  Auto color detection │
└─────────────────────────────────────────────────────┘
```

### Core Principles

1. **Zero Dependencies** — Only Rust std library
2. **Safety First** — No unsafe code in public API
3. **Performance** — Optimized for speed and size
4. **Simplicity** — Easy to learn, hard to misuse
5. **Cross-Platform** — Works everywhere Rust works

---

## 🚀 Project Status

**Current Version**: `0.1.10` (Active Development)

> 📋 **For detailed feature status, implementation notes, and roadmap, visit our [Notion Roadmap →](https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232)**

See [ROADMAP.md](./ROADMAP.md) for version plans.

## 📊 Feature Matrix

| Feature Category | Feature | Status | Module | Notes |
|-----------------|---------|--------|--------|-------|
| **Colors & Styling** | 16 ANSI Colors | ✅ | `style` | Black, Red, Green, Yellow, Blue, Magenta, Cyan, White + Bright variants |
| | 256-Color Palette | ✅ | `style` | `Color::Custom(0-255)` |
| | Text Styles | ✅ | `style` | Bold, Italic, Underline, Dim, Blink, Reverse, Hidden, Strikethrough |
| | Chained Styling | ✅ | `style` | `.style(Style::Bold).style(Style::Italic)` |
| **Argument Parsing** | Flags & Options | ✅ | `args` | `--flag`, `-f`, `--option value` |
| | Positional Args | ✅ | `args` | Automatic capture of non-flag arguments |
| | Short Flag Combos | ✅ | `args` | `-abc` → `-a -b -c` |
| **Commands & Subcommands** | Subcommand System | ✅ | `command` | Git-style nested commands |
| | Auto-generated Help | ✅ | `command` | `--help` for all commands |
| | Argument Validation | ✅ | `command` | Required args, possible values, custom validators |
| | Environment Fallbacks | ✅ | `command` | `.env("VAR_NAME")` for options |
| | Value Delimiters | ✅ | `command` | `--tags rust,cli,tool` |
| | Argument Dependencies | ✅ | `command` | `.requires("other_arg")` |
| | Conflict Detection | ✅ | `command` | `.conflicts_with("other_arg")` |
| | Variadic Arguments | ✅ | `command` | `[FILES]...` capture multiple values |
| | Command Aliases | ✅ | `command` | Multiple names for same command |
| **Progress Bars** | Bar Style | ✅ | `progress` | Classic progress bar |
| | Spinner Style | ✅ | `progress` | Rotating spinner |
| | Dots Style | ✅ | `progress` | Animated dots |
| | Arrow Style | ✅ | `progress` | Moving arrows |
| | Custom Width | ✅ | `progress` | `.width(50)` |
| **Interactive Prompts** | Text Input | ✅ | `prompt` | `Prompt::text("Question?")` |
| | Password Input | ✅ | `prompt` | Hidden input for sensitive data |
| | Confirmation | ✅ | `prompt` | Yes/No prompts |
| | Multi-select | 🔨 | - | Coming in v0.3.0 |
| **Tables** | 5 Box Styles | ✅ | `table` | Single, Double, Heavy, Rounded, ASCII |
| | Column Alignment | ✅ | `table` | Left, Right, Center per column |
| | Unicode-Aware Width | ✅ | `table` | Handles emojis, CJK, combining marks |
| | Header/Footer Separators | ✅ | `table` | Toggle separators on/off |
| | Manual Drawing | ✅ | `table` | `draw_box()`, `draw_separator()` for custom layouts |
| | Custom Indentation | ✅ | `table` | `.set_indent(n)` |
| **Logging** | 5 Log Levels | ✅ | `log` | Error, Warn, Info, Debug, Trace |
| | Timestamp Support | ✅ | `log` | Optional timestamps |
| | Level Filtering | ✅ | `log` | `.level(Level::Debug)` |
| **Terminal Control** | Clear Screen | ✅ | `term` | `Terminal::clear_screen()` |
| | Cursor Movement | ✅ | `term` | `Terminal::move_cursor(row, col)` |
| | Terminal Size | ✅ | `term` | `Terminal::size()` (cross-platform) |
| | Print at Position | ✅ | `term` | `Terminal::print_at(row, col, text)` |
| **Platform Support** | Linux | ✅ | `os` | Tier 1 support |
| | macOS | ✅ | `os` | Tier 1 support |
| | Windows | ✅ | `os` | Tier 1 support (cmd.exe, PowerShell) |
| | BSD | 🟡 | `os` | Should work, not officially tested |
| **Development** | Zero Dependencies | ✅ | - | Only uses Rust `std` library |
| | No Unsafe Code | ✅ | - | `#![forbid(unsafe_code)]` in public API |
| | Edition 2024 | ✅ | - | Uses latest Rust features |
| | Feature Flags | ✅ | - | `colour`, `raw`, `progress`, `interactive` |

**Legend**: ✅ Implemented | 🔨 In Progress | 🟡 Partial/Untested | ❌ Not Available

### Completed Features (v0.1.x - v0.2.x)

#### Core Styling & Colors
- ✅ 16 standard ANSI colors
- ✅ 256-color palette (0-255)
- ✅ Text styling (bold, italic, underline, dim, blink, reverse, hidden, strikethrough)

#### Argument Parsing & Commands
- ✅ Basic argument parser (flags, options, positional args)
- ✅ **Subcommand system** (git-style, multi-level hierarchies)
- ✅ **Auto-generated `--help`** text for all commands
- ✅ **Argument validation** (required args, possible values, custom validators)
- ✅ **Advanced arg features** (environment variable fallbacks, delimiters, dependencies, conflicts)
- ✅ **Variadic arguments** (capture multiple values)

#### Progress & Interactive
- ✅ 4 progress bar styles (bar, spinner, dots, arrows)
- ✅ Interactive prompts (text, confirm, password)

#### Terminal Control
- ✅ Terminal control (clear screen, cursor movement)
- ✅ Terminal size detection (Windows + Unix)

#### Logging & Tables
- ✅ Leveled logging (error, warn, info, debug, trace)
- ✅ **Table module** with 5 box styles (Single, Double, Heavy, Rounded, ASCII)
- ✅ **Unicode-aware** width calculation (emojis, CJK, combining marks)
- ✅ **Manual table drawing** fallback for custom layouts
- ✅ **Column alignment** (Left, Right, Center)

#### Development & Examples
- ✅ Feature flags (colour, raw, progress, interactive)
- ✅ Edition 2024 support
- ✅ 18 comprehensive examples

### Coming Next (v0.3.0+)
- 🔨 Multi-select prompts (checkbox lists)
- 🔨 Fuzzing tests for argument parser
- 🔨 Shell completion generation (bash, zsh, fish)
- 🔨 Configuration file support (TOML, JSON)
- 🔨 Spinner widgets with custom animations

> 💡 **Want more details?** Check our **[Interactive Roadmap on Notion](https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232)** for:
> - Detailed implementation notes for each feature
> - Release timelines and version planning
> - Known issues and workarounds
> - Feature request voting and discussions

---

## 🔧 Feature Flags

zfish uses Cargo feature flags for optional functionality:

```toml
[dependencies.zfish]
version = "0.1"
default-features = false  # Disable all defaults
features = ["colour"]     # Enable only what you need
```

| Flag | Default | Description | Status |
|------|---------|-------------|--------|
| `colour` | ✅ | ANSI color support (16 + 256 colors) | ✅ Available |
| `raw` | ❌ | Raw terminal mode for advanced I/O | ✅ Available |
| `progress` | ❌ | Progress bars (4 styles: bar, spinner, dots, arrows) | ✅ Available |
| `interactive` | ❌ | Interactive prompts (text, password, confirm) | ✅ Available |

**Note**: `progress` and `interactive` features require `raw` mode and are automatically enabled when you use them.

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Report Bugs** — Open an issue with details
2. **Request Features** — Discuss in GitHub Discussions
3. **Submit PRs** — Check open issues labeled "good first issue"
4. **Improve Docs** — Fix typos, add examples

### Documentation

- **[API Documentation](https://docs.rs/zfish)** — Complete API reference on docs.rs
- **[Developer Docs](https://zfish-devdocs.vercel.app)** — Interactive guides, examples, and tutorials
- **[Roadmap](https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232)** — Feature status and upcoming releases

### Development Setup

```bash
# Clone the repository
git clone https://github.com/JeetKarena/ZFish.git
cd zfish

# Run tests
cargo test

# Run tests (single-threaded to avoid env var conflicts)
cargo test -- --test-threads=1

# Build documentation
cargo doc --open

# Run examples
cargo run --example 01_hello_world
cargo run --example 02_argument_parsing
cargo run --example 03_colored_text
cargo run --example 04_progress_bar
cargo run --example 05_logger
cargo run --example 06_terminal_control
cargo run --example 07_interactive_prompts
cargo run --example 08_complete_cli
```

---

## 📊 Performance

zfish is designed for speed:

| Operation | Time (Ryzen 3600) |
|-----------|-------------------|
| Cold start | ~3ms |
| Parse 1M flags | ~180ms |
| Render progress bar | 60 FPS |
| Color detection | <1µs (cached) |

Benchmarks run on:
- CPU: AMD Ryzen 5 3600
- RAM: 16GB DDR4-3200
- OS: Windows 11 / Ubuntu 22.04

---

## 🌐 Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Linux | ✅ Tier 1 | Tested on Ubuntu 20.04+ |
| macOS | ✅ Tier 1 | Tested on macOS 12+ |
| Windows | ✅ Tier 1 | Tested on Windows 10/11 |
| BSD | 🟡 Should work | Not officially tested |
| WASM | ❌ Not supported | Terminal I/O not available |

---

## 📜 License

Licensed under the **MIT License**.

See [LICENSE](LICENSE) for details or visit http://opensource.org/licenses/MIT.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in zfish shall be licensed under the MIT License, without any additional terms or conditions.

---

## 🙏 Acknowledgments

Inspired by:
- **clap** — Excellent API design patterns
- **colored** — Color API inspiration
- **indicatif** — Progress bar concepts
- **dialoguer** — Interactive prompt ideas

Built with zero dependencies as a proof-of-concept that powerful CLIs don't need to pull in half of crates.io.

---

## 💬 Community

- **GitHub Discussions** — Ask questions, share ideas
- **Issues** — Report bugs, request features
- **Twitter** — Follow [@user_0xJeet](https://x.com/user_0xJeet) for updates

---

## 📈 Stargazers over time

[![Stargazers over time](https://starchart.cc/JeetKarena/ZFish.svg)](https://starchart.cc/JeetKarena/ZFish)

---

<div align="center">

**Created with ❤️ by Jeet Karena**

```text
╔═══════════════════════════════════════════════════════════════╗
║  zfish v0.1.10                                                ║
║  Copyright © 2025 Jeet Karena                                 ║
║  Licensed under MIT License                                   ║
╚═══════════════════════════════════════════════════════════════╝
```

[⬆ Back to Top](#-zfish--ultra-light-cli-framework-for-rust)

</div>
