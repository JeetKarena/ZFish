# Contributing to ZFish 🦈

Thank you for your interest in contributing to **ZFish**! We welcome contributions of all kinds: bug reports, feature requests, documentation improvements, and code contributions.

---

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Release Process](#release-process)

---

## 📜 Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

---

## 🤝 How Can I Contribute?

### Reporting Bugs 🐛

Before creating bug reports, please check the [existing issues](https://github.com/JeetKarena/ZFish/issues) to avoid duplicates.

**When filing a bug report, include:**

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Your environment (OS, Rust version)
- Code samples if applicable
- Any relevant error messages

**Example:**

```markdown
**Bug**: Progress bar rendering incorrect on Windows Terminal

**Environment:**
- OS: Windows 11
- Rust: 1.90.0
- Terminal: Windows Terminal 1.20

**Steps to Reproduce:**
1. Run `cargo run --example 04_progress_bar`
2. Observe output

**Expected:** Progress bar shows smooth animation
**Actual:** Characters appear corrupted
```

### Suggesting Features 💡

We love feature suggestions! Please:

1. Check [existing issues](https://github.com/JeetKarena/ZFish/issues) first
2. Create a new issue with the `enhancement` label
3. Describe the feature and its use case
4. Explain why this feature would be useful

### Contributing Code 💻

1. **Fork the repository**
2. **Create a feature branch** from `main`
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. **Make your changes** following our [coding standards](#coding-standards)
4. **Write tests** for your changes
5. **Run the test suite** to ensure everything passes
6. **Commit your changes** using [conventional commits](#commit-message-guidelines)
7. **Push to your fork** and create a pull request

---

## 🛠️ Development Setup

### Prerequisites

- **Rust 1.90.0 or later** (MSRV - Minimum Supported Rust Version)
- Git

### Setup Steps

1. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/ZFish.git
   cd ZFish
   ```

2. **Add upstream remote:**
   ```bash
   git remote add upstream https://github.com/JeetKarena/ZFish.git
   ```

3. **Build the project:**
   ```bash
   cargo build
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

5. **Run examples:**
   ```bash
   cargo run --example 01_hello_world
   cargo run --example 04_progress_bar
   # ... etc
   ```

### Development Commands

```bash
# Build
cargo build

# Run all tests
cargo test

# Run specific test
cargo test test_terminal_size

# Check for lint warnings
cargo clippy --all-targets -- -D warnings

# Format code
cargo fmt --all

# Build documentation
cargo doc --no-deps --all-features --open

# Run examples
cargo run --example <example_name>
```

---

## 📝 Commit Message Guidelines

We use **[Conventional Commits](https://www.conventionalcommits.org/)** for automatic semantic versioning.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

| Type       | Version Bump | Description                             |
| ---------- | ------------ | --------------------------------------- |
| `feat:`    | MINOR        | A new feature                           |
| `fix:`     | PATCH        | A bug fix                               |
| `docs:`    | PATCH        | Documentation only changes              |
| `style:`   | PATCH        | Code style changes (formatting, etc.)   |
| `refactor:`| PATCH        | Code refactoring                        |
| `perf:`    | PATCH        | Performance improvements                |
| `test:`    | PATCH        | Adding or updating tests                |
| `build:`   | PATCH        | Build system changes                    |
| `ci:`      | PATCH        | CI configuration changes                |
| `chore:`   | PATCH        | Other changes (dependencies, etc.)      |
| `feat!:`   | MAJOR        | Breaking change (new feature)           |
| `fix!:`    | MAJOR        | Breaking change (bug fix)               |

### Examples

```bash
# New feature (bumps 0.1.0 → 0.2.0)
feat: add table rendering support

# Bug fix (bumps 0.1.0 → 0.1.1)
fix: correct terminal size detection on Windows

# Breaking change (bumps 0.1.0 → 1.0.0)
feat!: redesign Progress API to use builder pattern

BREAKING CHANGE: Progress::new() now requires builder pattern.
Use Progress::builder().total(100).build() instead.

# Documentation update
docs: add examples for color themes

# Multiple changes
feat: add spinner animation styles

- Implement 8 new spinner styles
- Add animation speed control
- Update documentation with examples
```

### Special Markers

- **`[skip ci]`** - Skip CI pipeline
- **`[no release]`** - Don't trigger automatic release

```bash
git commit -m "docs: fix typo in readme [skip ci]"
```

---

## 🔀 Pull Request Process

### Before Submitting

1. ✅ Update documentation if needed
2. ✅ Add tests for new features
3. ✅ Run `cargo test` - all tests must pass
4. ✅ Run `cargo clippy --all-targets -- -D warnings` - no warnings
5. ✅ Run `cargo fmt --all` - code must be formatted
6. ✅ Update CHANGELOG.md if significant change
7. ✅ Ensure your branch is up to date with `main`

### Pull Request Template

```markdown
## Description
Brief description of what this PR does

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] All tests pass locally
- [ ] Added tests for new features
- [ ] Tested on: [OS/Platform]

## Checklist
- [ ] My code follows the project's coding standards
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
```

### Review Process

1. A maintainer will review your PR within a few days
2. Address any requested changes
3. Once approved, a maintainer will merge your PR
4. Your contribution will be included in the next release!

---

## 📐 Coding Standards

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Use `cargo clippy` to catch common mistakes
- Write idiomatic Rust code

### Code Quality

```rust
// ✅ Good - clear, documented, tested
/// Renders a progress bar to the terminal
///
/// # Arguments
/// * `current` - Current progress value
/// * `total` - Total progress value
///
/// # Examples
/// ```
/// use zfish::ProgressBar;
/// let pb = ProgressBar::new(100);
/// pb.render(50);
/// ```
pub fn render(&self, current: usize, total: usize) {
    // Implementation
}

// ❌ Bad - unclear, undocumented
pub fn do_thing(&self, a: usize, b: usize) {
    // ...
}
```

### Project-Specific Rules

- **Zero dependencies** - ZFish uses only the Rust standard library
- **Cross-platform** - Code must work on Windows, macOS, and Linux
- **Performance** - Avoid unnecessary allocations in hot paths
- **Safety** - Minimize use of `unsafe`, document when necessary
- **Compatibility** - Support MSRV (Rust 1.90.0)

### Module Organization

```
src/
├── lib.rs          // Public API and re-exports
├── args.rs         // Argument parsing
├── log.rs          // Logging functionality
├── progress.rs     // Progress bars
├── prompt.rs       // Interactive prompts
├── style.rs        // Color and styling
├── term.rs         // Terminal control
└── os/             // Platform-specific code
    ├── mod.rs
    ├── unix.rs
    └── windows.rs
```

---

## 🧪 Testing Guidelines

### Test Coverage

- Write unit tests for all new functions
- Write integration tests for new features
- Test edge cases and error conditions
- Test on multiple platforms when possible

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_creation() {
        let pb = ProgressBar::new(100);
        assert_eq!(pb.total, 100);
    }

    #[test]
    fn test_progress_bar_rendering() {
        let pb = ProgressBar::new(100);
        let output = pb.render(50);
        assert!(output.contains("50%"));
    }

    #[test]
    #[should_panic(expected = "total must be greater than 0")]
    fn test_invalid_progress_bar() {
        ProgressBar::new(0);
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_progress_bar

# Run tests with output
cargo test -- --nocapture

# Run tests on a specific platform
cargo test --target x86_64-pc-windows-msvc

# Run ignored tests
cargo test -- --ignored
```

---

## 📚 Documentation

### Code Documentation

- Add doc comments (`///`) to all public items
- Include examples in doc comments
- Document edge cases and panics
- Use `cargo doc` to preview documentation

### Examples

```rust
/// Creates a new progress bar with the specified total
///
/// # Arguments
/// * `total` - The maximum value for the progress bar
///
/// # Panics
/// Panics if `total` is 0
///
/// # Examples
/// ```
/// use zfish::ProgressBar;
///
/// let pb = ProgressBar::new(100);
/// pb.set(50);  // 50%
/// ```
pub fn new(total: usize) -> Self {
    assert!(total > 0, "total must be greater than 0");
    Self { total, current: 0 }
}
```

### Documentation Files

When updating documentation:

- **README.md** - Project overview, quick start, features
- **CHANGELOG.md** - Version history and changes
- **Examples** - Working code examples in `examples/`
- **Rustdoc** - API documentation in code

---

## 🚀 Release Process

### Automatic Releases

ZFish uses **automatic semantic versioning**. When you push to `main`, the system automatically:

1. Analyzes your commit message
2. Determines version bump (major/minor/patch)
3. Updates `Cargo.toml`
4. Creates a git tag
5. Publishes to crates.io
6. Creates GitHub release

See [`.github/RELEASE_GUIDE.md`](.github/RELEASE_GUIDE.md) for details.

### Version Bumping

Based on your commit type:

- `feat:` → Minor version (0.1.0 → 0.2.0)
- `fix:` → Patch version (0.1.0 → 0.1.1)
- `feat!:` → Major version (0.1.0 → 1.0.0)

**You don't need to manually update versions!** Just use proper commit messages.

---

## 🎯 First-Time Contributors

Looking for something to work on? Check out issues labeled:

- [`good first issue`](https://github.com/JeetKarena/ZFish/labels/good%20first%20issue)
- [`help wanted`](https://github.com/JeetKarena/ZFish/labels/help%20wanted)
- [`documentation`](https://github.com/JeetKarena/ZFish/labels/documentation)

### Getting Help

- 💬 Open a [discussion](https://github.com/JeetKarena/ZFish/discussions)
- 🐛 File an [issue](https://github.com/JeetKarena/ZFish/issues)
- 📧 Contact maintainers (see CODE_OF_CONDUCT.md)

---

## 📜 License

By contributing to ZFish, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

## 🙏 Thank You!

Your contributions make ZFish better for everyone. We appreciate your time and effort! 🦈✨

**Happy Contributing!** 🎉
