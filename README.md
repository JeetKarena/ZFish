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
  <a href="https://github.com/JeetKarena/ZFish#license"><img src="https://img.shields.io/crates/l/zfish.svg" alt="License"/></a>
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

## ✨ Features

- 🚀 **Zero Dependencies** — No third-party crates, only `std`
- 🎨 **Rich Styling** — 16 ANSI colors + 256-color palette
- ⚡ **Blazing Fast** — Cold start <5ms, parse 1M flags in ~200ms
- 🔒 **Memory Safe** — `#![forbid(unsafe_code)]` in public API
- 🌍 **Cross-Platform** — Linux, macOS, Windows (tier-1 support)
- 📦 **Lightweight** — Minimal binary size, fast compile times
- 🎯 **Intuitive API** — Ergonomic design, great docs

---

## 🚀 Quick Start

Add zfish to your `Cargo.toml`:

```toml
[dependencies]
zfish = "0.1"
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

---

## 📚 Documentation

- **API Docs**: [docs.rs/zfish](https://docs.rs/zfish)
- **Roadmap**: [ROADMAP.md](./ROADMAP.md)
- **Examples**: [examples/](./examples/) (coming soon)

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

## 🏗️ Project Status

**Current Version**: `0.1.0` (Foundation Release)

See [ROADMAP.md](./ROADMAP.md) for detailed version plans.

### Completed Features (v0.1.0)
- ✅ 16 standard ANSI colors
- ✅ 256-color palette
- ✅ Text styling (bold, italic, underline, etc.)
- ✅ Basic argument parser
- ✅ Progress bars
- ✅ Interactive prompts
- ✅ Terminal control utilities
- ✅ Leveled logging

### Coming Next (v0.2.0)
- 🔨 Subcommand support
- 🔨 Auto-generated `--help`
- 🔨 Argument validation
- 🔨 Fuzzing tests

---

## 🔧 Feature Flags

zfish uses Cargo feature flags for optional functionality:

```toml
[dependencies.zfish]
version = "0.1"
default-features = false  # Disable all defaults
features = ["colour"]     # Enable only what you need
```

| Flag | Default | Description |
|------|---------|-------------|
| `colour` | ✅ | ANSI color support |
| `raw` | ❌ | Raw terminal mode (coming in 0.3.0) |
| `progress` | ❌ | Progress bars (coming in 0.4.0) |
| `interactive` | ❌ | Interactive prompts (coming in 0.3.0) |

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Report Bugs** — Open an issue with details
2. **Request Features** — Discuss in GitHub Discussions
3. **Submit PRs** — Check open issues labeled "good first issue"
4. **Improve Docs** — Fix typos, add examples

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

# Run examples (coming soon)
cargo run --example basic_colors
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

Dual-licensed under your choice of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in zfish shall be dual-licensed as above, without any additional terms or conditions.

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
- **Twitter** — Follow [@jeetkarena](https://twitter.com/jeetkarena) for updates

---

## 📈 Stargazers over time

[![Stargazers over time](https://starchart.cc/JeetKarena/ZFish.svg)](https://starchart.cc/JeetKarena/ZFish)

---

<div align="center">

**Created with ❤️ by Jeet Karena**

```text
╔═══════════════════════════════════════════════════════════════╗
║  zfish v0.1.0                                                  ║
║  Copyright © 2025 Jeet Karena                                 ║
║  Licensed under MIT OR Apache-2.0                             ║
╚═══════════════════════════════════════════════════════════════╝
```

[⬆ Back to Top](#-zfish--ultra-light-cli-framework-for-rust)

</div>
