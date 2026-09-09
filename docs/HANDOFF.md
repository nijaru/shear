# Shear: structural formatter implementation map

Updated September 9, 2026 after the hackathon. The demo, its evidence, and the event-driven process docs are removed from the working tree; commit `a4a02ad` preserves the finished VHS take, and Git history before it preserves the qualified evidence. This document now owns the development roadmap.

## What exists

A local, deterministic CLI that humans and coding agents run like a formatter. It rewrites mechanically unnecessary control structure toward simpler language-idiomatic forms, then hands presentation to native formatters. No runtime inference, API credentials, helper extraction, generic codemod language, or arbitrary architecture changes.

The `202dc47` binary processed real CPython and path-browserify files with their upstream suites green (recorded in the removed `MIXED_EXAMPLE.md`, retrievable from Git history; its binary hash is pinned there, not here). Later work did not retain that exact binary; the recorded counts do not transfer to newer code. Since then the shared-branch-tail rule and expanded guard/exit analysis landed, with all 64 repository tests green. That suite needs a fresh `target/release/shear` build before use; it is not committed as a binary.

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
- Tests own regression evidence. Passing finite tests is not a proof for arbitrary programs.

Rust is the implementation language; source-language support is independent of it. Tree-sitter supplies syntax, not semantic equivalence. Each rule establishes its own safety conditions per language, optionally using richer language-native tooling later.

## Verified differentiation, September 9

The competitive question is not "are the rules novel" — the first four overlap known autofixers — but "what can Shear do that the current tools cannot, and is that worth a separate step in someone's workflow". Measured against Ruff 0.16.6 and ESLint 10.10.0 on 2026-09-09 (workstation versions; re-verify before relying on any claim):

| Capability | Ruff | ESLint | Shear |
|---|---|---|---|
| Redundant `else` after exit | Yes (`RET505`+, sometimes unsafe) | `return` only (`no-else-return`) | Yes, all exit kinds |
| Nested `if` merge | Yes (`SIM102`, unsafe) | No | Yes |
| Guard-clause conversion | **No fix** | **No fix** | Yes |
| Shared branch tail | **No fix** | **No fix** | Yes |
| Lifted-scope/exit verification | One rule at a time | One rule at a time | Whole file per edit |
| Safe fixes without unsafe gate | Partial | Partial | Yes, by design |
| Comment preservation during lift | Not verified | Not verified | Contract-enforced |

Verified observations behind the table:

- Ruff `--fix --unsafe-fixes` reproduces Shear's redundant-else, nested-merge, and elif-lift output on simple cases; safe-fix mode alone leaves these unchanged, so users wanting these changes must opt into `--unsafe-fixes`.
- Ruff `--select ALL --fix --unsafe-fixes` leaves the `shared-branch-tail` shape (a `locale._localize`-style duplicate continuation) completely untouched.
- Neither Ruff with all rules nor ESLint with `no-else-return`/`no-lonely-if` converts an exiting alternative into a guard (the `case6` shape: non-exiting `if`, `raise`/`throw` in the `else`).
- ESLint `no-else-return` does not fire on `throw` exits at all, and neither tool lifts lexical declarations with the ASI/disposal reasoning Shear performs per rewrite.
- Known Shear weaknesses exposed by the same session: Python guard output emits `not (item.checked)` instead of idiomatic `not item.checked`; the JavaScript backend has no shared-branch-tail rule (both verified live). Un-ruled out: whether some ESLint plugin (e.g. SonarJS) adds guard conversion — unverified, do not claim absence beyond core ESLint.

The recorded `MIXED_EXAMPLE.md` overlap analysis (earlier rules vs. configured Ruff) is retrievable from Git history. Ruff never stood still either: its newer versions keep adding unsafe fixes, so each capability row carries a version footnote and needs periodic re-verification.

## What would make Shear meaningfully better

Two levers exist: rewrites existing tools genuinely lack, and the trusted-agent-formatter experience. Ranked by expected value:

### 1. Own the guard-clause rewrite class

Converting "non-exiting `if` + exiting `else`" into a guard is the highest-value verified gap. Ruff has no such fix; ESLint core has none either. It is also the class that most directly reduces the nesting agents (and humans) leave behind after edits.

