# Testing Patterns

**Analysis Date:** 2026-01-31

## Test Framework

**Runner:**
- cargo test
- Insta snapshot testing library: `insta = { version = "1.46.1", features = ["yaml", "json", "redactions"] }`
- Pretty assertions: `pretty_assertions = "^1.3"`

**Config:**
- `Cargo.toml` dev-dependencies section lines 39-41
- Optimized insta performance: `[profile.dev.package.insta] opt-level = 3` (line 43-44)
- Pre-commit hook runs tests: `cargo test` (`.pre-commit-config.yaml` line 40-44)

**Run Commands:**
```bash
cargo test              # Run all tests
cargo test --lib       # Run library tests only
cargo test --test '*'  # Run integration tests
cargo test -- --nocapture  # Show output during test runs
```

## Test File Organization

**Location:**
- Integration tests: `tests/` directory (`tests/serde_test.rs`, `tests/pdf_js_test.rs`, `tests/deepspeech_test.rs`)
- Unit tests: Inline in source files with `#[cfg(test)] mod tests { ... }`
- Test utilities: `tests/common/mod.rs` for shared test functions

**Naming:**
- Integration tests: `{name}_test.rs` (e.g., `serde_test.rs`, `pdf_js_test.rs`)
- Test functions: `#[test] fn test_{feature_name}()`
- Pattern: `test_serde()` in `serde_test.rs`, `test_pdfjs()` in `pdf_js_test.rs`

**Structure:**
```
tests/
├── common/
│   └── mod.rs          # Shared test utilities: compare_rca_output_with_files()
├── serde_test.rs       # Single test function: test_serde()
├── pdf_js_test.rs      # Single test function: test_pdfjs()
├── deepspeech_test.rs  # Single test function: test_deepspeech()
└── repositories/       # Test fixtures (code samples to analyze)
    └── rca-output/
        └── snapshots/  # Expected snapshot outputs
```

## Test Structure

**Suite Organization:**
```rust
// Pattern from tests/serde_test.rs (lines 1-8)
mod common;

use common::compare_rca_output_with_files;

#[test]
fn test_serde() {
    compare_rca_output_with_files("serde", &["*.rs"], &[]);
}
```

**Unit Test Pattern (from src/c_macro.rs lines 68-101):**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
        let mut mac = HashSet::new();
        mac.insert("abc".to_string());

        // Setup fixtures
        assert!(replace(b"def ghi jkl", &mac).is_none());
        assert_eq!(
            b"$$$ def ghi jkl".to_vec(),
            replace(b"abc def ghi jkl", &mac).unwrap()
        );
        // ... more assertions
    }
}
```

**Patterns:**
- Setup: Create test data (HashSet with macros, test input bytes)
- Execute: Call function under test
- Assert: Compare actual vs expected output

## Snapshot Testing with Insta

**Configuration (from tests/common/mod.rs lines 42-62):**
```rust
insta::with_settings!({
    snapshot_path => Path::new(SNAPSHOT_PATH)
        .join(path.strip_prefix(Path::new(REPO)).unwrap())
        .parent()
        .unwrap(),
    prepend_module_to_snapshot => false,
    sort_maps => true,
}, {
    insta::assert_yaml_snapshot!(
        path.file_name().unwrap().to_string_lossy().as_ref(),
        funcspace_struct,
        {
            // Round floating point values to three decimal places
            ".spaces[].**.metrics.*.*" => insta::rounded_redaction(3),
            ".metrics.*.*" => insta::rounded_redaction(3),
            // Redact away the name since paths are different
            ".name" => "[filepath]",
        }
    );
});
```

**Snapshot Features:**
- YAML snapshot format: Snapshots stored as `.yml` files
- Automatic redaction: Floating point numbers rounded to 3 decimals
- Path normalization: File paths redacted to `[filepath]` for cross-platform consistency
- Sorted maps: JSON/YAML maps sorted for deterministic comparisons
- No module prefix: Cleaner snapshot file names

## Mocking

**Framework:** No explicit mocking framework detected (Rust test defaults)

**Patterns:**
- Mocking via trait bounds: Generic `T: ParserTrait` allows swapping implementations
- Test fixtures: Real code samples in `tests/repositories/` directory
- Configuration objects: `Config { language: None }` in `tests/common/mod.rs` line 71

**What to Mock:**
- Language parsers: `ParserTrait` implementations can be mocked by creating test types
- File I/O: Use in-memory buffers instead of actual files where possible

**What NOT to Mock:**
- Actual tree-sitter parsing: Tests use real parsers on real source code
- Metric calculations: Full computation pipeline tested end-to-end
- Snapshot comparisons: Real outputs compared against snapshots

## Fixtures and Factories

**Test Data:**
```rust
// From src/c_macro.rs test (lines 72-99)
let mut mac = HashSet::new();
mac.insert("abc".to_string());
mac.insert("z9_".to_string());
```

**Snapshot Fixtures:**
- Repository code samples: `tests/repositories/serde/`, `tests/repositories/pdf.js/`
- Expected outputs: `tests/repositories/rca-output/snapshots/`
- File pattern: Snapshots match source file structure

**Location:**
- Unit test fixtures: Inline in `#[cfg(test)]` modules (`src/c_macro.rs`, `src/metrics/abc.rs`)
- Integration test fixtures: `tests/repositories/` for code samples
- Snapshots: `tests/repositories/rca-output/snapshots/` for expected outputs

