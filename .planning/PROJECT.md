# rust-code-analysis Upstream Contribution

## What This Is

Contributing enhancements to Mozilla's rust-code-analysis library — a Rust library for computing code metrics on source code using tree-sitter parsers. The focus is getting three open PRs merged into upstream while fixing a critical blocking bug.

## Core Value

**Get PRs #1213, #1214, and #1215 merged into Mozilla upstream.** Everything else supports this goal.

## Requirements

### Validated

- ✓ Multi-language code metric computation (12+ languages) — existing
- ✓ Tree-sitter based AST parsing — existing
- ✓ CLI and web interfaces — existing
- ✓ Cyclomatic, Halstead, LOC, Cognitive metrics — existing
- ✓ PR #1210 (Expose inner tree-sitter::Node) — merged
- ✓ PR #1211 (Clippy collapsible_if fixes) — merged

### Active

- [ ] PR #1213: Node API utility methods (`has_ancestor`, `all_occurrences`)
- [ ] PR #1214: Tree-sitter 0.26.3 upgrade (BLOCKED by recursion bug)
- [ ] PR #1215: Complete Kotlin metrics implementation (depends on #1214)
- [ ] Fix `get_nesting_from_map` infinite recursion in mozcpp tests
- [ ] All tests passing locally on all PR branches
- [ ] Rustfmt formatting changes committed

### Out of Scope

- New language support beyond Kotlin — focus on existing PRs first
- CI/CD workflow changes — PR #1208 was closed intentionally
- AST tools module (PR #1209) — was closed, low priority
- Major architectural changes — maintain upstream compatibility

## Context

**Current State (Jan 2026):**
- 3 PRs open against mozilla/rust-code-analysis
- PR #1214 has a critical bug: `get_nesting_from_map` causes infinite recursion when running `check tree-sitter-mozcpp` tests
- Mozilla maintainers (@marco-c, @Luni-4) identified the bug is on rust-code-analysis side
- PR #1215 (Kotlin metrics) depends on PR #1214 being fixed first
- PR #1213 (Node API) is independent and can be merged separately

**Technical Details:**
- Bug location: `get_nesting_from_map` function calling `ts_node_child_with_descendant` recursively
- Kotlin grammar switched from `tree-sitter-kotlin-ng` to `tree-sitter-kotlin-codanna` 0.3.9
- 12 files have local rustfmt changes pending commit

**PR Dependencies:**
```
PR #1213 (Node API) ──────────────────────────────> Merge
PR #1214 (tree-sitter 0.26.3) ──> Fix bug ──────────> Merge
                                      │
PR #1215 (Kotlin metrics) ────────────┴─────────────> Merge (after #1214)
```

## Constraints

- **Upstream Compatibility**: Changes must be acceptable to Mozilla maintainers
- **Rust Edition**: 2024 edition, Rust 1.92.0+
- **Tree-sitter Version**: Upgrading from 0.25.3 to 0.26.3
- **Test Coverage**: All existing tests must pass, especially mozcpp tests

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Switch Kotlin grammar to tree-sitter-kotlin-codanna | tree-sitter-kotlin-ng unmaintained | — Pending |
| Implement all 11 metrics for Kotlin | Complete language support matches other languages | — Pending |
| Use non-recursive stack-based implementation for Node API | Avoid stack overflow on deep ASTs | ✓ Good |
| Separate PRs by feature | Easier review, independent merging | ✓ Good |

---
*Last updated: 2026-01-31 after initialization*
