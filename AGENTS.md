# 🤖 Agent Usage Documentation

This document logs the involvement of AI agents in the development of the **`is4010-final-artgen`** project.

| Date | Agent/Model Used | Purpose of Usage | Output Used |
| :--- | :--- | :--- | :--- |
| **Dec 7, 2025** | Gemini (Flash 2.5) | Idea generation, project structure setup (including the specific repository name), initial `Cargo.toml` dependencies, initial `src/main.rs` implementation (using `clap` and `figlet-rs`), complete integration testing setup (`tests/cli.rs`), and generation of all professional documentation (`README.md`, `LICENSE`, `AGENTS.md`, and CI/CD workflow). | **High.** The foundational code and entire project structure were provided by the agent. |
| **Dec 7, 2025** | GitHub Copilot | Created MIT license file (`LICENSE`), generated comprehensive GitHub Actions CI workflow (`.github/workflows/tests.yml` with build, test, rustfmt, and clippy jobs), and updated `AGENTS.md` documentation. | **High.** Professional licensing and robust CI/CD pipeline setup. |
| **Ongoing** | N/A | Future development and refinement. | N/A |

## Acknowledgment

The foundational structure, tooling suggestions, and initial logic for this project were generated through an iterative process with the Gemini agent.