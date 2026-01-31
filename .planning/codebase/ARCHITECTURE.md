# Architecture

**Analysis Date:** 2026-01-31

## Pattern Overview

**Overall:** Multi-language code analysis library using tree-sitter parsing with generic metric computation across supported languages.

**Key Characteristics:**
- **Language-agnostic metric system** - Trait-based design allows metrics to be computed on any supported language
- **Tree-sitter abstraction layer** - Unified AST parsing interface for 12+ programming languages
- **Generic parser container** - `Parser<T>` generic wrapper enabling language-specific implementations
- **Modular metrics** - Each metric (cyclomatic complexity, Halstead, LOC, etc.) is independently computed
- **Multi-threaded file processing** - Concurrent analysis of source files with configurable thread pools

## Layers

**Language Support Layer:**
- Purpose: Define and enumerate all supported programming languages
- Location: `src/langs.rs`, `src/languages/`, `src/macros.rs`
- Contains: Language enums (Rust, Python, JavaScript, C++, Java, Kotlin, TypeScript, TSX, Go, C#, CSS, HTML, MozJS, Preproc, CComment), tree-sitter bindings, language metadata
- Depends on: tree-sitter crates, language-specific enum definitions (auto-generated from `mk_langs!` macro)
- Used by: Parser, Checker, all metric calculations

**AST & Parsing Layer:**
- Purpose: Parse source code into abstract syntax trees and provide traversal utilities
- Location: `src/node.rs`, `src/ast.rs`, `src/parser.rs`, `src/asttools.rs`
- Contains: `Node<'a>` (wrapper around tree-sitter nodes), `Tree` (wrapper around tree-sitter AST), `Parser<T>` generic container, AST serialization structures (`AstNode`, `AstResponse`, `AstPayload`)
- Depends on: tree-sitter, tree-sitter language libraries
- Used by: Metrics, analyzers, checker, getter modules

**Trait System:**
- Purpose: Define generic interfaces that languages must implement for metric computation
- Location: `src/traits.rs`
- Contains: `ParserTrait` (main interface with associated metric types), `LanguageInfo`, `Callback`, `Search<'a>` (node traversal), `LanguageInfo`
- Depends on: All metric modules, all language-specific implementations
- Used by: Parser, concurrent runners, all analysis functions

**Language Analysis Layer:**
- Purpose: Detect language-specific syntactic structures (functions, classes, operators, comments)
- Location: `src/checker.rs`, `src/getter.rs`, `src/macros.rs`, `src/languages/`
- Contains: `Checker` trait implementations (detect functions, closures, operators), `Getter` trait implementations (extract function names, space kinds, operator types), macro-based implementations
- Depends on: Language-specific enums, Node types
- Used by: Metrics, space detection, function extraction

**Metrics Computation Layer:**
- Purpose: Calculate code quality and complexity metrics on parsed code
- Location: `src/metrics/`, `src/spaces.rs`
- Contains: Cyclomatic Complexity, Halstead Metrics, Lines of Code (LOC, PLOC, LLOC, CLOC), NOM (Number of Functions), NARGS, NEXITS, ABC metrics, Cognitive complexity, MI (Maintainability Index), WMC, NPA, NPM
- Depends on: Parser, Checker, Getter, Space definitions
- Used by: Analysis pipeline, output formatters

**Output & Serialization:**
- Purpose: Format and serialize analysis results
- Location: `src/output/`, `src/spaces.rs` (CodeMetrics)
- Contains: `DumpMetrics` (serialize metrics to JSON/YAML), `DumpOps` (operator sequences), `Dump` (general output), `CodeMetrics` struct (aggregates all metrics), `SpaceKind` enum
- Depends on: serde, metrics modules, termcolor for colored output
- Used by: CLI, web interface, external consumers

**File Processing Layer:**
- Purpose: Manage concurrent file analysis and build the analysis pipeline
- Location: `src/concurrent_files.rs`, `src/tools.rs`
- Contains: `ConcurrentRunner` (multi-threaded orchestrator), `FilesData` configuration, job queue implementation using crossbeam channels, worker thread pools
- Depends on: walkdir, globset, crossbeam, all analysis modules
- Used by: CLI interface

**Support Modules:**
- `src/comment_rm.rs` - Comment removal and processing
- `src/preproc.rs`, `src/c_macro.rs`, `src/c_langs_macros/` - C/C++ preprocessor directive handling
- `src/alterator.rs` - Code transformation/alteration (comment removal, macro expansion)
- `src/find.rs` - Pattern finding in AST
- `src/function.rs` - Function span detection and extraction
- `src/count.rs` - Generic node counting
- `src/ops.rs` - Operator extraction and analysis
- `src/spaces.rs` - Code space (function/class/module) grouping and metrics aggregation
- `src/tools.rs` - Utility functions (file I/O, color output, path utilities)

## Data Flow

**Source Code → Analysis → Metrics:**

1. **File Input** (`concurrent_files.rs`)
   - Walk directory tree, filter by glob patterns
   - Send file paths to worker thread queue

2. **Language Detection** (`tools.rs`, `langs.rs`)
   - Guess language from file extension or content
   - Load appropriate tree-sitter language

3. **Parsing** (`parser.rs`, `node.rs`)
   - Create `Parser<T>` instance with source code
   - Parse with tree-sitter → `Tree` → `Node` hierarchy

4. **Preprocessing** (`preproc.rs`, `c_macro.rs`)
   - For C/C++, extract macro definitions and preprocessor directives
   - Store in `PreprocResults` for reference

5. **Analysis** (metrics modules)
   - Walk AST with cursor, identifying function/class boundaries
   - Compute metrics per space (function, class, file)
   - Aggregate to file-level metrics

6. **Output** (`output/`)
   - Serialize `CodeMetrics` to JSON/YAML
   - Print formatted results to stdout or file

**State Management:**

- **Immutable AST**: Once parsed, tree-sitter trees are read-only
- **Mutable metrics**: `Stats` structs accumulated during tree traversal
- **Thread-safe sharing**: `Arc<Mutex<T>>` for concurrent metric aggregation
- **Job queue**: crossbeam unbounded channels distribute work across threads

## Key Abstractions

**Parser<T>:**
- Purpose: Generic container for language-specific analysis
- Examples: `RustParser`, `PythonParser`, `JavascriptParser`, etc.
- Pattern: Monomorphization at compile time - each language instantiates full Parser<LanguageCode>
- Trait bounds: Requires T to implement LanguageInfo + 13 metric traits + Alterator/Checker/Getter
- Lazy evaluation: Metrics only computed if requested via `Metrics` callback

**Node<'a>:**
- Purpose: Lifetime-scoped wrapper around tree-sitter nodes
- Pattern: Thin wrapper delegating to OtherNode<'a>, exposing traversal methods
- Key methods: `kind()`, `child_by_field_name()`, `start_byte()/end_byte()`, `utf8_text()`
- Used in: All metric computation, AST traversal, space detection

**CodeMetrics:**
- Purpose: Aggregated results of all metrics for a code space
- Location: `src/spaces.rs`
- Contains: `SpaceKind`, `Stats` for each metric type, source location info
- Serialization: Derives Serialize for JSON/YAML output

**Callback trait:**
- Purpose: Allow configuration objects to defer computation
- Pattern: Config holds Callback::Cfg, library calls Callback::call<T> with parser
- Examples: `Metrics`, `MetricsCfg`, `Count`, `CountCfg`
- Allows: Deferred initialization, metric selection, aggregation strategies

## Entry Points

**Library Entry (`src/lib.rs`):**
- Location: Re-exports all public API
- Exports: `Parser<T>`, metrics traits, output structs, `LANG` enum, `action()`, `preprocess()`
- Public API: For downstream libraries and web service

**CLI Entry (`rust-code-analysis-cli/src/main.rs`):**
- Location: Command-line tool implementation
- Triggers: Command-line arguments (clap parser)
- Responsibilities: Config parsing, file discovery, output format selection, concurrent execution
- Key functions: `act_on_file()`, `main()`, format handling

**Web Entry (`rust-code-analysis-web/src/lib.rs`, `/web/server.rs`):**
- Location: HTTP API server implementation
- Triggers: HTTP requests (POST /ast, /metrics, etc.)
- Responsibilities: Request parsing, parser instantiation, response serialization
- Endpoints: AST visualization, metrics computation, comment analysis

**Parser instantiation (`parser.rs`):**
- Function: `Parser::<T>::new(code, path, preproc)`
- Creates: Tree-sitter parser, AST, language-specific state
- Returns: Ready-to-analyze `Parser<T>` instance

## Error Handling

**Strategy:** Result types for I/O, unwrap for parsing failures

**Patterns:**
- File I/O: `std::io::Result<T>` propagated through `?` operator
- Parsing: `Parser::new()` returns Option (panics on failure), assumes valid UTF-8
- Concurrent: `ConcurrentErrors` for channel send failures, logged to stderr
- Metrics: Graceful degradation (unknown metrics return Unknown variant)
- Macro processing: Validation in `preproc.rs`, skips on error
- AST building: Returns `Option<AstNode>` if errors in traversal

## Cross-Cutting Concerns

**Logging:**
- Error output to stderr via `eprintln!` in concurrent workers
- No structured logging framework; debug output via Display/Debug traits
- Web interface has access to detailed error messages

**Validation:**
- Language detection heuristics in `tools.rs::guess_language()`
- UTF-8 validation in Node methods (returns Option)
- Macro syntax validation in `preproc.rs`
- File extension matching in `langs.rs` via LANG enum

**Authentication:**
- Not applicable - library operates on local code
- Web service has no built-in auth (expected to be behind HTTP gateway)

**Comment Handling:**
- Tree-sitter includes comment nodes in AST
- `comment_rm.rs` provides removal option
- `AstPayload.comment` flag controls inclusion in AST export
- CLOC metric explicitly counts comment nodes

---

*Architecture analysis: 2026-01-31*
