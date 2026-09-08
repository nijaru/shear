# Dependency strategy

## Decision

Keep Rust + tree-sitter and own a small structural-normalization engine. Reuse established language-specific infrastructure where it solves a concrete problem; do not rebuild compilers for independence's sake or embed a complete linter without need.

Current direct dependencies:

| Dependency | Owner/responsibility |
|---|---|
| `tree-sitter` + Python grammar | Parse source and provide syntax/source ranges |
| `usage-rs` | CLI declarations, parsing, help |
| `ignore` | Ignore-aware file discovery |
| `similar` | Unified diffs |
| `tempfile` | Temporary files and replacement |
| `anyhow` | Application error context |

`Cargo.lock` records exact resolved versions. No runtime model SDK or network service is required. Keep dependency changes evidence-driven and use Cargo to manage them.

## Ruff

Ruff has a Python-specific parser/AST, semantic analysis, source-trivia utilities, code generation, diagnostics/fixes, and a separate formatter. It is not merely textual pattern matching and does not use tree-sitter as its Python frontend.

Use its CLI for behavior comparisons and formatter compatibility. Study rule safety and regression cases. Our initial two rules overlap Ruff; that is acceptable and documented, not a reason to halt the product or claim superiority.

Do not embed all of `ruff_linter` today. Its Python-specific model would create integration and representation costs. The internal crates are published, but their Rust APIs have no stability guarantees; deliberate reuse would require version maintenance and a concrete benefit. A future Python backend can adopt its parsing/semantic infrastructure if doing so is simpler than implementing equivalent facts ourselves. Tree-sitter is not a mandatory lowest common denominator.

References inspected September 8, 2026:

- Architecture: https://github.com/astral-sh/ruff/blob/main/CONTRIBUTING.md#project-structure
- Crate API policy: https://docs.astral.sh/ruff/versioning/#crate-versioning
- Licensing and third-party notices: https://github.com/astral-sh/ruff/blob/main/LICENSE

## What Shear owns

Language-specific applicability and normalization policy; deterministic pass ordering; surgical edit validation; convergence; concise explanations; formatter-style write/check/diff behavior. Add control-flow, binding, effect, or type facts only where a transformation needs them. Shared infrastructure should emerge from actual backends, not a universal semantic AST designed in advance.

## Licensing

Shear is publicly visible but intentionally has no project license yet. Do not add one without a user decision and do not describe it as open source. Public visibility alone is not a general grant to reuse or redistribute.

Ruff's principal license is MIT with additional third-party notices. Reusing eligible code is possible while retaining applicable notices; it does not itself require choosing MIT for Shear. Dependencies retain their own licenses regardless of Shear's status. Audit actual copied code and distributed dependencies before release; source inspection is not a completed distribution-license audit.
