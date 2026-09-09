# Sources and provenance

## Product constraints

The product was defined during September 8, 2026 development:

- Deterministic, local, cross-language structural formatting.
- Astra and other agents develop and invoke Shear; no inference runs inside Shear.
- Rust + tree-sitter, with source-language choice independent of the implementation language.
- Transformation is the product; metrics support it, not replace it.

## Technical references

- Tree-sitter Rust API: https://docs.rs/tree-sitter/latest/tree_sitter/
- Python grammar: https://github.com/tree-sitter/tree-sitter-python
- JavaScript grammar: https://github.com/tree-sitter/tree-sitter-javascript
- usage-rs CLI: https://usage.jdx.dev/rust/
- Python control-flow semantics: https://docs.python.org/3/reference/compound_stmts.html
- Python expression/Boolean semantics: https://docs.python.org/3/reference/expressions.html
- Native Go modernization precedent: https://go.dev/blog/gofix
- Ruff rule reference (compare overlapping fixes): https://docs.astral.sh/ruff/rules/
- Clippy: https://doc.rust-lang.org/clippy/
- OpenRewrite: https://github.com/openrewrite/rewrite
- ast-grep: https://github.com/ast-grep/ast-grep
- Ripwire analysis reference from the original concept: https://github.com/redhat-et/ripwire

References establish capabilities or comparison targets, not Shear correctness or novelty. `Cargo.lock` records selected versions; source and tests record actual implementation evidence.

## Attribution

Published excerpts of upstream projects retain their licenses and notices. The JavaScript comparison input was pinned to https://github.com/browserify/path-browserify/commit/872fec31a8bac7b9b43be0e54ef3037e0202c5fb, which identifies its Node v8.11.1/Babel origin. Do not claim a Shear license absent an explicit choice.
