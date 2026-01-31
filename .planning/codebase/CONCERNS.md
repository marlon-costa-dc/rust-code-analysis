# Codebase Concerns

**Analysis Date:** 2026-01-31

## Tech Debt

**Recursive Function Detection Gap:**
- Issue: Cognitive complexity metric cannot reliably detect recursive functions because call graphs are resolved at runtime, not through static analysis
- Files: `src/metrics/cognitive.rs` (lines 11-18)
- Impact: Cognitive complexity scores may underestimate complexity for recursive code, particularly in C++ where interprocedural analysis is challenging
- Fix approach: Would require implementing language-specific call graph construction or integrating a lightweight interpreter; consider future optimization only if this limitation blocks users

**Macro Handling Incomplete for All Languages:**
- Issue: Cognitive complexity metric macro support marked as TODO for multiple languages (Rust, C++, Kotlin)
- Files: `src/metrics/cognitive.rs` (lines 314, 365, 501)
- Impact: Metrics computed on code with complex macros may be inaccurate or missing complexity contributions from macro bodies
- Fix approach: Implement macro expansion detection per language; requires understanding language-specific macro semantics

**Performance Consideration: Vec vs HashSet in Graph Processing:**
- Issue: Preprocessor graph cycle detection uses Vec with linear search for small strongly-connected components rather than HashSet
- Files: `src/preproc.rs` (lines 115-117, 122-130)
- Impact: Performance acceptable for Firefox codebase (few small SCCs) but could degrade on heavily circular include hierarchies
- Fix approach: Benchmark with larger codebases; defer optimization if not a practical bottleneck for real projects

**Unsafe UTF-8 Conversion Assumptions:**
- Issue: Multiple unwrap() calls on String::from_utf8 without fallback
- Files: `src/c_macro.rs` (lines 35, 50)
- Impact: Crash risk if code contains non-UTF-8 bytes in macro identifiers; silent failure path is incomplete
- Fix approach: Return Result<Option<Vec<u8>>> to gracefully handle invalid UTF-8 sequences, or validate inputs upfront

**Template String Handling Incomplete:**
- Issue: JavaScript/MozJS template string analysis marked TODO; may not correctly handle nested interpolations
- Files: `src/alterator.rs` (lines 94-96)
- Impact: Template strings with replacement expressions may be incorrectly counted or analyzed
- Fix approach: Parse template expression children separately; requires understanding tree-sitter grammar for template nodes

**Include Cycle Self-Inclusion Detection Severity:**
- Issue: Self-inclusion (file includes itself) only generates eprintln warning with no mechanism to escalate or configure
- Files: `src/preproc.rs` (lines 98-99)
- Impact: Silent processing of circular includes may mask build issues; warnings disappear in non-debug output
- Fix approach: Add configurable warning level option; consider returning Result to fail-fast on cycles

**Debug Print Left in Production Code:**
- Issue: Unconditional eprintln!("DEBUG: ...") statement in preprocessor
- Files: `src/preproc.rs` (line 169)
- Impact: Noisy output during preprocessing; suggests incomplete debugging session
- Fix approach: Remove or gate behind debug_assert!/debug_log flag

## Known Bugs

