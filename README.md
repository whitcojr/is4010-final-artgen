# 🎨 IS4010-Final-ArtGen: Terminal Art Generator

[![Rust CI Status](https://github.com/whitcojr/is4010-final-artgen/actions/workflows/tests.yml/badge.svg)](https://github.com/whitcojr/is4010-final-artgen/actions/workflows/tests.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Language-Rust--2021-orange?logo=rust)](https://www.rust-lang.org/)

## 📝 Project Description

A fun, fast, and feature-rich command-line interface written in **Rust** to convert plain text into beautiful ASCII art using standard font. 

---

## ✨ Features List

The application can perform the following key functions:

* **Fast Rendering:** Leverages Rust's performance for instant art generation.
* **Multiple Fonts:** Supports popular FIGlet fonts like `standard`, `slant`, `chunky`, and `starwars`.
* **Easy CLI:** Simple argument parsing using the powerful `clap` library.
* **Robust Testing:** Automated unit and integration tests verified by GitHub Actions.

---

## 🚀 Installation Instructions

### Building from Source

To get the application running on your machine:

1.  **Clone the repository** :
    ```bash
    git clone [https://github.com/whitcojr/is4010-final-artgen](https://github.com/whitcojr/is4010-final-artgen)
    cd is4010-final-artgen
    ```
2.  **Build the executable** (the `--release` flag ensures optimal speed):
    ```bash
    cargo build --release
    ```
3.  **Run the application** from the `./target/release/` directory:
    ```bash
    ./target/release/is4010-final-artgen --help
    ```

---

## 💡 Usage Examples

The application takes the text to convert as the primary argument and allows specifying a font using the `-f` or `--font` option.

### Quick Start

```bash
# Build and run in one command
cargo run -- "Hello World"
```

### Example 1: Default Font (Standard)

Generate ASCII art using the default font:

```bash
cargo run -- "Luke Skywalker"
```

Output:
```
  _               _               ____    _                                 _   _
 | |      _   _  | | __   ___    / ___|  | | __  _   _  __      __   __ _  | | | | __   ___   _ __
 | |     | | | | | |/ /  / _ \   \___ \  | |/ / | | | | \ \ /\ / /  / _` | | | |/ /  / _ \ | '__|
 | |___  | |_| | |   <  |  __/    ___) | |   <  | |_| |  \ V  V /  | (_| | | | |   <  |  __/ | |
 |_____|  \__,_| |_|\_\  \___|   |____/  |_|\_\  \__, |   \_/\_/    \__,_| |_| |_|\_\  \___| |_|
                                                 |___/
```

### Example 2: Long Text

The tool handles longer text gracefully:

```bash
cargo run -- "May the force be with you."
```

### Example 3: Special Characters

Works with numbers and special characters:

```bash
cargo run -- "2025!"
cargo run -- "Order#66"
```

### Example 4: Help and Options

View all available options:

```bash
cargo run -- --help
```

Output:
```
Usage: is4010-final-artgen [OPTIONS] <TEXT>

Arguments:
  <TEXT>  The text to convert into ASCII art

Options:
  -h, --help         Print help
  -V, --version      Print version
```

### Example 5: Running the Release Binary

After building a release version, run it directly:

```bash
# Build release
cargo build --release

# Run the binary directly (faster execution)
./target/release/is4010-final-artgen "Final Project"
```

### Tips & Tricks

- **Single quotes on Unix/Linux/macOS**, double quotes on Windows for special characters
- Text is case-sensitive in some fonts
- Longer text may wrap to multiple lines depending on terminal width
- For best results, use short, uppercase text (5-15 characters)

---

## 🧪 Testing

Run the full test suite to ensure everything works correctly:

```bash
cargo test --verbose
```

This includes:
- Unit tests for core functionality
- Integration tests for CLI behavior
- All tests are automatically run by GitHub Actions on every push

---

## 🔧 Development & Code Quality

### Format Code

Keep your code consistently formatted:

```bash
cargo fmt
```

### Lint with Clippy

Check for common mistakes and code improvements:

```bash
cargo clippy -- -D warnings
```

### Build in Debug Mode

For development with additional debugging information:

```bash
cargo build
```

---

## 📋 Project Structure

```
is4010-final-artgen/
├── .github/
│   └── workflows/
│       └── tests.yml                # GitHub Actions CI/CD pipeline
├── src/
│   └── main.rs                      # Application entry point and logic
├── tests/
│   └── cli.rs                       # Integration tests
├── Cargo.toml                       # Rust package manifest
├── Cargo.lock                       # Dependency lock file
├── LICENSE                          # MIT License
├── README.md                        # This file
└── AGENTS.md                        # AI agent involvement documentation
```

---

## 🛠 Dependencies

Key dependencies used in this project:

- **[clap](https://github.com/clap-rs/clap)**: Command-line argument parsing
- **[figlet-rs](https://github.com/figlet-rs/figlet-rs)**: ASCII art generation with FIGlet fonts
- **[anyhow](https://docs.rs/anyhow/)**: Error handling

See `Cargo.toml` for the complete list and versions.

---

## 🔄 CI/CD Pipeline

This project uses **GitHub Actions** for continuous integration and continuous deployment. The workflow runs:

- ✅ **Build**: Compiles the project in release mode
- ✅ **Test**: Runs all unit and integration tests
- ✅ **Rustfmt**: Validates code formatting compliance
- ✅ **Clippy**: Performs advanced linting and static analysis

The pipeline is triggered on every push to `main` branch and on pull requests.

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for the full text and details.

---

## 🤝 Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure all tests pass and code follows the project's style guidelines.

---

## 🙏 Acknowledgments

- **Rust Community**: For the amazing language and ecosystem
- **AI Agents**: This project was developed with assistance from AI agents. See [AGENTS.md](AGENTS.md) for detailed information about their contributions.
- **IS4010 Course**: For the inspiration and requirements

---

## 📞 Support & Questions

If you encounter any issues or have questions:

1. Check existing [GitHub Issues](https://github.com/whitcojr/is4010-final-artgen/issues)
2. Review the [AGENTS.md](AGENTS.md) file for development insights
3. Create a new issue with detailed information about your problem

---

**Last Updated:** December 7, 2025  
**Version:** 1.0.0

---

## 💡 Usage Examples

The application takes the text to convert as the primary argument and allows specifying a font using the `-f` or `--font` option.

### Example 1: Default Font

```bash
./target/release/is4010-final-artgen "Hello"