# Gemini Code Assistant Context: rust-code-analysis

This document provides context for the `rust-code-analysis` project, a suite of tools for parsing and analyzing source code in multiple languages.

## Beads Integration

This project uses **Beads** for issue tracking. Always use `bd` commands:

```bash
# Before starting work
bd ready                                    # Show available tasks
bd show <id>                                # View task details
bd update <id> --status=in_progress --actor=gemini

# After completing work
bd close <id> --actor=gemini
bd sync                                     # Push to remote
```

**Current Focus**: See `.planning/REQUIREMENTS.md` and `.planning/ROADMAP.md`

## Project Overview

`rust-code-analysis` is a Rust-based workspace containing a core library, a command-line interface (CLI), and a web service. It leverages the `tree-sitter` parsing framework to build Abstract Syntax Trees (ASTs) from source files and compute a wide variety of code metrics, such as complexity and maintainability.

### Key Components

*   **`rust-code-analysis` (library):** The core crate that provides APIs for parsing code, traversing ASTs, and calculating metrics.
*   **`rust-code-analysis-cli`:** A powerful command-line tool for utilizing the library's features on files and directories. It supports various operations like metric calculation, AST dumping, comment removal, and code pattern searches.
*   **`rust-code-analysis-web`:** A web server that exposes the analysis capabilities via a REST API.
*   **`tree-sitter-*` grammars:** The project includes submodules for various `tree-sitter` language grammars (e.g., Rust, Python, JavaScript, C++, Java) which are fundamental to its parsing capabilities.

The primary goal of the project is to provide a robust, multi-language tool for static code analysis, with a strong focus on code quality and maintainability metrics.

## Building and Running

The project is a standard Rust workspace. Common tasks are managed via Cargo and a `Makefile`.

### Building the Project

*   **Build the entire workspace (recommended):**
    ```bash
    cargo build --workspace
    ```
*   **Build a specific package (e.g., the CLI):**
    ```bash
    cargo build -p rust-code-analysis-cli
    ```
*   **Build in release mode:**
    ```bash
    cargo build --release --workspace
    ```
*   **Using the Makefile:**
    ```bash
    make build # Development build
    make build-release # Release build
    ```

### Running Tests

The project uses `cargo test` for unit and integration testing, and `insta` for snapshot testing.

*   **Run all tests:**
    ```bash
    cargo test --workspace --all-features
    ```
*   **Update snapshot tests:**
    To review and update failing `insta` snapshots, first install `cargo-insta`, then run:
    ```bash
    cargo insta test --review
    ```

## Key Tools & Usage

### Command-Line Interface (`rust-code-analysis-cli`)

The CLI is the main entry point for most users. It can be run via `cargo run -p rust-code-analysis-cli --`.

**Common Commands:**

*   **Calculate Metrics:** Calculate a wide range of metrics for a given file and output in JSON format.
    ```bash
    cargo run -p rust-code-analysis-cli -- --paths /path/to/file.rs --metrics --output-format json
    ```

*   **Dump AST:** Print the full Abstract Syntax Tree of a file.
    ```bash
    cargo run -p rust-code-analysis-cli -- --paths /path/to/file.py --dump
    ```

*   **Find Nodes:** Search for specific node types within the AST.
    ```bash
    cargo run -p rust-code-analysis-cli -- --paths /path/to/file.js --find "function_declaration"
    ```

*   **Count Nodes:** Count occurrences of specific node types.
    ```bash
    cargo run -p rust-code-analysis-cli -- --paths /path/to/file.java --count "method_declaration,field_declaration"
    ```

*   **Process Directories:** Use globs and multi-threading to process entire directories.
    ```bash
    cargo run -p rust-code-analysis-cli -- --paths ./src -j 8 --include "**/*.rs" --metrics
    ```

## Development Conventions

*   **Formatting:** The project uses `cargo fmt`. Run `make fmt` to format the entire workspace.
*   **Linting:** `cargo check` is used for linting. Run `make check`.
*   **Pre-commit Hooks:** The project uses `.pre-commit-config.yaml` to enforce checks before committing. This includes linting, formatting, and other quality gates.
*   **Dependencies:** Language grammars are included as git submodules.
