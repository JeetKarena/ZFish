# README Update Checklist

This checklist ensures the README.md stays in sync with the actual library implementation.

## When to Update README.md

Update the README whenever you:
- ✅ Implement a new feature
- ✅ Add a new module or API
- ✅ Change version numbers
- ✅ Add new examples
- ✅ Update documentation URLs
- ✅ Change license information

## Sections to Keep Updated

### 1. **Badges & Links** (Lines 12-18)
- [ ] crates.io version badge
- [ ] docs.rs link
- [ ] Dev docs link (zfish-devdocs.vercel.app)
- [ ] Notion roadmap link
- [ ] MIT License badge
- [ ] MSRV (Minimum Supported Rust Version)
- [ ] CI Status

### 2. **Features List** (Lines 38-45)
Check these reflect current capabilities:
- [ ] Zero Dependencies ✓
- [ ] Rich Styling (16 ANSI + 256 colors) ✓
- [ ] Blazing Fast ✓
- [ ] Memory Safe ✓
- [ ] Cross-Platform ✓
- [ ] Lightweight ✓
- [ ] Intuitive API ✓
- [ ] Edition 2024 ✓

### 3. **Quick Start Examples** (Lines 50-230)
Keep examples synchronized with actual API:
- [ ] Installation instructions match Cargo.toml version
- [ ] Hello World example works
- [ ] Argument parsing example uses correct API
- [ ] Progress bar example shows correct methods
- [ ] Subcommands example reflects command.rs API
- [ ] Interactive prompts use Prompt::text/confirm/password
- [ ] Table example shows correct BoxStyle and methods
- [ ] 256-color palette example demonstrates Custom(n)

### 4. **Documentation Links** (Lines 234-244)
Verify all links are accessible:
- [ ] Developer Docs: https://zfish-devdocs.vercel.app
- [ ] API Docs: https://docs.rs/zfish
- [ ] Roadmap: ROADMAP.md file exists
- [ ] Notion Roadmap: https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232
- [ ] PACKAGES.md exists
- [ ] All example files listed actually exist

### 5. **Project Status** (Lines 268-303)
**CRITICAL**: This section must reflect actual implementation!

#### Completed Features Checklist
Compare against `src/` directory:

**Core Styling & Colors** (`src/style.rs`):
- [ ] 16 standard ANSI colors
- [ ] 256-color palette (Color::Custom(u8))
- [ ] Text styling (Bold, Italic, Underline, Dim, Blink, Reverse, Hidden, Strikethrough)

**Argument Parsing & Commands** (`src/args.rs`, `src/command.rs`):
- [ ] Basic Args::parse() with flags, options, positional
- [ ] Subcommand system (App, Command, Arg)
- [ ] Auto-generated --help
- [ ] Argument validation (required, possible values)
- [ ] Environment variable fallbacks (.env())
- [ ] Variadic arguments ([FILES]...)
- [ ] Argument dependencies (.requires())
- [ ] Conflict detection (.conflicts_with())

**Progress & Interactive** (`src/progress.rs`, `src/prompt.rs`):
- [ ] 4 progress bar styles (ProgressStyle enum)
- [ ] Interactive prompts (Prompt::text, confirm, password)

**Terminal Control** (`src/term.rs`):
- [ ] Terminal::clear_screen()
- [ ] Terminal::move_cursor()
- [ ] Terminal::size()
- [ ] Terminal::print_at()

**Logging & Tables** (`src/log.rs`, `src/table.rs`):
- [ ] Logger with levels (error, warn, info, debug, trace)
- [ ] Table with 5 BoxStyles
- [ ] Unicode-aware width calculation (src/unicode.rs)
- [ ] Manual table drawing (draw_box, draw_separator)
- [ ] Column alignment (Alignment enum)

**Development & Examples**:
- [ ] Feature flags match Cargo.toml (colour, raw, progress, interactive)
- [ ] Edition 2024 in Cargo.toml
- [ ] Count examples/*.rs files (should match README claim)

#### Coming Next Section
Only list features that are:
- [ ] NOT yet implemented
- [ ] Planned for next version
- [ ] Have GitHub issues/discussions

### 6. **Version Numbers** (Multiple locations)
Search and replace when bumping versions:
- [ ] Line 12: Badge version
- [ ] Line 55: Installation `zfish = "0.1"`
- [ ] Line 62: Download URL version
- [ ] Line 268: "Current Version: `0.1.10`"
- [ ] Line 422: Footer box version

### 7. **License Section** (Lines 361-366)
- [ ] States "MIT License" only
- [ ] Links to LICENSE file
- [ ] Footer copyright box says "MIT License"

### 8. **Contributing Section** (Lines 295-330)
Keep example commands accurate:
- [ ] `cargo test` works
- [ ] `cargo test -- --test-threads=1` explanation is accurate
- [ ] `cargo doc --open` generates docs
- [ ] All `cargo run --example` commands work
- [ ] Example file list matches examples/ directory

## Automated Checks

Run these commands to verify README accuracy:

```bash
# Count actual examples
ls examples/*.rs | wc -l

# Check version in Cargo.toml
grep "^version" Cargo.toml

# Verify all example files listed in README exist
# (manually check against README list)

# Test that all Quick Start examples compile
# (copy each example to a test file and run)

# Verify all links are accessible
# (use a link checker or manually test)
```

## GitHub Actions Integration

Consider adding a CI check that:
1. Extracts version from Cargo.toml
2. Verifies it appears in README.md
3. Counts examples/ files and compares to README claim
4. Validates all internal file links exist
5. Checks external links are accessible (docs.rs, vercel, notion)

## When Publishing to crates.io

The README.md is automatically included in the crate package. Ensure it's current before:
```bash
cargo publish --dry-run  # Preview what will be published
cargo publish           # Actually publish
```

## Quick Verification Script

```bash
#!/bin/bash
# verify-readme.sh

echo "🔍 Checking README accuracy..."

# Version consistency
CARGO_VERSION=$(grep "^version" Cargo.toml | cut -d'"' -f2)
README_VERSIONS=$(grep -c "$CARGO_VERSION" README.md)
echo "✓ Cargo.toml version: $CARGO_VERSION"
echo "✓ Appears in README: $README_VERSIONS times"

# Example count
EXAMPLE_COUNT=$(ls examples/*.rs 2>/dev/null | wc -l)
echo "✓ Example files: $EXAMPLE_COUNT"

# Link checks
echo "🔗 Checking critical links..."
curl -s -o /dev/null -w "%{http_code}" https://zfish-devdocs.vercel.app
curl -s -o /dev/null -w "%{http_code}" https://docs.rs/zfish
curl -s -o /dev/null -w "%{http_code}" https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232

echo "✅ README verification complete!"
```

---

**Remember**: The README is often the first thing users see. Keep it accurate, current, and honest about what's implemented vs. planned!
