# 🪁 Kite Development Roadmap

> **Ultra-Light, Zero-Dependency CLI Framework for Rust**

```text
╭─╮
│ ╰─╮  Soar above the complexity
╰─╯
```

## 📍 Where We Are Now

**Current Version**: `0.1.0` — Foundational Release  
**Status**: ✅ Milestone 1 Complete  
**Next Target**: Version 0.2.0 (Argument Parser Enhancement)

---

## 🎯 Version Milestones

### ✅ Version 0.1.0 — Foundation (COMPLETED)
**Status**: Released  
**Theme**: Cross-Platform Terminal Basics

**Completed Features**:
- ✅ Zero-dependency architecture
- ✅ Basic terminal styling (16 ANSI colors)
- ✅ 256-color palette support (`Color::Custom(0..=255)`)
- ✅ Text styles (bold, italic, underline, dim, blink, reverse, hidden)
- ✅ Simple argument parser (flags, options, positional args)
- ✅ Progress bars with throughput display
- ✅ Interactive prompts (text input, password, confirmation)
- ✅ Leveled logging system
- ✅ Terminal control (clear screen, cursor movement, size detection)
- ✅ Windows PowerShell compatibility
- ✅ Automatic color detection (`NO_COLOR`, `COLORTERM`)

**Exit Criteria**:
- ✅ Compiles on tier-1 platforms (Linux, macOS, Windows)
- ✅ All tests passing
- ✅ Zero third-party dependencies
- ✅ Basic documentation

---

### 🚧 Version 0.2.0 — Advanced Argument Parsing (IN PROGRESS)
**Target**: Q2 2025  
**Theme**: Production-Ready CLI Parser

**Planned Features**:
- 🔨 Subcommand support (e.g., `git commit`, `cargo build`)
- 🔨 Auto-generated `--help` / `-h` output
- 🔨 Argument validation and error messages
- 🔨 Required vs optional arguments
- 🔨 Default values for options
- 🔨 Short and long flag aliases (`-v` / `--verbose`)
- 🔨 Environment variable fallbacks
- 🔨 Fuzzing tests for parser robustness
- 🔨 Custom parsing rules and hooks

**Performance Target**:
- Parse 1 million flags in ≤200ms (Ryzen 3600)

**Exit Criteria**:
- Subcommands work with nested options
- `--help` auto-generation with examples
- Fuzz testing passes 100k iterations
- Comprehensive API documentation

---

### 📋 Version 0.3.0 — Interactive Excellence
**Target**: Q3 2025  
**Theme**: Advanced User Interaction

**Planned Features**:
- Raw terminal mode (cross-platform)
- Real-time key event handling (`read_key`)
- Enhanced prompts (with validation callbacks)
- Spinner animations (multiple styles)
- Ctrl-C / SIGINT hygiene (graceful shutdown)
- Multi-line text input
- Password input with visual feedback options
- Select menus (arrow key navigation)

**Performance Target**:
- Spinner refresh at 60 FPS, zero allocations per frame

---

### 📋 Version 0.4.0 — Progress & Logging
**Target**: Q4 2025  
**Theme**: Visual Feedback Systems

**Planned Features**:
- Multi-progress bar support (parallel operations)
- Throughput calculation and ETA
- Log level filtering (`-v`, `-vv`, `-vvv`)
- JSON-lines output mode
- Custom log formatters
- Progress bar templates
- File logging support
- Colored log output

---

### 📋 Version 0.5.0 — Configuration & Completion
**Target**: Q1 2026  
**Theme**: User Experience Polish

**Planned Features**:
- TOML parser (self-written, zero-dependency)
- Configuration file loading (`~/.config/app/config.toml`)
- Environment variable integration
- Shell completion generation (Bash, Zsh, Fish, PowerShell)
- Man page generation
- Config validation and schema
- Merge strategies (CLI > env > config file)

---

### 📋 Version 0.6.0 — Advanced UX
**Target**: Q2 2026  
**Theme**: Premium CLI Features

**Planned Features**:
- Fuzzy search prompt (interactive filtering)
- Multi-select menus (checkboxes)
- Syntax-highlighted pager
- Hyperlink support (clickable URLs in terminals)
- Table formatting
- Tree views
- Rich error messages with suggestions

