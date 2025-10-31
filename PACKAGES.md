# 📦 Installation Guide

This document explains different ways to install and use `zfish` in your Rust projects.

## Installation Methods

ZFish is primarily distributed through crates.io, with GitHub Releases providing backup distribution:

- **Stable releases**: Published on crates.io
- **Direct downloads**: `.crate` files available from GitHub Releases
- **Checksum verification**: SHA256 checksums for security
- **Version control**: Semantic versioning following semver standards

## 📥 Installation Methods

### Method 1: From crates.io (Recommended)

The standard and easiest way to use ZFish:

```toml
[dependencies]
zfish = "0.1"
```

Or use cargo add:

```bash
cargo add zfish
```

### Method 2: From GitHub Releases (Backup)

Download and install from GitHub Packages:

```bash
# Set the version you want
VERSION="0.1.10"

# Download the .crate file
wget https://github.com/JeetKarena/ZFish/releases/download/v${VERSION}/zfish-${VERSION}.crate

# Download checksums
wget https://github.com/JeetKarena/ZFish/releases/download/v${VERSION}/zfish-${VERSION}.crate.sha256
wget https://github.com/JeetKarena/ZFish/releases/download/v${VERSION}/zfish-${VERSION}.crate.sha512

# Verify integrity (choose one)
sha256sum -c zfish-${VERSION}.crate.sha256
sha512sum -c zfish-${VERSION}.crate.sha512

# Extract and use
tar -xzf zfish-${VERSION}.crate
```

### Method 3: Direct Git Dependency

Use the latest development version:

```toml
[dependencies]
zfish = { git = "https://github.com/JeetKarena/ZFish.git", branch = "main" }
```

Or a specific tag:

```toml
[dependencies]
zfish = { git = "https://github.com/JeetKarena/ZFish.git", tag = "v0.1.10" }
```

## 🔐 Security & Verification

### Checksum Verification

Every GitHub release includes SHA256 and SHA512 checksums:

```bash
# SHA256
sha256sum zfish-0.1.10.crate
# Should match: zfish-0.1.10.crate.sha256

# SHA512
sha512sum zfish-0.1.10.crate
# Should match: zfish-0.1.10.crate.sha512
```

### GPG Signatures (Coming Soon)

Future releases will include GPG signatures for additional verification.

## 📦 Package Contents

Each GitHub release includes:

- `zfish-{version}.crate` — The compiled Rust crate
- `zfish-{version}.crate.sha256` — SHA256 checksum
- `zfish-{version}.crate.sha512` — SHA512 checksum
- Source code (zip/tar.gz) — Full source archive

## 🔄 Automated Publishing

GitHub Packages publishing is fully automated:

1. **Auto-release workflow** bumps version based on commit messages
2. **Release workflow** publishes to crates.io
3. **GitHub Packages workflow** uploads artifacts to GitHub Releases
4. All workflows run on every tagged release

## 🌐 Using in CI/CD

### GitHub Actions

```yaml
- name: Install zfish
  run: |
    VERSION="0.1.10"
    wget https://github.com/JeetKarena/ZFish/releases/download/v${VERSION}/zfish-${VERSION}.crate
    tar -xzf zfish-${VERSION}.crate
```

### GitLab CI

```yaml
install_zfish:
  script:
    - VERSION="0.1.10"
    - wget https://github.com/JeetKarena/ZFish/releases/download/v${VERSION}/zfish-${VERSION}.crate
    - tar -xzf zfish-${VERSION}.crate
```

## 📊 Comparison

| Feature | crates.io | GitHub Packages |
|---------|-----------|-----------------|
| **Ease of use** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Integration** | Native Cargo | Manual download |
| **Security** | Checksums | SHA256 + SHA512 |
| **Availability** | 99.9% SLA | GitHub uptime |
| **Version control** | Immutable | Git-based |
| **Private repos** | ❌ | ✅ (with GitHub Enterprise) |

## 🤝 Contributing

If you notice issues with GitHub Packages distribution, please [open an issue](https://github.com/JeetKarena/ZFish/issues).

## 📚 Additional Resources

- **[Developer Documentation](https://zfish-devdocs.vercel.app)** — Interactive guides and examples
- **[API Reference](https://docs.rs/zfish)** — Complete API documentation
- **[GitHub Packages Documentation](https://docs.github.com/en/packages)** — Official GitHub guide
- **[Cargo Documentation](https://doc.rust-lang.org/cargo/)** — Rust package manager docs
- **[ZFish Releases](https://github.com/JeetKarena/ZFish/releases)** — All versions and downloads
- **[Roadmap](https://sprinkle-toque-13b.notion.site/ZFish-29d4eaaebc9d80bd82f3c27833a92232)** — Feature status and upcoming releases

---

**Questions?** Open a [GitHub Discussion](https://github.com/JeetKarena/ZFish/discussions)

**License:** [MIT License](https://github.com/JeetKarena/ZFish/blob/main/LICENSE)