**C++ Macro Parse Error (Issue #1142):**
- Symptoms: Test `test_fn_id_strings` fails with parse error on `nsPrintfCString("%\" PRIi32, lifetime.mTag);`
- Files: `src/c_langs_macros/mod.rs` (lines 43-50)
- Trigger: Any C++ code with printf-style format strings containing PRIxNN macros (standard POSIX inttypes)
- Workaround: Test marked `#[ignore]` to allow CI to pass; issue tracked but not actively resolved
- Root cause: Tree-sitter grammar or preprocessor does not correctly handle macro expansion in format strings

**Ignored Test for CPP Macro Handling:**
- Symptoms: Test marked as ignored with explicit reference to issue #1142
- Files: `src/c_langs_macros/mod.rs` (lines 44-46)
- Impact: Parsing any Firefox code using standard POSIX format macros (common pattern) will silently fail
- Priority: High - blocks accurate analysis of Mozilla's own C++ codebase

## Security Considerations

**Unwrap Chains on Tree-Sitter Nodes:**
- Risk: Multiple consecutive unwrap() calls on Node operations without validation
- Files: `src/metrics/abc.rs` (lines 216-227), multiple other metric files
- Current mitigation: Assumes tree-sitter parser output is always well-formed; valid for internal use but fragile if parser behavior changes
- Recommendations:
  - Add unit tests that intentionally trigger unwrap failures
  - Consider replacing critical paths with `expect("descriptive message")`
  - Document assumptions about node structure invariants

**String Slicing on Byte Boundaries:**
- Risk: Code extracts substrings using byte ranges from text without validating UTF-8 boundaries
- Files: `src/getter.rs` (lines 31-32, 113-114, 182, etc. - widespread pattern)
- Current mitigation: Assumes source files are valid UTF-8; tree-sitter enforces this invariant
- Recommendations:
  - Document UTF-8 requirement in public API
  - Add validation in public entry points

**Unsafe Pattern in Cpp Getter Function Names:**
- Risk: Complex nested Option matching with multiple unwrap paths in function name extraction
- Files: `src/getter.rs` (lines 433-476)
- Current mitigation: Returns None if any extraction fails
- Recommendations: Add integration tests with edge-case C++ syntax (templates, operators, qualified names)

## Performance Bottlenecks

**Large File Complexity - LOC Metrics:**
- Problem: `src/metrics/loc.rs` at 3516 lines is largest single file; difficult to navigate and modify
- Files: `src/metrics/loc.rs`
- Cause: Multiple metric implementations (Sloc, Ploc, Lloc, and more) combined in one file
- Improvement path: Split by metric type (one file per metric class); reduces cognitive load and compile time

**Large File Complexity - C++ Language Module:**
- Problem: `src/languages/language_cpp.rs` at 1328 lines with 300+ enum variants
- Files: `src/languages/language_cpp.rs` (lines 240-310 show only a portion of variants)
- Cause: Exhaustive enumeration of C++ language constructs including Mozilla-specific macros
- Improvement path: Consider generating variant list from grammar; use procedural macros to reduce boilerplate

**Cognitive Metrics Graph Complexity:**
- Problem: `src/metrics/cognitive.rs` at 1979 lines; per-language implementations with duplicated nesting logic
- Files: `src/metrics/cognitive.rs` (lines 312-366, 363-501 show duplication pattern)
- Cause: Language-specific nesting map handling implemented separately for Rust, C++, Kotlin, etc.
- Improvement path: Extract common nesting logic into trait; use composition over duplication

**String Cloning in C Macro Replace:**
- Problem: Macro replacement creates intermediate String allocations in hot path
- Files: `src/c_macro.rs` (lines 35, 50)
- Cause: String::from_utf8 on subslices; could avoid allocation with careful byte validation
- Improvement path: Profile actual memory usage; consider cow::Cow<str> or pre-allocation strategy

## Fragile Areas

**Getter Trait Implementation for JavaScript Variants:**
- Files: `src/getter.rs` (lines 179-205, 248-274, 317-343)
- Why fragile: Three nearly-identical implementations for JavascriptCode, TypescriptCode, TsxCode with hardcoded Mozjs:: references mixed with correct types
- Example fragility: Line 188 and 194 use `Mozjs::Pair` and `Mozjs::VariableDeclarator` instead of Javascript/Typescript variants; works only if tree-sitter produces identical node types
- Safe modification: Extract shared logic into helper function; add tests that verify each language variant produces correct names for anonymous functions
- Test coverage: Limited coverage of edge cases (arrow functions, class methods, getters/setters)

**Preprocessor Graph Cycle Handling:**
- Files: `src/preproc.rs` (lines 105-154)
- Why fragile: SCC replacement creates synthetic PathBuf("") node; dependent code must understand this convention
- Example fragility: Line 162 checks for empty PathBuf to detect synthetic SCC nodes; no type safety
- Safe modification: Create explicit SccNode enum variant instead of empty PathBuf; add documentation of graph invariants
- Test coverage: Lacks integration tests with real circular include patterns from Firefox

**C Macro Keyword Extraction:**
- Files: `src/c_macro.rs` (lines 22-60)
- Why fragile: Triple unwrap() with no validation of identifier boundaries or encoding
- Example fragility: Lines 35, 50 unwrap String::from_utf8 on user code that could be non-UTF-8 binary data
- Safe modification: Return Result; add unit tests with invalid UTF-8, zero-width characters, Unicode edge cases
- Test coverage: Tests only use ASCII; no coverage of multi-byte sequences or invalid encoding

**ABC Metric Space Counting:**
- Files: `src/metrics/abc.rs` (lines 37, 62)
- Why fragile: Declaration vector appends without explicit capacity; min/max initialization to f64::MAX
- Safe modification: Validate space_count never reaches usize::MAX; document initialization invariant
- Test coverage: No tests for degenerate cases (empty files, single-line files)

## Scaling Limits

**Language Support Enumeration:**
- Current capacity: ~12 supported languages with manually-maintained grammars
- Limit: Each new language requires:
  - Tree-sitter grammar (external dependency)
  - Language enum variant (generated or manual)
  - Getter trait implementation (250+ lines per language)
  - Metric implementations per language (duplicated logic in cognitive, abc, etc.)
- Scaling path:
  - Implement code generation for enum variants from grammar metadata
  - Use trait with default implementations to reduce per-language boilerplate
  - Consider tree-sitter plugin model for community-contributed languages

**Metric Enum Variants for C++:**
- Current capacity: 300+ enum variants in Cpp language module
- Limit: Breaks at ~500 variants (rough Rust enum size limit); each variant impacts compilation time and binary size
- Scaling path: Generate Cpp variant list from tree-sitter grammar file; use build script (build.rs)

**Graph Size in Preprocessor:**
- Current capacity: Tested on Firefox (few SCCs, small components); no benchmarks provided
- Limit: Strongly-connected component detection uses Kosaraju algorithm (O(V+E)); dominates time for large include trees
- Scaling path: Profile on large monorepo (10k+ files); consider incremental SCC caching

## Dependencies at Risk

**tree-sitter Version Pinning:**
- Risk: Recent commit "Revert tree-sitter 0.26.3 update (#1212)" suggests breaking changes in tree-sitter API
- Impact: tree-sitter minor version bumps can break code due to Node API changes
- Migration plan: Maintain compatibility layer for node operations; add integration tests that detect API breakage

**No Dependency on Standard Metrics Libraries:**
- Risk: All metrics implemented from scratch; no validation against established tools (Sonarqube, CodeClimate)
- Impact: Metric calculations may diverge from industry standard; no cross-validation
- Mitigation: Compare results against well-known codebases; document metric definitions clearly

## Missing Critical Features

**No Test Coverage for Large File Handling:**
- Problem: No tests for files >1MB or >100k lines
- Blocks: Cannot confidently use on real-world large codebases (Firefox, Linux kernel)
- Priority: High - marketing claims analysis on "large-scale" code but no proof of capability

**No Streaming or Incremental API:**
- Problem: All metrics require loading entire file into memory; no support for streaming analysis
- Blocks: Cannot analyze files larger than available RAM; no support for continuous integration on resource-constrained systems
- Priority: Medium - may limit adoption for embedded systems or large-scale CI pipelines

**No Configuration for Metric Calculation Parameters:**
- Problem: All metric thresholds hardcoded (e.g., nesting depth penalties in cognitive complexity)
- Blocks: Cannot customize metrics for different coding standards or organizations
- Priority: Low - acceptable for Firefox; may limit general-purpose adoption

## Test Coverage Gaps

**Preprocessor Circular Include Handling:**
- What's not tested: Real-world circular include patterns from Firefox; synthetic node behavior
- Files: `src/preproc.rs` (lines 105-154)
- Risk: Changes to graph handling could silently break preprocessor without test detection
- Priority: High - critical for C/C++ analysis

**C Macro Keyword Extraction Edge Cases:**
- What's not tested: Non-UTF-8 bytes in macro names; zero-width characters; very long macros (>2KB); predefined macro interactions
- Files: `src/c_macro.rs` (lines 22-60)
- Risk: Unwrap failures could crash analysis on edge-case code
- Priority: Medium - impacts reliability on diverse codebases

**Cognitive Complexity per Language:**
- What's not tested: Recursive functions; macros generating control flow; nested lambdas in JavaScript
- Files: `src/metrics/cognitive.rs` (multiple language implementations)
- Risk: Metric accuracy cannot be verified; TODO markers indicate incomplete implementation
- Priority: High - core metric used for code health assessment

**ABC Metric Space Transitions:**
- What's not tested: Functions with no statements; transitions between function types; nested closures
- Files: `src/metrics/abc.rs`
- Risk: Edge cases could produce incorrect magnitude scores
- Priority: Medium - metric used for complexity analysis

**Getter Function Name Extraction:**
- What's not tested: C++ operator overloads; template functions; qualified names with namespaces; anonymous nested classes
- Files: `src/getter.rs` (lines 433-476 for Cpp, similar for other languages)
- Risk: Complex language constructs silently return wrong names
- Priority: Medium - function identification is critical for function-level metrics

---

*Concerns audit: 2026-01-31*
