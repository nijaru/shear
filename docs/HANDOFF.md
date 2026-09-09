# Shear: structural formatter implementation map

Updated September 8, 2026 after explicit user correction. This replaces the mistaken model-proposal product described in the earlier revision. Original concept supplied in `structural-formatter-handoff.md`: **the transformation engine is the product**.

## Active delivery status — September 8, 4:55 PM Pacific

The expanded normalization pass supports Python shared branch tails, exhaustive exits, redundant `elif` flattening and broader conservative guards, plus JavaScript exhaustive exits, nested-condition merging and guards. All 64 repository tests, formatting and Clippy pass locally with Python 3.14.7 and Node v26.8.1. The default login shell selected Python 3.9 in the desktop session and failed the existing `anext` test; selecting `/opt/homebrew/bin` first restored the intended environment.

The current recording centerpiece is [NORMALIZATION_EXAMPLE.md](NORMALIZATION_EXAMPLE.md): a real shared continuation in CPython `locale._localize` after configured Ruff autofixes. The earlier argparse+JavaScript evidence in [MIXED_EXAMPLE.md](MIXED_EXAMPLE.md) remains tied to `202dc47`; do not apply its eight-rewrite count to the expanded binary.

Delivery now uses user-selected VHS automation: fresh Ruff-only locale input alongside original path-browserify JavaScript, typed real terminal commands, upstream checks and an unchanged second pass. See [TERMINAL_DEMO.md](TERMINAL_DEMO.md) and `demo/terminal.tape`. This supersedes the desktop-operator plan and rejected custom slideshow. The event deadline is 5:30 PM Pacific. External upload needs an authorized destination. No feature freeze is in force; prioritize delivery of the checked pass.

## End state

A local, deterministic CLI that humans and coding agents run like a formatter. It rewrites mechanically unnecessary structure toward simpler language-idiomatic forms, then hands presentation to native formatters. No runtime inference, API credentials, helper extraction, generic codemod language, or arbitrary architecture changes.

Rust is the implementation language. Tree-sitter supplies cross-language syntax trees and byte ranges. Source-language support is independent of the host language. Scope/effects/types are not provided by tree-sitter: each rule must establish its own safety conditions, optionally using language-native semantic tooling later.

## Product value and scope

The goal is useful structural cleanup per human/agent invocation, not exclusive ownership of every rewrite. Overlap with Ruff, Clippy, ESLint, or other autofixers is acceptable. Do not gate progress on finding a novel rule or claim that those tools lack semantic analysis.

**Today:** build on the checked Python pass to demonstrate useful Python and JavaScript cleanup in one invocation. Preserve behavior/comments and convergence with language-specific safety, not parser-only support. Keep the shared engine small; add the required grammar through Cargo. Use Ruff/ESLint externally for comparison rather than claim their overlapping fixes are missing.

**Long term:** reduce recurring manual and agent cleanup across languages. Develop shared edit/convergence mechanics and language-specific syntax, binding, termination, effect, and type facts as needed. Tree-sitter is the common syntax entry point, not a restriction against richer backends. Agent time savings and fewer iterations are hypotheses to measure, not current performance claims. Naming, architecture, and intent remain human/agent responsibilities.

Shear's source is public but has no project license yet. Do not choose one or call the project open source without the user's decision. Preserve all applicable dependency and copied-code notices.

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

The follow-up review led to declaration-order, local-slot/finalizer-order, form-feed, and symlink-path fixes, plus more conservative and readable merging. See `REVIEW.md` for verified fixes and residual limits; passing tests still do not establish arbitrary equivalence. VHS recording is authorized, and feature work remains open.

Current checks and known limitations are recorded in `BUILD_LOG.md`; exact rule applicability is in `RULES.md`. Initial Ruff comparison confirms the first two rules overlap existing autofixes.

### 1. Formatter foundation — implemented prototype

Rust, tree-sitter, usage-rs CLI; paths/directories, default write, `--check`, `--diff`, `--explain`. Ignore generated/irrelevant paths where supported and never follow symlinks. Parse before and after changes. Non-mutating modes never write. Idempotence and bounded passes prevent oscillation. Writes compare current source with the read snapshot and preserve permissions; simultaneous writers remain unsupported.

### 2. First transformations — implemented and fixture-tested

Start with Python for the initial engine exercise: it has no branch-local variable scope, supports small readable control-flow examples, and can be behavior-tested with the local interpreter. This is an implementation decision, not a showcase selection or a permanent language priority.

Initial rules:

1. Remove `else` after a directly terminating branch.
2. Merge nested `if` statements without alternatives using short-circuit `and`.
3. Normalize a directly exiting `else` into a guard and lift the normal path.

Use conservative syntax subsets. Preserve ordinary suite comments and enforce exact comment-text preservation after each rewrite; skip directives, ambiguous comment placement, multiline string literals, tabs, one-line suites, and named-expression conditions. Preserve expression evaluation and control destinations. Do not equate arbitrary truthy values with Boolean values when simplifying returns.

Each rule needs positive/negative, nesting, comment, malformed-source, idempotence, and differential fixtures. Record limitations honestly; these rules overlap existing lint fixes and do not establish novelty.

### 3. Coherent control-flow normalization — active

Compose redundant-else removal, nested-condition flattening, and guard normalization. Add redundant-path simplification only with explicit safety conditions. Prioritize useful combined output, preservation, and stable formatter behavior over isolated rule novelty. Use existing tool implementations and tests as references; record attribution for any copied/adapted material.

Keep `tree-sitter`, language grammars, `usage-rs`, `ignore`, `similar`, `tempfile`, and `anyhow`. Do not add a generic rewrite framework. Ruff internals are published but have unstable Rust interfaces; consider a specific crate only when its Python semantic capabilities materially reduce complexity. See `DEPENDENCIES.md`.

The implemented JavaScript backend follows the useful Python pass; see its distinct contract in `RULES.md`. Scope changes, overloaded operators, destructors, labels, coercions, and evaluation order require language-specific reasoning. No language is supported merely because its parser loads.

### 4. Real-project validation

The user's separate session supplied provisional targets. The user then authorized evaluating Python's standard library: CPython v3.14.7 `Lib/argparse.py` currently has six real rewrites and 1,894 unchanged upstream tests with no skips. `Lib/urllib/parse.py` remains historical fallback evidence requiring revalidation before use. See `SELECTED_EXAMPLE.md` and `CANDIDATES.md` for exact identities, source review, limitations, and reproduction. Do not describe mature stdlib code as already optimal or claim a performance fix. Keep one file's readable change central, with full source context and attribution.

### 5. Agent workflow and video

Demonstrate the actual two-language Shear invocation, useful changes, named passing checks, and an unchanged second run. Codex may operate the desktop if that capability is available; credit Astra development separately and never fabricate agent activity. No hidden patch injection or runtime model request. Use CLI capture/edit tools and actual playback QA. See `DEMO_VIDEO.md` and `VIDEO_PROMPT.md`.

## Time and scope

No feature freeze is in force, per the user's correction. Thorough review and testing precede recording; improvements remain in scope. Earlier recording/submission targets were 4:30 PM/5:00 PM, with a 5:30 PM deadline; reassess scheduling against actual quality rather than locking features. Recording is now authorized through VHS. Preserve failed attempts and provenance. Public repo/video publication still requires applicable authorization.

## Superseded implementation

The initial uncommitted Go model-proposal prototype was moved outside the repository to `/tmp/shear-superseded-go-20260908`. It is not the product and must not be presented as completed Shear functionality. Earlier planning and this correction remain distinguishable in Git history.
