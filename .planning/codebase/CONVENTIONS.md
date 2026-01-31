# Coding Conventions

**Analysis Date:** 2026-01-31

## Naming Patterns

**Files:**
- snake_case: `c_macro.rs`, `comment_rm.rs`, `concurrent_files.rs`, `asttools.rs`
- Module files: `mod.rs` for module organization (e.g., `src/c_langs_macros/mod.rs`, `src/languages/mod.rs`)
- Grouped by functionality: metrics, languages, output directories

**Functions:**
- snake_case: `read_file()`, `read_file_with_eol()`, `get_function_spaces()`, `count_specific_ancestors()`
- Descriptive verbs: `get_`, `check_`, `compute_`, `merge_`, `replace()`
- Private functions: Use `fn` without pub, example: `fn dump_span()`, `fn dump_spans()`
- Inline hints: `#[inline(always)]` used for small performance-critical functions like `is_identifier_part()`

**Variables:**
- snake_case: `code_start`, `k_start`, `start_line`, `end_line`, `assignments_sum`
- Underscores for unused: `_guard`, `_simulatedSedEdit`
- Abbreviations acceptable in context: `cc` (cyclomatic complexity), `mi` (maintainability index)

**Types and Structs:**
- PascalCase: `FunctionSpan`, `CountCfg`, `Count`, `Ops`, `Stats`, `Tree`, `Node`
- Metrics types: `Abc`, `Cognitive`, `Cyclomatic`, `Halstead`, `Loc`, `Nom`, `Nargs`, `Exit`
- Trait names: `ParserTrait`, `Callback`, `LanguageInfo`, `Search`, `Alterator`, `Checker`, `Getter`

**Constants:**
- SCREAMING_SNAKE_CASE: `DOLLARS`, `REPO`, `SNAPSHOT_PATH`, `AHO_CORASICK`, `RE`
- OnceLock patterns: `static AHO_CORASICK: OnceLock<AhoCorasick>`

**Enums:**
- PascalCase: `DeclKind` (variants: `Var`, `Const`)
- Language enum: `LANG` (uppercase for language identifier)

## Code Style

**Formatting:**
- Cargo fmt is configured but commented out in `.pre-commit-config.yaml` (line 17-23: "FIXME: Uncomment when fmt is fixed")
- No `rustfmt.toml` found - uses default Rust formatting
- Line length: No explicit limit detected; code follows standard Rust conventions
- Indentation: 4 spaces (standard Rust)

**Linting:**
- Cargo clippy enabled in pre-commit hooks: `cargo clippy --all-targets --all -- -D warnings`
- Treat warnings as errors (`-D warnings`)
- Clippy runs on all targets

**Code Organization:**
- Structs defined with fields in logical order (e.g., `name`, `start_line`, `end_line`, `error` in `FunctionSpan`)
- Private helper functions immediately follow public API functions
- Tests grouped in `#[cfg(test)]` modules at end of files

## Import Organization

**Order:**
1. Standard library: `use std::...` (e.g., `use std::io::Write`, `use std::path::PathBuf`)
2. External crates: `use serde::`, `use termcolor::`, `use regex::`, `use tree_sitter::`
3. Internal crate: `use crate::...` (e.g., `use crate::traits::*`, `use crate::checker::Checker`)

**Patterns:**
- Glob imports: `use crate::*` for broad module re-exports (seen in `c_langs_macros/mod.rs`, `metrics/abc.rs`)
- Specific imports for clarity: `use crate::checker::Checker`, `use crate::getter::Getter`
- Trait imports: `use crate::traits::*` to bring all trait methods into scope
- Star imports used liberally for related types: `use crate::*` in metrics modules

**Path Aliases:**
- No path aliases detected (no configuration in Cargo.toml)

## Error Handling

**Patterns:**
- `unwrap()` used for known-safe operations: `parser.set_language(...).unwrap()`, `parser.parse(...).unwrap()`
- `?` operator for propagating errors: `read_file()` returns `std::io::Result<Vec<u8>>`
- `Option<T>` for nullable values: `fn get_func_name(n, code) -> Option<&str>`
- `is_some()`, `is_none()` for Option checking
- `if let Some(value) = ...` for safe extraction: Example in `function.rs` line 41-42