---

### 📋 Version 0.7.0 — Plugin System
**Target**: Q3 2026  
**Theme**: Extensibility

**Planned Features**:
- Dynamic subcommand discovery
- WASM plugin runtime (lightweight, no Cranelift)
- Sandboxed plugin API
- Plugin registry and loading
- Hot-reload support (development mode)
- Plugin security model

---

### 📋 Version 0.8.0 — Self-Update & Telemetry
**Target**: Q4 2026  
**Theme**: Distribution & Maintenance

**Planned Features**:
- Verified binary updates (using Minisign)
- Crates.io version checking
- Self-update command (`app update`)
- Opt-in telemetry (privacy-first, rustls-ffi only)
- Update channels (stable / beta / nightly)
- Rollback support

---

### 📋 Version 0.9.0 — Release Candidate
**Target**: Q1 2027  
**Theme**: Stabilization

**Focus Areas**:
- API freeze (semver commitment)
- Full Miri compliance (memory safety)
- `cargo deny` clean (security audit)
- mdBook user guide (comprehensive docs)
- Migration guide from other CLI frameworks
- Performance benchmarks published
- Real-world case studies

**Exit Criteria**:
- Zero known critical bugs
- 100% API documentation coverage
- All examples compile with `--no-default-features`
- 5-year MSRV policy defined

---

### 🎉 Version 1.0.0 — Stable Release
**Target**: Q2 2027  
**Theme**: Production Ready

**Promises**:
- Semantic versioning guarantee
- Long-term support (LTS) branch
- 5-year MSRV policy
- Deprecation cycle: 3 minor versions before removal
- Stable API surface
- Official announcement posts (blog, Reddit, HN)

---

## 🏗️ Current Development Focus

### Active Work (Version 0.1.x → 0.2.0)
1. **Fix Parallel Test Execution Issues**
   - Environment variable isolation in tests
   - Thread-safe color detection

2. **Documentation Improvements**
   - Add doctests for all public APIs
   - Create example programs
   - Write contributor guide

3. **Code Organization**
   - Restructure into subdirectories (`parse/`, `term/`, `prompt/`)
   - Add proper module documentation
   - Enforce `#![forbid(unsafe_code)]` at crate root

4. **Subcommand Parser (0.2.0 Preview)**
   - Design subcommand API
   - Implement basic routing
   - Add help text generation

---

## 📊 Feature Flags

| Flag | Default | Version | Description |
|------|---------|---------|-------------|
| `colour` | ✅ Yes | 0.1.0 | ANSI color support |
| `raw` | ❌ No | 0.3.0 | Raw terminal mode |
| `progress` | ❌ No | 0.4.0 | Progress bars (requires `raw`) |
| `interactive` | ❌ No | 0.3.0 | Interactive prompts (requires `raw`) |
| `plugin` | ❌ No | 0.7.0 | WASM plugin system |
| `self-update` | ❌ No | 0.8.0 | Binary self-update |

---

## 🎯 Design Principles

1. **Zero Dependencies**: Only `std`, `core`, `alloc` allowed in `src/`
2. **Safety First**: `#![forbid(unsafe_code)]` except in isolated platform modules
3. **Performance**: Cold start ≤5ms, parse 1M flags ≤200ms
4. **Cross-Platform**: Tier-1 support (Linux, macOS, Windows)
5. **User-Friendly**: Intuitive APIs, great error messages
6. **Stable**: Semver commitment, deprecation cycles

---

## 🤝 Contributing

Want to help? Check out:
- Open issues labeled with version milestones
- `CONTRIBUTING.md` (coming in 0.2.0)
- Discussion board for design proposals

---

## 📜 License

Dual-licensed under **MIT OR Apache-2.0** (your choice).

---

## 💬 Questions?

- GitHub Issues: Report bugs or request features
- Discussions: Ask questions or share ideas

---

**Created with ❤️ by Jeet Karena**

```text
╔═══════════════════════════════════════════════════════════════╗
║  Kite — Zero-Dependency CLI Framework                         ║
║  Copyright © 2025 Jeet Karena                                 ║
╚═══════════════════════════════════════════════════════════════╝
```
