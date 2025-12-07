# 🤖 Agent Usage Documentation

This document logs the involvement of AI agents in the development of the **`is4010-final-artgen`** project.

| Date | Agent/Model Used | Purpose of Usage | Output Used |
| :--- | :--- | :--- | :--- |
| **Dec 7, 2025** | Gemini (Flash 2.5) | Idea generation, project structure setup (including the specific repository name), initial `Cargo.toml` dependencies, initial `src/main.rs` implementation (using `clap` and `figlet-rs`), complete integration testing setup (`tests/cli.rs`), and generation of all professional documentation (`README.md`, `LICENSE`, `AGENTS.md`, and CI/CD workflow). | **High.** The foundational code and entire project structure were provided by the agent. |
| **Dec 7, 2025** | GitHub Copilot | Created MIT license file (`LICENSE`), generated comprehensive GitHub Actions CI workflow (`.github/workflows/tests.yml` with build, test, rustfmt, and clippy jobs), and updated `AGENTS.md` documentation. | **High.** Professional licensing and robust CI/CD pipeline setup. |
| **Dec 7, 2025 (Ongoing)** | GitHub Copilot | Comprehensive project refinement including: fixed code formatting issues (Rustfmt compliance), debugged and fixed compilation errors in `src/main.rs` (API compatibility with figlet-rs 0.1.5), updated test assertions to work with current API, resolved edition and dependency version issues in `Cargo.toml`, enhanced `.gitignore` with comprehensive entries, added `.github/workflows/tests.yml` renaming to `tests.yml`, updated all documentation references, debugged GitHub Actions CI/CD failures and ensured all tests pass locally with no warnings. | **High.** Critical bug fixes, comprehensive documentation improvements, and full CI/CD pipeline validation ensuring project meets professional standards. |

## Acknowledgment

The foundational structure, tooling suggestions, and initial logic for this project were generated through an iterative process with the Gemini agent. Ongoing development, debugging, and refinement have been guided by GitHub Copilot to ensure code quality and professional documentation standards.