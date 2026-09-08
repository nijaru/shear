# Sources and provenance

## Product authority

The user supplied the original `structural-formatter-handoff.md` and explicitly clarified the current product during September 8 development:

- Deterministic, local, cross-language structural formatting.
- Astra develops and invokes Shear; no inference inside Shear.
- Rust + tree-sitter, with source-language choice independent of implementation language.
- Transformation is the product; metrics support it, not replace it.

The early repository handoff mistakenly recast this as Go plus runtime model proposals. Git history preserves that planning; it is superseded, not implementation evidence. An uncommitted Go prototype was set aside before the Rust implementation.

## Technical references

- Tree-sitter Rust API: https://docs.rs/tree-sitter/latest/tree_sitter/
- Python grammar: https://github.com/tree-sitter/tree-sitter-python
- usage-rs CLI: https://usage.jdx.dev/rust/
- Python control-flow semantics: https://docs.python.org/3/reference/compound_stmts.html
- Python expression/Boolean semantics: https://docs.python.org/3/reference/expressions.html
- Native Go modernization precedent: https://go.dev/blog/gofix
- Ruff rule reference (compare overlapping fixes): https://docs.astral.sh/ruff/rules/
- Clippy: https://doc.rust-lang.org/clippy/
- OpenRewrite: https://github.com/openrewrite/rewrite
- ast-grep: https://github.com/ast-grep/ast-grep
- Ripwire analysis reference from the original concept: https://github.com/redhat-et/ripwire
- FFmpeg: https://ffmpeg.org/ffmpeg.html
- ffprobe: https://ffmpeg.org/ffprobe.html
- VHS: https://github.com/charmbracelet/vhs

References establish capabilities or comparison targets, not Shear correctness or novelty. `Cargo.lock` records selected versions; source/tests and `BUILD_LOG.md` record actual implementation evidence.

## Event basis

The user-provided participant guide supplies the September 8, 2026 Pacific schedule: hacking 10:30 AM–5:30 PM, public repository and accessible one-minute video, four equally weighted categories (Astra in development, Astra in project, live demo, technical implementation), and five finalists with three-minute demos plus two-minute questions. Its stage schedule lists both 6:45 and 7:00 PM; confirm separately.

Public event reference: https://cerebralvalley.ai/e/openai-gpt-6-astra-sf . This document does not claim independent verification of the guide or that agent invocation alone satisfies the organizers' “Astra in project” criterion. Do not distort the product to guess at that criterion; clarify with organizers if needed. Private access details are not publication material.

## Evidence not yet established

The checked CPython example is documented independently in `SELECTED_EXAMPLE.md`, with upstream notices in `demo/evidence/cpython/`. This establishes the recorded module-test result, not cross-language rewrite parity, speed superiority, exhaustive equivalence, a final video, or submission. Preserve upstream attribution and licenses in published excerpts; do not claim a Shear license absent an explicit choice.
