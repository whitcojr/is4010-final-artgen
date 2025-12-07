# 🎨 IS4010-Final-ArtGen: Terminal Art Generator

[![Rust CI Status](https://github.com/YourUsername/is4010-final-artgen/actions/workflows/tests.yml/badge.svg)](https://github.com/YourUsername/is4010-final-artgen/actions/workflows/tests.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Language-Rust--2021-orange?logo=rust)](https://www.rust-lang.org/)

## 📝 Project Description

A fun, fast, and feature-rich command-line interface written in **Rust** to convert plain text into beautiful ASCII art using standard and custom FIGlet fonts. This project serves as the final submission for **IS4010**.

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

1.  **Clone the repository** (replace `YourUsername` with your GitHub username):
    ```bash
    git clone [https://github.com/YourUsername/is4010-final-artgen.git](https://github.com/YourUsername/is4010-final-artgen.git)
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

### Example 1: Default Font

```bash
./target/release/is4010-final-artgen "Hello"