## Coverage

**Requirements:** Not enforced in codebase (no coverage config found)

**View Coverage:**
```bash
# Using tarpaulin (if installed)
cargo tarpaulin --verbose

# Using llvm-cov (if installed)
cargo llvm-cov
```

**Current Coverage:**
- Unit tests: Present in metrics modules (`src/metrics/abc.rs`, `src/metrics/cognitive.rs`, etc.)
- Integration tests: Three main test suites (serde, pdf.js, deepspeech)
- Test count: ~10+ unit tests detected with `#[test]` attribute
- Coverage gaps: No coverage enforcement, gaps unknown

## Test Types

**Unit Tests:**
- Scope: Individual functions in isolation
- Approach: Test bytes replacement, macro detection, metric calculations
- Examples: `test_replace()` in `c_macro.rs`, metric validation tests in `metrics/` modules
- Setup/Teardown: Minimal - mostly inline fixture creation

**Integration Tests:**
```rust
// From tests/common/mod.rs lines 68-93
pub fn compare_rca_output_with_files(
    repo_name: &str,
    include: &[&str],
    exclude: &[&str]
) {
    // 1. Iterate files matching patterns
    // 2. Parse with language detection
    // 3. Compute all metrics
    // 4. Compare against snapshots
    // 5. Fail if snapshot differs
}
```
- Scope: Complete code analysis pipeline (read file → detect language → compute metrics)
- Approach: Real code repositories analyzed, results compared to snapshots
- Runners: Concurrent with 4 jobs: `ConcurrentRunner::new(num_jobs, act_on_file)`
- Error handling: Exit with code 1 on failure (line 91)

**E2E Tests:**
- Framework: Not explicitly used
- Implicit E2E: Integration tests cover full workflows
- Example: `test_serde()` analyzes entire Serde repository

## Common Patterns

**Assertion Patterns:**
```rust
// Equality assertions
assert_eq!(expected, actual);

// Option assertions
assert!(result.is_none());
assert!(result.is_some());

// Snapshot assertions
insta::assert_yaml_snapshot!(name, value, { redactions });
```

**Async Testing:**
- Not detected - no async/await in tests
- Concurrent testing via `ConcurrentRunner` (thread pool, not async)

**Error Testing:**
```rust
// From c_macro.rs test: Testing None case (error condition)
assert!(replace(b"def ghi jkl", &mac).is_none());
```

**Parallel Test Execution:**
- Integration tests use 4 worker threads: `ConcurrentRunner::new(4, act_on_file)`
- Each file processed independently: `act_on_file(path, cfg)`

## Test Execution Flow

**Integration Test Flow (tests/common/mod.rs):**

1. **Setup Phase:**
   - Create config: `Config { language: None }`
   - Build glob sets for include/exclude patterns
   - Create `FilesData` with paths and patterns

2. **Execution Phase:**
   - `ConcurrentRunner::new(4, act_on_file).run(cfg, files_data)`
   - For each file: Read with `read_file_with_eol()`, detect language
   - Compute function spaces: `get_function_spaces()`
   - Snapshot comparison: Redact paths and floats, assert YAML matches

3. **Result Phase:**
   - Exit code 0 if all snapshots match
   - Exit code 1 if any snapshot differs or error occurs

## Known Test Issues

**Skipped Tests:**
- `pdf_js_test.rs`: 128 files excluded due to parse errors (see `pdf_js_test.rs` lines 9-128)
- Reason: "Parse error (see issue: https://github.com/mozilla/rust-code-analysis/issues/1143)"
- Impact: Some JavaScript files not tested until issues resolved

**Test Gaps:**
- No property-based testing (no quickcheck/proptest usage)
- No fuzzing detected
- Limited error case coverage (mostly happy path)

---

*Testing analysis: 2026-01-31*
