# Release Process

This document describes how to create a new release of ZFish.

## Philosophy

**No automatic version bumps.** All releases are manual and intentional to maintain proper semantic versioning and release quality.

## Release Steps

### 1. Prepare the Release

```bash
# Ensure you're on main and up to date
git checkout main
git pull origin main

# Ensure all tests pass
cargo test --all
cargo clippy --all-targets -- -D warnings -D dead_code
cargo fmt -- --check
```

### 2. Update Version

Edit `Cargo.toml` and update the version following [Semantic Versioning](https://semver.org/):

- **MAJOR** (x.0.0): Breaking changes
- **MINOR** (0.x.0): New features, backward compatible
- **PATCH** (0.0.x): Bug fixes, backward compatible

```toml
[package]
version = "0.2.0"  # Update this line
```

### 3. Update CHANGELOG

Edit `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/):

```markdown
## [0.2.0] - 2025-10-31

### Added
- New feature descriptions

### Changed
- Changed functionality descriptions

### Fixed
- Bug fix descriptions
```

### 4. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: release v0.2.0"
```

### 5. Create and Push Tag

```bash
# Create the version tag
git tag v0.2.0

# Push the commit and tag
git push origin main
git push origin v0.2.0
```

### 6. Automated Steps

Once the tag is pushed, GitHub Actions will automatically:

1. ✅ Verify version matches tag
2. 🧪 Run all tests
3. 🔍 Run clippy checks
4. 🔨 Build release version
5. 📦 Publish to crates.io (if not already published)
6. 📝 Create GitHub release with artifacts

## Troubleshooting

### Version Already Published

If the version already exists on crates.io, the workflow will skip publishing and just create the GitHub release.

### Failed CI Checks

If tests or clippy fail, the release will not be published. Fix the issues and create a new patch version.

### Manual Publish

If you need to publish manually:

```bash
# Dry run to check everything
cargo publish --dry-run

# Actual publish
cargo publish
```

## CI/CD Workflows

### CI Workflow (`ci.yml`)
- **Triggers**: Pull requests and pushes to main (only for code changes)
- **Purpose**: Ensure code quality and tests pass
- **Runs**: Tests, clippy, formatting checks, documentation

### Release Workflow (`release.yml`)
- **Triggers**: Only when version tags (v*.*.*) are pushed
- **Purpose**: Publish releases to crates.io and create GitHub releases
- **Runs**: Version verification, tests, publishing, artifact creation

## Notes

- **No auto version bumps**: Version changes must be explicit in Cargo.toml
- **No GitHub Packages**: Removed due to Docker dependency issues
- **Semantic versioning**: Follow semver strictly
- **Quality first**: All checks must pass before release
