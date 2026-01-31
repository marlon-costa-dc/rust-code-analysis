# Technology Stack

**Analysis Date:** 2026-01-31

## Languages

**Primary:**
- Rust 1.92.0 - Main library and all binaries; Edition 2024

**Secondary:**
- Python - Utilities for testing and grammar validation (`check-grammar-crate.py`, `split-minimal-tests.py`)
- Shell - Build and grammar recreation scripts

## Runtime

**Environment:**
- Rust toolchain 1.92.0 (ded5c06cf 2025-12-08)
- Cargo 1.92.0 (344c4567c 2025-10-21)
- Nightly Rust required for `cargo +nightly udeps` (development only)

**Package Manager:**
- Cargo (Rust's package manager)
- Lockfile: `Cargo.lock` present and committed

## Frameworks

**Core:**
- `tree-sitter` 0.26.3 - Incremental parsing library for AST generation and code analysis
- `actix-web` 4.2 - Web framework for HTTP server in `rust-code-analysis-web`
- `actix-rt` 2.6 - Async runtime for web service

**Serialization:**
- `serde` 1.0 (with derive feature) - Serialization/deserialization
- `serde_json` 1.0 - JSON encoding/decoding
- `serde_cbor` 0.11 - CBOR format support
- `serde_yaml` 0.9 - YAML format support
- `toml` 0.9 - TOML format support

**Testing:**
- `insta` 1.46.1 - Snapshot testing with YAML/JSON/redactions features
- `pretty_assertions` 1.3 - Better assertion output for tests

**Build/Dev:**
- `cargo fmt` - Code formatting (integrated via pre-commit)
- `cargo clippy` - Linting with `-D warnings` enforcement
- `cargo +nightly udeps` - Detect unused dependencies

## Key Dependencies

**Critical:**
- `tree-sitter` 0.26.3 - Core parsing for 12+ languages
  - Exact version pinning (`=0.26.3`) for stability
  - Language grammars: `tree-sitter-java` 0.23.5, `tree-sitter-typescript` 0.23.2, `tree-sitter-javascript` 0.25.0, `tree-sitter-python` 0.25.0, `tree-sitter-rust` 0.24.0, `tree-sitter-kotlin-codanna` 0.3.9

**Local Tree-Sitter Grammars:**
- `tree-sitter-preproc` 0.20.3 - C preprocessor directives
- `tree-sitter-ccomment` 0.20.3 - C comment handling
- `tree-sitter-mozcpp` 0.20.4 - Mozilla C++ extensions
- `tree-sitter-mozjs` 0.20.3 - Mozilla JavaScript extensions

**Algorithms & Data Structures:**
- `petgraph` 0.8 - Graph algorithms for AST traversal
- `regex` 1.7 - Regular expression matching
- `aho-corasick` 1.0 - Multi-pattern string matching
- `walkdir` 2.3 - Filesystem traversal
- `globset` 0.4 - Glob pattern matching

**Numeric & Formatting:**
- `num` 0.4, `num-derive` 0.4, `num-traits` 0.2 - Numeric abstractions
- `num-format` 0.4 - Number formatting

**Utilities:**
- `crossbeam` 0.8 (with crossbeam-channel feature) - Threading primitives
- `clap` 4.0 (with derive feature) - CLI argument parsing
- `termcolor` 1.2 - Terminal color output
- `futures` 0.3 - Async abstractions (used in web service)

## Configuration

**Environment:**
- No `.env` files detected
- Configuration via CLI arguments for both CLI and web service tools
- Hardcoded defaults in code (e.g., web server defaults: host "127.0.0.1", port 8080, 4 threads)

**Build:**
- `Cargo.toml` files in 8 locations:
  - `/home/marlonsc/mozilla-rust-code-analisis/Cargo.toml` (root workspace)
  - `rust-code-analysis-cli/Cargo.toml` (CLI tool)
  - `rust-code-analysis-web/Cargo.toml` (Web service)
  - 5 tree-sitter grammar crates in `tree-sitter-*/` directories

**Release Profile:**
- Aggressive optimization: `opt-level = 3`, `lto = true`, `codegen-units = 1`
- Debug info stripped: `strip = "debuginfo"`
- No debug assertions, overflow checks disabled in release builds

## Platform Requirements

**Development:**
- Rust 1.92.0+
- Cargo
- Nightly Rust (for `cargo +nightly udeps` pre-commit hook)
- Python 3 (for utility scripts)
- C compiler (tree-sitter native code compilation)

**Production:**
- Statically compiled binaries (no runtime dependencies beyond libc)
- Supported targets: Any platform Rust supports

## Workspace Structure

**Workspace members:**
- `rust-code-analysis` (core library)
- `rust-code-analysis-cli` (command-line interface)
- `rust-code-analysis-web` (HTTP server)

**Workspace excludes:**
- `enums` (template/helper crate, not part of main workspace)

---

*Stack analysis: 2026-01-31*
