# Codebase Structure

**Analysis Date:** 2026-01-31

## Directory Layout

```
mozilla-rust-code-analysis/
├── src/                          # Core library implementation
│   ├── lib.rs                    # Library root, public API exports
│   ├── parser.rs                 # Parser<T> generic container
│   ├── node.rs                   # Node<'a> AST wrapper
│   ├── ast.rs                    # AST serialization (AstNode, AstPayload, AstResponse)
│   ├── asttools.rs               # AST traversal utilities
│   ├── traits.rs                 # Core traits (ParserTrait, LanguageInfo, Callback)
│   ├── checker.rs                # Language-specific structure detection (functions, operators)
│   ├── getter.rs                 # Extract names, kinds, operator types from nodes
│   ├── langs.rs                  # Language enumeration and detection
│   ├── languages/                # Generated language definitions
│   │   ├── mod.rs                # Language module exports
│   │   ├── language_*.rs         # Auto-generated enum definitions (12+ files)
│   ├── metrics/                  # Metric calculation modules
│   │   ├── mod.rs                # Metric module exports
│   │   ├── abc.rs                # ABC complexity
│   │   ├── cognitive.rs          # Cognitive complexity
│   │   ├── cyclomatic.rs         # Cyclomatic complexity (CC)
│   │   ├── exit.rs               # Exit point counting (NEXITS)
│   │   ├── halstead.rs           # Halstead metrics (difficulty, effort, bugs, time)
│   │   ├── loc.rs                # Lines of Code (SLOC, PLOC, LLOC, CLOC, BLANK)
│   │   ├── mi.rs                 # Maintainability Index
│   │   ├── nargs.rs              # Number of arguments per function
│   │   ├── nom.rs                # Number of methods/functions
│   │   ├── npa.rs                # Number of public attributes
│   │   ├── npm.rs                # Number of public methods
│   │   └── wmc.rs                # Weighted Method Count
│   ├── output/                   # Result serialization
│   │   ├── mod.rs                # Output module exports
│   │   ├── dump.rs               # Main output formatting
│   │   ├── dump_metrics.rs       # Metrics serialization (JSON/YAML)
│   │   └── dump_ops.rs           # Operator list output
│   ├── spaces.rs                 # Code space (function/class/file) abstraction
│   ├── concurrent_files.rs       # Multi-threaded file processing
│   ├── count.rs                  # Generic node counting
│   ├── find.rs                   # AST pattern finding
│   ├── function.rs               # Function span detection
│   ├── ops.rs                    # Operator extraction and analysis
│   ├── comment_rm.rs             # Comment removal
│   ├── preproc.rs                # C/C++ preprocessor handling
│   ├── c_macro.rs                # Macro definition extraction
│   ├── c_langs_macros/           # C-specific macro utilities
│   │   ├── mod.rs                # Module root
│   │   ├── c_macros.rs           # Macro parsing
│   │   └── c_specials.rs         # Special handling (includes, guards)
│   ├── alterator.rs              # Code transformation (comment/macro removal)
│   ├── tools.rs                  # Utilities (file I/O, path handling, color output)
│   ├── macros.rs                 # Code generation macros (mk_langs!, implement_metric_trait!)
│   └── getter.rs                 # [NOTE: Large file - 27KB, see below]
├── rust-code-analysis-cli/       # Command-line interface
│   ├── src/
│   │   ├── main.rs               # CLI entry point
│   │   └── formats.rs            # Output format handlers (JSON, YAML, etc.)
│   └── Cargo.toml                # CLI package manifest
├── rust-code-analysis-web/       # HTTP API server
│   ├── src/
│   │   ├── lib.rs                # Web service library root
│   │   ├── bin/rust-code-analysis-web.rs  # Server entry point
│   │   └── web/
│   │       ├── mod.rs            # Web module root
│   │       ├── server.rs         # HTTP server setup (actix/rocket)
│   │       ├── metrics.rs        # /metrics endpoint
│   │       ├── function.rs       # /function endpoint
│   │       └── comment.rs        # /comment endpoint
│   └── Cargo.toml                # Web package manifest
├── enums/                        # Test/helper crate for enum generation
│   ├── src/
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   └── languages.rs
│   └── Cargo.toml
├── tree-sitter-*/                # Custom tree-sitter language grammars
│   ├── tree-sitter-ccomment/     # C comment handling
│   ├── tree-sitter-preproc/      # Preprocessor directives
│   ├── tree-sitter-mozcpp/       # Mozilla C++ variant
│   └── tree-sitter-mozjs/        # Mozilla JavaScript variant
├── tests/                        # Integration tests
│   └── [test files]
├── data/                         # Test data fixtures
├── Cargo.toml                    # Workspace manifest (members: cli, web)
├── Cargo.lock                    # Dependency lock file
├── Makefile                      # Build targets
├── README.md                     # Project documentation
├── .github/                      # GitHub Actions CI config
├── .pre-commit-config.yaml       # Pre-commit hooks
├── .taskcluster.yml              # TaskCluster CI config
└── .typos.toml                   # Typo checker configuration
```

