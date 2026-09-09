# Architecture

Shear is a local, deterministic CLI that humans and coding agents run like a formatter. It rewrites mechanically unnecessary control structure toward simpler language-idiomatic forms, then hands presentation to native formatters. There is no runtime inference, API credentials, helper extraction, generic codemod language, or arbitrary architecture change.

Rust is the implementation language; source-language support is independent of it. Tree-sitter supplies syntax, not semantic equivalence. Each rule establishes its own safety conditions per language, optionally using richer language-native tooling later.

## Pipeline

```text
CLI paths / modes / ignores
  -> source-language dispatch
  -> tree-sitter parse (reject malformed input)
  -> language-specific rule applicability + byte-range edit
  -> reparse and repeat to fixed point
  -> check / diff / stale-checked file write
  -> user's normal formatter, compiler, linter, tests
```

## Ownership

- **CLI** (`src/main.rs`) owns file discovery, output, exit status, and writes.
- **Engine** (`src/lib.rs`) owns parsing, edit validity, deterministic ordering, bounded convergence, and the comment-multiset check.
- **Language rule** (`src/python.rs`, `src/javascript.rs`) owns semantics, comment handling, and canonical direction.
- **Tests** (`tests/`) own regression evidence. Passing finite tests is not a proof for arbitrary programs.

## Engine guarantees

- One deterministic preorder-selected edit is applied per pass, followed by a full reparse.
- Every current rule removes a statement or alternative, giving a decreasing structural measure.
- After 1,024 edits the engine fails rather than writing a partial result. Files over 2 MiB are refused.
- Every reparse compares the multiset of exact comment texts with the original. Dropping, duplicating, or changing a comment is an error with no file write.
- Writes are stale-checked, preserve permissions, and use a temporary-file replacement. This is not a multi-file transaction, and concurrent writers are unsupported.
- Two runs must produce identical source; tests enforce idempotence.
- A malformed or oversized input prevents any write in the batch.

The implementation does not provide type analysis, complexity scoring, diagnostic-only smells, automatic native formatting, Git-changed selection, or languages beyond Python and JavaScript. Add those only with a concrete purpose and corresponding checks.

## Dependencies

Keep `tree-sitter`, the Python and JavaScript grammars, `usage-rs`, `ignore`, `similar`, `tempfile`, and `anyhow`. `Cargo.lock` records exact resolved versions; no runtime model SDK or network service is required. Dependency changes are evidence-driven and managed with Cargo. See [DEPENDENCIES.md](DEPENDENCIES.md).

## Rule contracts

Per-rule applicability, actions, reasoning, and checks live in [RULES.md](RULES.md). A rewrite claim is demonstrated only when a real file was processed and the surrounding project's checks stayed green; generated fixtures are verifier evidence, not real-project demonstrations.
