
# Kite

**Kite** is a Rust library for building secure, dependency-free CLI tools.  
It provides developers with essential utilities like:

- Argument parsing (`args`)
- Styled/colorized output (`style`)
- Progress bars and spinners (`progress`)
- User input prompts (`prompt`)
- Logging utilities (`log`)
- Terminal manipulation (`term`)

The library is implemented **entirely from scratch using only Rust’s standard library**, to avoid third-party supply chain risks and maintain maximum security.

---

## Goals
- Cross-platform (Linux, macOS, Windows)
- No external crates or dependencies
- Simple, minimal API for developers
- Secure and auditable code
- Extensible for future CLI features

---

## Example

```rust
use kite::{args::Args, style::Color, progress::ProgressBar, log::Logger};

fn main() {
    // Argument parsing
    let cli = Args::parse();
    println!("Args: {:?}", cli);

    // Styled text
    println!("{}", Color::Green.paint("Success!"));

    // Progress bar
    let mut pb = ProgressBar::new(100);
    for i in 0..=100 {
        pb.set(i);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    pb.finish("Done!");

    // Logging
    let mut logger = Logger::new();
    logger.info("Build completed");
}
````

---

## Roadmap

* [x] Initial argument parser
* [x] ANSI color styling
* [ ] Progress bar & spinner
* [ ] Input prompt utilities
* [ ] Logging system
* [ ] Terminal helpers (clear, move cursor, detect width)

---

## License

MIT