## Directory Purposes

**src/ (Core Library):**
- Purpose: Primary library providing code analysis functionality
- Contains: AST parsing, metric computation, language support, output formatting
- Key files: `lib.rs` (public API), `parser.rs` (main entry), `traits.rs` (interfaces)

**src/languages/:**
- Purpose: Language enum definitions (one file per language)
- Contains: Auto-generated token/syntax element enums via tree-sitter bindings
- Key files: All files are generated from `language_*.rs` pattern
- Auto-generated from: `mk_langs!` macro in `macros.rs`
- Pattern: Each language has enum mapping tree-sitter kind IDs to names

**src/metrics/:**
- Purpose: Modular metric calculation implementations
- Contains: 12 different code quality metrics, each independently computable
- Pattern: Each metric is one file with a `Stats` struct and `compute()` trait method

**src/output/:**
- Purpose: Serialization and formatting of analysis results
- Contains: JSON/YAML output, metrics dumping, operator extraction
- Entry point: Called by CLI and web service after metrics computed

**rust-code-analysis-cli/:**
- Purpose: Standalone command-line tool
- Contains: Argument parsing, file discovery, output formatting
- Entry: `main.rs` with clap CLI parser
- Uses workspace member linking to core library

**rust-code-analysis-web/:**
- Purpose: HTTP API server for remote analysis
- Contains: HTTP endpoints, request handling, response serialization
- Entry: `bin/rust-code-analysis-web.rs` starts actix/rocket server
- Endpoints: /ast, /metrics, /function, /comment, /ops
- Uses workspace member linking to core library

**tree-sitter-*/ (Custom Grammars):**
- Purpose: Language grammar definitions for tree-sitter
- Included languages: C comments (comment handling), Preprocessor (C/C++ directives), Mozilla C++ (browser code), Mozilla JavaScript (Firefox internals)
- Each contains: grammar.js, src/ (C grammar implementation)

## Key File Locations

**Entry Points:**
- `src/lib.rs`: Library public API - re-exports all public modules and functions
- `rust-code-analysis-cli/src/main.rs`: CLI entry - argument parsing, orchestration
- `rust-code-analysis-web/src/bin/rust-code-analysis-web.rs`: Web server startup
- `src/parser.rs`: Parser<T>::new() - instantiation point for analysis

**Configuration:**
- `Cargo.toml`: Workspace root, dependency versions, release profiles
- `Cargo.lock`: Locked dependency versions
- `Makefile`: Make targets (check, test, build, install)
- `rust-code-analysis-cli/Cargo.toml`: CLI-specific dependencies
- `rust-code-analysis-web/Cargo.toml`: Web-specific dependencies (actix, serde)

**Core Logic:**
- `src/parser.rs`: Main Parser<T> generic container
- `src/traits.rs`: ParserTrait interface (core abstraction)
- `src/getter.rs`: Language-specific node extraction (27KB - largest single file)
- `src/checker.rs`: Language-specific structure detection (19KB)
- `src/metrics/*.rs`: Individual metric implementations
- `src/spaces.rs`: Space abstraction (functions, classes, modules)
- `src/concurrent_files.rs`: Multi-threaded orchestration

**Testing:**
- `tests/`: Integration tests directory
- `data/`: Test fixtures and sample code

## Naming Conventions

**Files:**
- Core modules: lowercase with underscores (e.g., `parser.rs`, `comment_rm.rs`)
- Language definitions: `language_<name>.rs` (e.g., `language_rust.rs`, `language_python.rs`)
- Metric modules: `<metric_name>.rs` (e.g., `cyclomatic.rs`, `halstead.rs`)
- Sub-modules: Use `mod.rs` in directory and re-export with `pub use`

