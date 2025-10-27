# Security Policy

## 🔒 Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

**Note:** As ZFish is a CLI framework library (not a service), most security considerations relate to:
- Terminal escape sequence handling
- Input validation
- File system operations (if any)
- Dependencies (we use zero external dependencies)

---

## 🛡️ Security Considerations

### Terminal Safety

ZFish handles ANSI escape sequences for terminal control. We take the following precautions:

- ✅ All escape sequences are carefully validated
- ✅ No user input is directly interpolated into escape sequences
- ✅ Terminal state is properly restored on errors
- ✅ No execution of shell commands from user input

### Input Validation

When using ZFish's interactive prompt features:

- ✅ Input is sanitized before display
- ✅ No code execution from user input
- ✅ Buffer overflow protection via Rust's memory safety

### Platform-Specific Code

ZFish uses platform-specific APIs (Windows Console API, Unix ioctl):

- ✅ FFI calls are carefully validated
- ✅ Proper error handling for all system calls
- ✅ No unsafe memory operations without bounds checking

---

## 🚨 Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in ZFish, please follow these steps:

### **DO NOT** open a public issue

Public disclosure of vulnerabilities can put the entire community at risk.

### ✅ Instead, Report Privately

**Method 1: GitHub Security Advisories (Recommended)**

1. Go to https://github.com/JeetKarena/ZFish/security/advisories
2. Click "Report a vulnerability"
3. Fill in the details following the template below

**Method 2: Email**

Send an email to: **karenajeet@proton.me**

### 📋 Vulnerability Report Template

Please include the following information:

```markdown
## Summary
Brief description of the vulnerability

## Severity
Your assessment: Critical / High / Medium / Low

## Component
Which part of ZFish is affected?
- [ ] Terminal control (term.rs)
- [ ] ANSI escape sequences (style.rs)
- [ ] Progress bars (progress.rs)
- [ ] Interactive prompts (prompt.rs)
- [ ] Argument parsing (args.rs)
- [ ] Platform-specific code (os/)
- [ ] Other: ___________

## Description
Detailed description of the vulnerability

## Steps to Reproduce
1. Step 1
2. Step 2
3. Step 3

## Proof of Concept
```rust
// Code that demonstrates the vulnerability
```

## Impact
What could an attacker achieve with this vulnerability?

## Suggested Fix (Optional)
If you have ideas on how to fix this

## Environment
- OS: 
- Rust version:
- ZFish version:
```

---

## 🔍 What Happens Next?

1. **Acknowledgment** - We'll acknowledge receipt within **48 hours**
2. **Investigation** - We'll investigate and confirm the vulnerability
3. **Timeline** - We'll provide an expected timeline for a fix
4. **Coordination** - We'll work with you on coordinated disclosure
5. **Fix & Release** - We'll release a patch and security advisory
6. **Credit** - You'll be credited in the advisory (unless you prefer anonymity)

---

## 🕐 Response Timeline

- **Initial Response:** Within 48 hours
- **Status Update:** Within 7 days
- **Fix Timeline:** Depends on severity
  - Critical: Within 7 days
  - High: Within 30 days
  - Medium: Within 90 days
  - Low: Next regular release

---

## 🏆 Security Hall of Fame

We appreciate security researchers who help keep ZFish safe:

<!-- Security researchers who responsibly disclose vulnerabilities will be listed here -->

*No vulnerabilities reported yet. Be the first!*

---

## 📚 Security Best Practices for Users

When using ZFish in your applications:

### ✅ DO:

- Keep ZFish updated to the latest version
- Validate user input before passing to ZFish functions
- Handle errors appropriately
- Review terminal output in security-sensitive contexts
- Use ZFish's safe APIs for terminal control

### ❌ DON'T:

- Don't pass unsanitized user input directly to style functions
- Don't execute shell commands based on user input
- Don't trust terminal size detection in security-critical code
- Don't use ZFish in setuid/setgid programs without review

### Example: Safe Input Handling

```rust
use zfish::{Style, Color};

// ✅ Safe - controlled styling
let user_input = get_user_input();
let sanitized = user_input.replace('\x1b', ""); // Remove escape sequences
println!("{}", Style::new().fg(Color::Green).apply(&sanitized));

// ❌ Unsafe - potential injection
let user_input = get_user_input();
println!("\x1b[32m{}\x1b[0m", user_input); // DON'T DO THIS
```

---

## 🔐 Security Features

### Memory Safety

ZFish is written in **Rust**, providing:

- No buffer overflows
- No use-after-free
- No null pointer dereferences
- Thread safety

### Zero Dependencies

ZFish has **zero external dependencies**, which means:

- Smaller attack surface
- No supply chain vulnerabilities
- Easier security auditing
- Faster security response

### Minimal `unsafe` Code

- `unsafe` is only used for necessary FFI calls
- All `unsafe` blocks are documented and reviewed
- Platform-specific code is isolated in `os/` module

---

## 📜 Security Disclosure Policy

### Coordinated Disclosure

We follow a **coordinated disclosure** process:

1. Security researchers report vulnerabilities privately
2. We investigate and develop a fix
3. We coordinate a disclosure date
4. We release a patch
5. We publish a security advisory
6. Public disclosure occurs

### Public Disclosure Timeline

- **90 days** from initial report, or
- **When a fix is released**, whichever comes first

We may request additional time for complex vulnerabilities.

---

## 🚀 Security Updates

Security patches are released as soon as possible:

- **Patch version bump** for security fixes
- Published to crates.io immediately
- GitHub Security Advisory published
- Announcement in repository README

### Stay Informed

- ⭐ **Watch** this repository for security advisories
- 📧 **Subscribe** to GitHub notifications
- 🔔 **Enable** security alerts for your projects using ZFish

---

## 📞 Contact

For security-related inquiries:

- **Security Email:** security@zfish.dev
- **GitHub Security:** https://github.com/JeetKarena/ZFish/security/advisories
- **General Contact:** See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

---

## 🙏 Acknowledgments

We thank the security research community for helping keep ZFish and its users safe.

**Responsible disclosure is appreciated and will be recognized.**

---

## 📖 Additional Resources

- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [CWE - Common Weakness Enumeration](https://cwe.mitre.org/)

---

*Last Updated: October 2025*

**Stay Safe! 🦈🔒**