- **a. Output quality first:** current output `if not (item.checked):` is visibly worse than hand refactoring. Idiomatic negation (`not item.checked` for simple names/calls, keep parens only where precedence genuinely requires) is table stakes before promoting this rule; without it the rewrite makes code look machine-stamped.
- **b. Verify plugin scope:** check SonarJS and other popular ESLint plugin sets for guard-style fixes before claiming the JavaScript gap is real beyond core ESLint.
- **c. Multi-exit variants:** `else` containing `raise` + more statements after it, guard-conversion for `while`/`for` loops (loop-else and exit conditions), and chained `elif` arms where the last arm exits.

### 2. Broaden shared-tail beyond identical text

`shared-branch-tail` is Shear's most distinctive idea: duplicate continuation after both branches. Ruff/ESLint have nothing like it (verified for the identical-text shape). Broadening it multiplies its hit rate on real code.

- **a. JavaScript port:** the rule exists only in Python. Porting it to the JS backend (which already has the scope machinery for lifted statements) extends the verified two-language pass to all rules.
- **b. Return simplification:** when both branches' tails are `return a, b` vs `return b, a` style variants, conservative unification.
- **c. Semicolon-tail and comment-tail tolerance:** currently skipped conservatively; each tolerated shape needs fixture evidence.

### 2-note. Convergence on combined passes

Shear composes rules to a fixed point, so `redundant-else` → `merge-nested-if` → `guard-clause` chains apply automatically. Ruff's one-shot per-rule fix model cannot express this class without separate runs. Verified as designed behavior; value claim rests on the rules themselves being worth running.

### 3. TypeScript backend

TypeScript is the highest-demand agent language with no Shear support. A TS backend doubles addressable users without requiring new rules (control-flow shapes are the same). The JS backend's scope/ASI/disposal machinery is the template; TS adds type syntax to skip.

### 4. Fresh real-target pass with current code

The old evidence machine (pinned-revision demo scripts, giant workspace copies) is retired. Replace it with the lightweight pattern already used at hackathon scale: fresh clone → `shear` → before/after check runs → inspect diff, without hash-pin scripts. A new pass over CPython/two JS projects with the current binary establishes what the tool does today and feeds honest numbers into any comparison claim.

### 5. The trusted-step experience (if pursuing agent usage)

Agents already run formatters; Shear's premise is being runnable the same way — deterministic, offline, no model, bounded runtime. The verified differentiators there: whole-file reparse after every edit (Ruff's fuzzer-evident architecture applies one rule at a time), comment-multiset preservation enforced at every rewrite, and refusal-to-write on any failed check. These are not roadmap items; they are existing guarantees to keep and state clearly.

### Explicit non-goals

Helper extraction, renaming, arbitrary restructuring, "AI-suggested" rewrites, type checking, universal semantic AST, generic codemod framework, performance work beyond the 2 MiB/1,024-edit bounds, additional languages before current backends justify them, and optimizing any complexity score.

## Milestones

| # | Milestone | Exit criteria |
|---|---|---|
| 1 | Idiomatic guard output | `not (x)` → `not x` where safe; fixtures for precedence cases; README example updated |
| 2 | Verify plugin landscape | SonarJS (and top plugins) checked for guard fixes; docs table updated with findings |
| 3 | JS shared-tail | Ported with Node execution regressions; two-language rule parity |
| 4 | TS backend | Parser wired, rules 1:1, TS corpus smoke pass, native tests green |
| 5 | Real-target pass | Fresh clones, current binary, before/after suites green, diff inspected; findings documented |
| 6 | Release decision | With 1–4 done, revisit license question and distribution with the user |

Milestones 1–2 are small and unlock the highest-value gap; 3 multiplies the distinctive rule; 4 extends reach; 5 replaces retired evidence with current truth; 6 is a user decision, not development.

## Dependency and review posture

Keep `tree-sitter`, grammars, `usage-rs`, `ignore`, `similar`, `tempfile`, `anyhow`. Learn from Ruff's rule/semantic infrastructure externally; embed specific internals only for a concrete benefit. Ruff's Rust interfaces are unstable — a decision to borrow must survive a version bump.

`docs/REVIEW.md` retains the verified safety fixes (declaration-order, local-slot/finalizer-order, form-feed, symlink-path, comment handling) and `docs/RULES.md` the per-rule safety contracts; `docs/BUILD_LOG.md` the build evidence. Both document the engine's history and known limits and stay. Post-hackathon safety work continues the same policy: any new rewrite class ships with the same fixture coverage and real-target pass, and existing guarantees (reparse, convergence, comment multiset, refusal on failed check) are never traded for coverage.

## Source and license

Source is public but has no project license yet. Do not choose a license or call it open source without the user's decision. When a license is chosen, review third-party license implications of the tree-sitter grammars and any borrowed code before publishing releases.
