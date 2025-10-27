# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-10-26

### Added
- Initial release of zfish CLI framework
- Zero-dependency argument parsing with `Args`
- ANSI color support with 16 standard colors + 256-color palette
- Text styling (bold, italic, underline, dim, blink, reverse, hidden)
- Progress bars with customizable width
- Leveled logging (Error, Warn, Info, Debug)
- Interactive prompts (text input, password, confirmation)
- Terminal control (cursor movement, screen clearing, size detection)
- Cross-platform support (Windows, Linux, macOS)
- Platform-specific terminal handling (Unix/Windows)
- Comprehensive test suite (70+ tests)
- 8 example programs demonstrating all features
- Full API documentation with examples
- MIT license

### Features
- `#![forbid(unsafe_code)]` in public API
- Zero third-party dependencies
- Rust 2024 Edition
- MSRV: 1.90.0

[Unreleased]: https://github.com/jeetkarena/zfish/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jeetkarena/zfish/releases/tag/v0.1.0
