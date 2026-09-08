# Shear: structural formatter implementation map

Updated September 8, 2026 after explicit user correction. This replaces the mistaken model-proposal product described in the earlier revision. Original concept supplied in `structural-formatter-handoff.md`: **the transformation engine is the product**.

## End state

A local, deterministic CLI that humans and coding agents run like a formatter. It rewrites mechanically unnecessary structure toward simpler language-idiomatic forms, then hands presentation to native formatters. No runtime inference, API credentials, helper extraction, generic codemod language, or arbitrary architecture changes.

Rust is the implementation language. Tree-sitter supplies cross-language syntax trees and byte ranges. Source-language support is independent of the host language. Scope/effects/types are not provided by tree-sitter: each rule must establish its own safety conditions, optionally using language-native semantic tooling later.

## Ownership and architecture

```text
CLI paths / modes / ignores
  -> source-language dispatch
  -> tree-sitter parse (reject malformed input)
  -> language-specific rule applicability + byte-range edit
  -> reparse and repeat to fixed point
  -> check / diff / stale-checked file write
  -> user's normal formatter, compiler, linter, tests
```

- CLI owns file discovery, output, exit status, and writes.
- Engine owns parsing, edit validity, deterministic ordering, and bounded convergence.
- Language rule owns semantics, comment handling, and canonical direction.
- Rule report records concrete transformations, not a universal quality score.
- Tests own regression evidence. Passing finite tests is not a proof for arbitrary programs.

Keep one small Rust package. Share edit/report mechanics, not a speculative universal AST. Learn the backend boundary by implementing a second language.

## Milestones and acceptance criteria

Current checks and known limitations are recorded in `BUILD_LOG.md`; exact rule applicability is in `RULES.md`. Initial Ruff comparison confirms the first two rules overlap existing autofixes.

### 1. Formatter foundation — implemented prototype

Rust, tree-sitter, usage-rs CLI; paths/directories, default write, `--check`, `--diff`, `--explain`. Ignore generated/irrelevant paths where supported and never follow symlinks. Parse before and after changes. Non-mutating modes never write. Idempotence and bounded passes prevent oscillation. Writes compare current source with the read snapshot and preserve permissions; simultaneous writers remain unsupported.

### 2. First transformations — implemented and fixture-tested

Start with Python for the initial engine exercise: it has no branch-local variable scope, supports small readable control-flow examples, and can be behavior-tested with the local interpreter. This is an implementation decision, not a showcase selection or a permanent language priority.

Initial rules:

1. Remove `else` after a directly terminating branch.
2. Merge nested `if` statements without alternatives using short-circuit `and`.

Use conservative syntax subsets. Skip comments in edited regions, multiline string literals, tabs, one-line suites, and named-expression conditions until dedicated handling is implemented. Preserve expression evaluation and control destinations. Do not equate arbitrary truthy values with Boolean values when simplifying returns.

Each rule needs positive/negative, nesting, comment, malformed-source, idempotence, and differential fixtures. Record limitations honestly; these rules overlap existing lint fixes and do not establish novelty.

### 3. Second backend and useful gap — next

Compare candidate transformations against existing native fixes (Ruff/Clippy/ESLint/Go modernize as appropriate). Select a second language and rules based on useful real-source opportunities and semantic tractability, not implementation language. Add that language's grammar and tests. No language is supported merely because its parser loads.

Broader candidates: guard normalization, equivalent-branch consolidation, redundant Boolean/control-flow paths. Scope changes, overloaded operators, destructors, labels, coercions, and evaluation order require language-specific reasoning. Unsafe or judgment-heavy cases remain unchanged.

### 4. Real-project validation

The user's separate discovery session owns target research. Supply it the corrected deterministic/cross-language contract. Use exact source snapshots and unchanged project checks. Keep one file's substantive, readable change for the demonstration, with full source context and attribution. See `EXAMPLE_SELECTION.md`.

### 5. Agent workflow and video

Demonstrate Astra editing code or invoking Shear, Shear's deterministic simplification, named passing checks, and an unchanged second run. No hidden patch injection or runtime model request. Use CLI capture/edit tools and actual playback QA. See `DEMO_VIDEO.md` and `VIDEO_PROMPT.md`.

## Time and scope

At reset: 12:14 PM Pacific. Target a checked formatter slice before expanding languages. Feature freeze 3:45 PM; recording complete 4:30 PM; submission preparation by 5:00 PM; deadline 5:30 PM. Preserve failed attempts and provenance. Public repo/video publication still requires applicable authorization.

## Superseded implementation

The initial uncommitted Go model-proposal prototype was moved outside the repository to `/tmp/shear-superseded-go-20260908`. It is not the product and must not be presented as completed Shear functionality. Earlier planning and this correction remain distinguishable in Git history.