**Error Types:**
- `std::io::Result<()>` for I/O operations
- Custom error handling via `Option` types (return `None` when detection fails)
- No custom error types detected - relies on standard library conventions

**Panic Safety:**
- Unwrap used only on guaranteed-safe operations (parsing setup)
- File I/O wrapped in `?` for graceful error propagation
- Example: `read_file_with_eol()` returns `Option` indicating failure

## Logging

**Framework:** No logger framework detected (console output uses termcolor for colors)

**Patterns:**
- Direct stdout/stderr: `eprintln!()` for errors in tests (e.g., `tests/common/mod.rs` line 90)
- Colored output: `use termcolor::{Color, StandardStream}` for formatted terminal output
- Functions: `color()`, `intense_color()` wrappers in `tools.rs` and `function.rs`
- No structured logging or log macros detected

## Comments

**When to Comment:**
- Doc comments on public APIs: `///` for public functions and types
- Doc comment example: `/// Function span data.` above `FunctionSpan` struct
- Module docs: `//!` at top of `lib.rs` explaining library purpose and supported languages
- Algorithm explanation: Comments explaining complex logic (e.g., macro processing in `c_macro.rs`)
- TODO/FIXME notes for known issues: See concerns document

**JSDoc/TSDoc:**
- Not applicable - Rust uses `///` doc comments instead
- Example from `function.rs` line 28-31:
  ```rust
  /// Detects the span of each function in a code.
  ///
  /// Returns a vector containing the [`FunctionSpan`] of each function
  ```

**Doc links:**
- Cross-references with backticks: ``[`FunctionSpan`]: struct.FunctionSpan.html``
- URL linking for external docs (e.g., links in `lib.rs` to GitHub)

## Function Design

**Size:**
- Small focused functions: `is_identifier_part()` (3 lines), `is_identifier_starter()` (3 lines)
- Medium functions: `count()` (26 lines), `dump_span()` (33 lines)
- Larger functions for complex operations: `read_file_with_eol()` (40 lines), `replace()` (49 lines)
- Largest files decomposed into logical modules: `checker.rs` (730 lines split across multiple concerns)

**Parameters:**
- Generic type parameters for reusability: `fn count<T: ParserTrait>(parser: &T, filters: &[String])`
- Builder/config structs for multiple options: `FunctionCfg`, `CountCfg` instead of multiple params
- Trait bounds for flexibility: `T: ParserTrait`, `T: Getter`

**Return Values:**
- `Result<T>` for fallible operations: `std::io::Result<Vec<u8>>`
- `Option<T>` for nullable values: `Option<String>`, `Option<FunctionSpan>`
- Tuples for multiple values: `fn count<T>(...) -> (usize, usize)` returns (good, total) count
- Custom structs for complex returns: `FunctionSpan`, `Ops` contain multiple related fields

## Module Design

**Exports:**
- Public re-exports in `lib.rs`: `pub use crate::function::*;`, `pub use crate::count::*;`
- Conditional public: `pub(crate)` for internal APIs (e.g., `pub(crate) use languages::*;`)
- Private modules: `mod c_macro;` then `pub use c_macro::*;` for controlled export

**Barrel Files:**
- `lib.rs` acts as barrel file: 106 lines re-exporting all public modules
- Each module can be accessed as: `use rust_code_analysis::FunctionSpan`, `use rust_code_analysis::count`
- Language-specific modules: `languages/mod.rs` groups all language implementations

**Macro-Heavy Modules:**
- `checker.rs` (730 lines): Mostly macro rules for language-specific checkers
- `macros.rs` (326 lines): Central macro definitions for trait implementation
- Macro patterns hide complexity while reusing code across languages

## Traits and Generics

**Key Traits:**
- `ParserTrait`: Parameterizes across languages, defines associated types for each metric
- `LanguageInfo`: Associates language identifiers with implementations
- `Search<'a>`: Provides AST traversal methods with lifetime bounds
- `Checker`: Language-specific node type checking
- `Getter`: Language-specific metadata extraction

**Generic Patterns:**
- Type parameters with trait bounds: `fn count<T: ParserTrait>(...)`
- Associated types for modularity: `type Checker: Alterator + Checker`, `type Getter: Getter`
- Lifetime parameters for borrowed data: `Node<'a>`, `Search<'a>`

---

*Convention analysis: 2026-01-31*
