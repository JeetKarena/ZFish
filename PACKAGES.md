# 📦 GitHub Packages

This document explains how to install and use `zfish` from GitHub Packages as an alternative to crates.io.

## Why GitHub Packages?

GitHub Packages provides an alternative distribution channel for ZFish:

- **Direct from source**: Download `.crate` files directly from GitHub Releases
- **Checksum verification**: SHA256 and SHA512 checksums for security
- **Full control**: Not dependent on crates.io availability
- **Enterprise-friendly**: Works with GitHub Enterprise and private repositories

## 📥 Installation Methods

### Method 1: From crates.io (Recommended)

The easiest way to use ZFish:

```toml
[dependencies]
zfish = "0.1"
```

```bash
cargo add zfish
```

### Method 2: From GitHub Releases

Download and install from GitHub Packages:

```bash
# Set the version you want
VERSION="0.1.8"

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
zfish = { git = "https://github.com/JeetKarena/ZFish.git", tag = "v0.1.8" }
```

## 🔐 Security & Verification

### Checksum Verification

Every GitHub release includes SHA256 and SHA512 checksums:

```bash
# SHA256
sha256sum zfish-0.1.8.crate
# Should match: zfish-0.1.8.crate.sha256

# SHA512
sha512sum zfish-0.1.8.crate
# Should match: zfish-0.1.8.crate.sha512
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
    VERSION="0.1.8"
    wget https://github.com/JeetKarena/ZFish/releases/download/v${VERSION}/zfish-${VERSION}.crate
    tar -xzf zfish-${VERSION}.crate
```

### GitLab CI

```yaml
install_zfish:
  script:
    - VERSION="0.1.8"
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

- [GitHub Packages Documentation](https://docs.github.com/en/packages)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [ZFish Releases](https://github.com/JeetKarena/ZFish/releases)

---

**Questions?** Open a [GitHub Discussion](https://github.com/JeetKarena/ZFish/discussions)
