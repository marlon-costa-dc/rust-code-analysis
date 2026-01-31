# External Integrations

**Analysis Date:** 2026-01-31

## APIs & External Services

**Not detected.**

This is a standalone code analysis library with no external API dependencies. It does not integrate with third-party services, cloud providers, or remote APIs.

## Data Storage

**Databases:**
- Not used - The library operates entirely in-memory on source code input
- No persistent storage layer
- No ORM or database client library in dependencies

**File Storage:**
- Local filesystem only
- Reading source code files via `walkdir` 2.3 for directory traversal
- Output formats: JSON, YAML, CBOR, plain text (all written locally)

**Caching:**
- No caching layer implemented
- Each analysis run is independent

## Authentication & Identity

**Auth Provider:**
- Not applicable - No authentication required
- Standalone library and tools with no user management

## Monitoring & Observability

**Error Tracking:**
- Not detected - No integration with error tracking services

**Logs:**
- Terminal output only via `termcolor` 1.2
- Errors printed to stderr
- CLI and web server use `println!` and `eprintln!` macros
- No structured logging framework (e.g., `log`, `tracing` crates not used in core library)

## CI/CD & Deployment

**Hosting:**
- Self-hosted (user responsible for deployment)
- Can be deployed as:
  - Standalone CLI binary: `rust-code-analysis-cli`
  - HTTP service: `rust-code-analysis-web` (Actix-web server)
  - Library: Embedded in Rust projects via `rust-code-analysis` crate

**CI Pipeline:**
- GitHub Actions via `.taskcluster.yml` (Taskcluster integration, not GitHub Actions)
- Automated via Mozilla's TaskCluster infrastructure
- Dependent updates: Managed by GitHub Dependabot (`dependabot.yml`)
  - Weekly cargo updates across all workspace members
  - Separate tracking for: root, tree-sitter-mozcpp, tree-sitter-mozjs, tree-sitter-preproc, tree-sitter-ccomment, enums

**Pre-commit Hooks (local):**
- Format check: `cargo fmt -- --check --verbose` (currently disabled - "FIXME: Uncomment when fmt is fixed")
- Linting: `cargo clippy --all-targets --all -- -D warnings`
- Dependency check: `cargo +nightly udeps --all-targets`
- Testing: `cargo test`

## Environment Configuration

**Required env vars:**
- None detected - All configuration via CLI arguments

**Web server configuration (via CLI args):**
- `--host` - Server address (default: 127.0.0.1)
- `--port` - Server port (default: 8080)
- `--threads` - Worker thread count (default: 4)

**CLI tool configuration (via CLI args):**
- Input file/directory paths
- Output format: `--output-format` (json, yaml, cbor, plaintext)
- Language specification: `--language`
- Metrics selection: `--metrics`, `--functions`, `--comments`, `--spaces`

**Secrets location:**
- Not applicable - No secrets management

## Webhooks & Callbacks

**Incoming:**
- Not detected - Web service is request-response only, no webhook endpoints

**Outgoing:**
- Not detected - Library does not make external HTTP calls

## Web Service Endpoints

**HTTP API (`rust-code-analysis-web`):**

- `POST /ast` - Abstract Syntax Tree parsing
  - Content types: `application/json` (JSON payload) or `application/octet-stream` (raw code)
  - Request: File name, code content, analysis options
  - Response: JSON AST structure with node types, spans, children

- `POST /metrics` - Code metrics computation
  - Content types: `application/json` (JSON payload) or `application/octet-stream` (raw code)
  - Metrics: Cyclomatic complexity, cognitive complexity, Halstead metrics, LOC variants, maintainability index, NOM, NEXITS, ABC metrics
  - Response: JSON metrics for file and nested scopes (functions, classes, etc.)

- `POST /comment` - Comment removal
  - Content types: `application/json` or `application/octet-stream`
  - Response: Source code with all comments removed

- `POST /function` - Function/method extraction
  - Content types: `application/json` or `application/octet-stream`
  - Response: JSON array of function spans (name, start_line, end_line)

- `GET /ping` - Health check
  - Response: Empty 200 OK

**Response format:**
- Always JSON for structured responses
- Binary octet-stream for raw code output
- Max request payload: 4 MB (hardcoded: `1024 * 1024 * 4`)
- Error responses include request ID and error message

## Language Support

**Supported for analysis:**
- C++ / C (via tree-sitter-c, tree-sitter-mozcpp for Mozilla extensions)
- C# (via tree-sitter-c-sharp)
- CSS (via tree-sitter-css)
- Go (via tree-sitter-go)
- HTML (via tree-sitter-html)
- Java (via tree-sitter-java 0.23.5)
- JavaScript (via tree-sitter-javascript 0.25.0)
- JavaScript (Mozilla/SpiderMonkey extensions via tree-sitter-mozjs)
- Kotlin (via tree-sitter-kotlin-codanna 0.3.9)
- Python (via tree-sitter-python 0.25.0)
- Rust (via tree-sitter-rust 0.24.0)
- TypeScript / TSX (via tree-sitter-typescript 0.23.2)

Language detection: Automatic via file extension in web service and CLI.

---

*Integration audit: 2026-01-31*
