# Requirements: rust-code-analysis Upstream Contribution

**Defined:** 2026-01-31
**Core Value:** Get PRs #1213, #1214, and #1215 merged into Mozilla upstream

## v1 Requirements

Requirements for getting all open PRs merged.

### Bug Fixes

- [x] **BUG-01**: Fix `get_nesting_from_map` infinite recursion in mozcpp tests
- [x] **BUG-02**: Ensure all tests pass on pr/tree-sitter-0.26 branch

### PR #1213 - Node API Extensions

- [x] **API-01**: `has_ancestor()` method detects if any ancestor matches predicate
- [x] **API-02**: `find_all()` method collects all descendant nodes matching predicate
- [x] **API-03**: Both methods use non-recursive stack-based implementation
- [ ] **API-04**: PR #1213 merged into upstream

### PR #1214 - Tree-sitter Upgrade

- [ ] **DEPS-01**: Update tree-sitter from 0.25.3 to 0.26.3
- [ ] **DEPS-02**: Update all grammar dependencies to compatible versions
- [ ] **DEPS-03**: Switch Kotlin grammar from tree-sitter-kotlin-ng to tree-sitter-kotlin-codanna
- [ ] **DEPS-04**: PR #1214 merged into upstream

### PR #1215 - Kotlin Metrics

- [ ] **KOTLIN-01**: Implement Checker trait for KotlinCode
- [ ] **KOTLIN-02**: Implement Getter trait with SpaceKind and Halstead classification
- [ ] **KOTLIN-03**: Implement all 11 metrics (ABC, Cognitive, Cyclomatic, Exit, Halstead, LOC, NArgs, NOM, NPA, NPM, WMC)
- [ ] **KOTLIN-04**: PR #1215 merged into upstream

### Code Quality

- [x] **QUALITY-01**: Commit rustfmt formatting changes
- [x] **QUALITY-02**: All clippy warnings resolved
- [x] **QUALITY-03**: All tests passing on master branch

## v2 Requirements

Deferred to future work after PRs are merged.

### Additional Languages

- **LANG-01**: Add metrics support for additional languages
- **LANG-02**: Update remaining tree-sitter grammars

### API Improvements

- **API-05**: AST tools module with macro-based traversal helpers

## Out of Scope

| Feature | Reason |
|---------|--------|
| CI/CD workflows | PR #1208 was intentionally closed |
| Major architectural changes | Maintain upstream compatibility |
| New language support (beyond Kotlin) | Focus on existing PRs first |
| AST tools module | PR #1209 was closed, low priority |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| BUG-01 | Phase 1 | Complete |
| BUG-02 | Phase 1 | Complete |
| QUALITY-01 | Phase 2 | Complete |
| QUALITY-02 | Phase 2 | Complete |
| QUALITY-03 | Phase 2 | Complete |
| API-01 | Phase 3 | Complete |
| API-02 | Phase 3 | Complete |
| API-03 | Phase 3 | Complete |
| API-04 | Phase 3 | Pending |
| DEPS-01 | Phase 4 | Pending |
| DEPS-02 | Phase 4 | Pending |
| DEPS-03 | Phase 4 | Pending |
| DEPS-04 | Phase 4 | Pending |
| KOTLIN-01 | Phase 5 | Pending |
| KOTLIN-02 | Phase 5 | Pending |
| KOTLIN-03 | Phase 5 | Pending |
| KOTLIN-04 | Phase 5 | Pending |

**Coverage:**
- v1 requirements: 17 total
- Mapped to phases: 17
- Unmapped: 0 ✓

---
*Requirements defined: 2026-01-31*
*Last updated: 2026-02-01 (Phase 3 Complete)*