**Directories:**
- Core library: `src/`
- Sub-systems: descriptive lowercase (e.g., `metrics/`, `languages/`, `output/`)
- Workspace members: kebab-case (e.g., `rust-code-analysis-cli/`, `rust-code-analysis-web/`)
- Custom grammars: `tree-sitter-<variant>/`

**Modules:**
- Enums: PascalCase (e.g., `Cpp`, `Python`, `SpaceKind`, `LANG`)
- Structs: PascalCase (e.g., `Parser<T>`, `Node<'a>`, `CodeMetrics`)
- Traits: PascalCase (e.g., `ParserTrait`, `Getter`, `Checker`)
- Functions: snake_case (e.g., `get_language()`, `action()`, `guess_language()`)
- Constants: UPPER_CASE (e.g., `AHO_CORASICK`, `RE`)

**Metric naming:**
- Module: lowercase (e.g., `cyclomatic`, `halstead`)
- Stats struct: PascalCase + "Stats" (e.g., `CyclomaticStats`, `HalsteadStats`)
- Trait impl: Just the metric name in caps (via implement_metric_trait! macro)

## Where to Add New Code

**New Language Support:**
1. Add tree-sitter binding: `Cargo.toml` (add `tree-sitter-<lang>` dependency)
2. Add language enum: Create `src/languages/language_<name>.rs` (or auto-generate)
3. Register in `langs.rs`: Add entry to `mk_langs!()` macro with:
   - Enum name, description, display name
   - Empty code struct name
   - Parser name
   - tree-sitter function
   - File extensions
   - Emacs modes
4. Implement analysis: `src/checker.rs` - add language-specific function/class detection
5. Implement getters: `src/getter.rs` - add language-specific name/kind extraction
6. Implement metrics: For each metric module, add trait impl (or use `implement_metric_trait!` macro)

**New Metric:**
1. Create module: `src/metrics/<metric_name>.rs`
   - Define `Stats` struct (derives Clone, Default, Serialize)
   - Implement computation logic in `compute()` function
   - Implement `<MetricName>` trait for all languages
2. Register trait: Add to `src/traits.rs` as associated type on `ParserTrait`
3. Register in `Parser<T>`: Add bound in `src/parser.rs` generic constraints
4. Serialization: Add field to `CodeMetrics` in `src/spaces.rs`
5. Output: Update formatters in `src/output/dump_metrics.rs`
6. CLI: Add flag in `rust-code-analysis-cli/src/main.rs` if user-selectable

**New CLI Option:**
1. Edit: `rust-code-analysis-cli/src/main.rs`
   - Add field to `Config` struct
   - Add clap derive attributes for parsing
   - Use in `act_on_file()` or main logic
2. Test: Run `cargo build -p rust-code-analysis-cli` to verify

**New Web Endpoint:**
1. Create: `rust-code-analysis-web/src/web/<endpoint_name>.rs`
2. Define: Request/response structs with serde Serialize/Deserialize
3. Register: Add route in `src/web/server.rs`
4. Implementation: Use `Parser<T>` and existing analysis functions

**New Output Format:**
1. Edit: `rust-code-analysis-cli/src/formats.rs`
2. Add: New Format enum variant and handler
3. Test: Add integration test

## Special Directories

**target/:**
- Purpose: Cargo build output
- Generated: Yes, by `cargo build`
- Committed: No (.gitignore excludes)
- Contains: Compiled binaries, intermediate objects, dependencies

**tree-sitter-*/ (Custom Grammars):**
- Purpose: Local tree-sitter language definitions
- Generated: No (source is grammar.js, C code generated by tree-sitter-cli)
- Committed: Yes - checked in as dependencies
- Each has: grammar.js (grammar definition), src/ (generated C bindings)

**.github/:**
- Purpose: GitHub Actions CI/CD workflows
- Generated: No
- Committed: Yes
- Contains: Release, test workflows (recently cleaned per commits)

**enums/:**
- Purpose: Helper crate for developing/testing enum generation
- Generated: No (source crate)
- Committed: Yes
- Excluded: From workspace members (exclude = ["enums"])

---

*Structure analysis: 2026-01-31*
