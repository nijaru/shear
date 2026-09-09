# Correctness and code-quality review

> Provenance note (September 9): the `demo/` paths and removed docs referenced below (`MIXED_EXAMPLE.md`, `SELECTED_EXAMPLE.md`, `demo/run_cpython.py`) were retired after the hackathon; they are retrievable from Git history at commit `a4a02ad`.

September 8, 2026. Reviewed implementation `8d055d3` (engine unchanged since `f573755`). This review supersedes any implication that a passing argparse demonstration establishes general readiness. **No feature freeze is in force. Recording remains paused at the user's request.** No formatter implementation was changed during the initial review. The follow-up fixes are now in `6629139` and `8cf6242`. The original findings below remain as provenance; the final section records their resolution, the additional audit findings, and the new six-application result.

## Two-language follow-up, implementation `202dc47`

JavaScript now has a separately guarded redundant-alternative rule. An independent review found a real `using_declaration` defect: lifting the block changed disposal order from `body,dispose,tail` to `body,tail,dispose`. The parent reproduced it, added recursive `using` exclusion, and added synchronous/`await using` disposal-order and binding-scope regressions. The grammar's declaration kinds were inspected; lexical/class/function/generator/using declarations are excluded, while `var` retains its function/global scope. A second independent review confirmed the fix and found no additional P0–P2 issues in the backend, dispatch, or mixed rehearsal harness.

Current verification: 43 repository tests, formatting, Clippy, and release build pass on macOS; Linux CI run [34284873742](https://github.com/nijaru/shear/actions/runs/34284873742) also passed. The fresh mixed rehearsal at `/private/tmp/shear-mixed-checked` passed 1,894 Python tests with no skips, 263 default JavaScript assertions with 11 unchanged Windows suite skips, and 247 additional assertions with four unchanged Windows suite skips. Only the two selected tracked files changed; original source hashes remained intact; the second shared check returned zero. Full evidence and reading-path assessment: `MIXED_EXAMPLE.md`.

An additional fixed-seed path API comparison matched 66,492 observations across 128 path strings and invalid input cases, comparing actual original/candidate modules (values and error names/messages, not stacks). Reproducer: `demo/compare_path.cjs`; original diagnostic result: `/private/tmp/shear-path-differential.json`. These are finite checks, not exhaustive equivalence or a performance claim. The preservation audit concerns tracked inputs and selected hashes, not every untracked artifact. No final video or playback QA has been completed.

## Findings

### P1 — Guards can produce uncompilable Python

`src/python.rs:181–194`: reordering suites can move a use ahead of its `global` or `nonlocal` declaration. Tree-sitter accepts the result; CPython rejects it. Both the independent reviewer and parent reproduced this.

```python
x = 42
def f(flag):
    if flag:
        global x
        x = 7
    else:
        return x
    return x
print(f(False))
```

Original: exit 0, prints `42`. After Shear: Shear exits 0 and writes the file, but Python exits 1 with `SyntaxError: name 'x' is used prior to global declaration`. The equivalent nested-function `nonlocal x` case fails identically. Shear's subsequent `--check` exits 0 on both invalid results.

Direction: conservatively exclude affected declaration-containing suites from guard reordering, or establish scope-aware declaration-order safety. Add compiler-backed regressions for both declarations. Parse acceptance and idempotence are insufficient acceptance checks. This defect is absent from the checked argparse patch, but blocks a general safety claim.

Parent reproduction files/results: `/tmp/shear-review-bugs/`.

### P2 — Explicit paths traverse symlinked ancestors

`src/main.rs:43–56`: only the final component is inspected for a symlink, then `canonicalize` resolves the entire path. Given `linked -> outside`, invoking `shear linked/target.py` modifies `outside/target.py`, without any race. This contradicts `HANDOFF.md`'s blanket “never follow symlinks” guarantee, although directory traversal itself skips discovered links.

Direction: define and enforce the exact path boundary, including ancestor behavior, with tests. Do not blindly reject every absolute ancestor without considering macOS `/tmp -> /private/tmp`; narrow the guarantee only if that is the intended contract. The current trust-local-repositories restriction reduces exposure but does not make the blanket claim true.

### P2 — The full demo output is not consistently more readable

`src/python.rs:231–236`, visible in `demo/evidence/cpython/argparse.patch` at review commit `08d01da`: nested merges accumulate parentheses and place the entire conjunction on one physical line. Four changed condition headers reach **114, 126, 107, and 130 characters**, in `_add_action`, `_parse_known_args2`, `_parse_optional`, and `_check_value` respectively. The three-level defaults check becomes a double-parenthesized horizontal condition. This reduces indentation, not the underlying number of decisions, and can make scanning harder.

Direction: evaluate conservative local multiline presentation or applicability that avoids making the reading path worse. Do not solve this by formatting unrelated upstream source. An external Ruff 0.16.6 experiment made the conditions readable and Shear remained idempotent afterward, but formatting the original whole file alone changed **627 added / 554 removed lines**. That is not an acceptable hidden addition to the focused demo patch. No formatted replacement has been selected or represented as the checked demo.

## Is there meaningful improvement?

**Yes, locally; not enough to endorse the entire current output without reservations.**

- `_format_action_invocation`: removing two redundant alternatives makes positional, zero-argument optional, and value-taking optional paths easier to follow. The existing `color_option_strings` helper remains in the same function, not hidden in a new abstraction. This is the strongest example.
- `_metavar_formatter` and the `continue` path in argument consumption: small, straightforward removal of redundant wrapping.
- `consume_optional`: a rejection guard lifts the normal option-decoding path; a useful reduction in indentation, though `not (x in mapping)` is less idiomatic than `x not in mapping`.
- `parse_args` and `parse_intermixed_args`: modest stylistic changes, not substantial algorithmic simplification. Do not count these as major quality gains merely because a rule fired.
- Nested-condition merging: compact cases can improve scanning; the long cases above trade vertical nesting for horizontal complexity. Thirteen rule applications are not thirteen independently established quality improvements.

The implementation remains small with clear CLI/engine/rule ownership. Its core weakness exposed here is incomplete semantic applicability, not a need for a generic framework. Broader tests help discover defects but do not replace language-specific reasoning.

## Checks actually run

| Check | Observed result | What it establishes |
|---|---|---|
| `cargo test` | 19 passed (14 rewrite, 5 CLI) | Existing regression suite still passes, despite the newly reproduced bug. |
| `cargo clippy --all-targets -- -D warnings`; `cargo fmt --check`; `cargo build --release` | Passed | Build/static/style checks, not rewrite equivalence. |
| Fresh `demo/run_cpython.py` execution | 1,894 upstream argparse tests before and after, no skips; identical named outcomes except duration; idempotent; target-only modification; original preserved | Actual planned command workflow works on the pinned input. Final on-screen agent interaction is not rehearsed. |
| Differential generated programs | 240 programs changed; 23,040 before/after scenario comparisons matched; all outputs executed and were idempotent | Effectful `__bool__`, comparison objects, Boolean combinations, conditional expressions, short-circuit order, raised truth tests, all four exits, enclosing `finally`, and loop `else` over finite inputs. Does not cover declaration ordering. |
| CPython non-test library corpus | 721 UTF-8 Python files processed: 220 changed, 501 unchanged; zero rejections/timeouts; every result compiled and was idempotent | Broad syntax/compile smoke evidence, not behavior tests for all 721 files. Originals untouched. |
| Three upstream modules using the rewritten corpus first on `PYTHONPATH` | 2,328 tests passed, same ten skips as original baseline | Additional integration coverage for argparse/configparser/urlparse with rewritten dependencies; not all CPython tests. |
| `global` and `nonlocal` counterexamples | Both originals execute; both rewritten outputs fail compilation; Shear accepts both | Confirmed correctness defect. |
| Ancestor-symlink counterexample | Outside target modified; Shear exit 0 | Confirmed mismatch with the documented boundary. |
| External Ruff formatting of copied before/after source | Formatted candidate remains unchanged under Shear; unrelated formatting churn measured separately | Limited formatter composition evidence, not a selected new patch. |

Fresh demo command:

```sh
python3 demo/run_cpython.py \
  --repo /tmp/shear-cpython-3.14.7 \
  --shear target/release/shear \
  --out /tmp/shear-cpython-review-run
```

Its candidate remains SHA-256 `cacfb95dd8d27c3d6ef50565a2c9bbaa8ba03428996b90be76de1594e237e7be`; release binary remains `f6c8adf7f94626733ce5fc65d2915e7897d1708db0dc8f6af74bf77b894d9e66`. Detailed identity is in `SELECTED_EXAMPLE.md`.

Additional local evidence: `/tmp/shear-review-differential.py`, `/tmp/shear-review-differential/summary.json`, `/tmp/shear-review-corpus.py`, `/tmp/shear-review-corpus/summary.json`, `/tmp/shear-review-corpus-tests.log`, `/tmp/shear-review-format/`. The corpus test command was:

```sh
PYTHONPATH=/tmp/shear-review-corpus:/tmp/shear-cpython-3.14.7/Lib \
  PYTHONDONTWRITEBYTECODE=1 python3 -S -m unittest \
  test.test_argparse test.test_configparser test.test_urlparse
```

An independent fresh-context reviewer inspected semantics and file writes; the parent separately assessed the full actual patch, ran broader checks, and reproduced the substantive reviewer findings. This is not an external maintainer review.

## Follow-up fixes and deeper audit — completed at `8cf6242`

The declaration and symlink failures were first captured by failing repository regressions, then fixed. A second independent adversarial audit found two additional concrete failures, also reproduced as failing tests before correction:

- A literal form feed within suite indentation could compile before rewriting but raise `IndentationError` afterward. All form-feed-containing regions are now skipped. The observed indentation semantics are CPython-specific.
- Swapping suites changed CPython local-variable ordering and observable finalizer order, even with no `global`/`nonlocal`. A branch reading a subsequently bound local also reproduces the issue; assignment-only screening is insufficient. Guards now require at least one identifier-free suite, preserving name-encounter order rather than dismissing the resulting effects as unsupported. Declaration-containing regions are separately skipped, conservatively including nested scopes.

The path policy now rejects explicit symlink components before canonicalization, including `link/..`. Absolute macOS `/tmp` aliases are rejected by direct CLI use; the owned-checkout scripts explicitly resolve their paths. No concurrent-mutation transaction is claimed.

Merged headers use precedence-aware grouping and an 88-byte applicability budget, avoiding stacked wrappers and new overlong conditions. Removing an else no longer duplicates its surrounding blank padding; trimming stays within the moved suite. The new argparse patch was read in full: it has **six applications, 28 added/35 removed lines**, retaining useful help-path flattening and short merges while omitting the questionable changes. Current patch and hashes are in `SELECTED_EXAMPLE.md`.

### Final verification

- **30 repository tests pass:** 21 rewrite tests, seven CLI tests, two file-write unit tests. New coverage includes all reproduced bugs, precedence, long-header rejection, blank padding, async context exits, synchronous/asynchronous generator cleanup and injected exceptions, stale source, a replaced symlink target, and rewrite-budget failure without any batch writes. The actual 1,025-opportunity limit test takes roughly 20–30 seconds in debug builds; it is not a throughput benchmark or a fast-path claim.
- `cargo fmt --check`, Clippy with all targets/features and warnings denied, release build, and Python harness Ruff checks pass.
- Actual demo rerun at `/tmp/shear-cpython-final-audit`: **1,894 upstream tests before/after, no skips**, identical named outcomes, current candidate hash verified, target-only change, original preserved, unchanged second run.
- Final binary processed **721 non-test CPython modules: 210 changed, 511 unchanged**. All compiled and were idempotent. Recursively compared compiled code-object local/cell/free-variable names and their order: no differences. This is compiler metadata verification, not execution coverage of all modules.
- **240 generated programs, 23,040 finite before/after scenario comparisons** matched; 227 programs changed and 13 were conservatively skipped. All were idempotent. Sources/output: `/tmp/shear-final-differential.py` and `/tmp/shear-final-differential/`.
- The three upstream module suites with the revised corpus first on `PYTHONPATH` passed **2,328 tests with the same ten skips**. Corpus harness and evidence: `/tmp/shear-final-corpus.py`, `/tmp/shear-final-corpus/summary.json`, `/tmp/shear-final-corpus-tests.log`.
- The existing installed-stdlib observation harness passed against the final binary at `/tmp/shear-final-smoke`.
- A fresh integration reviewer found no additional evidence-backed P0–P2 issue in the fixes. Its 2,700 Python comparisons modeled operand rendering rather than executing a new Shear build; actual binary, compiler corpus, and end-to-end tests above were parent-run. The reviewer checked the guard gate, precedence, local padding, and path checks.

The additional audit's original reproducers remain in `/tmp/shear-audit2-evidence/results.json`; permanent regression coverage is in the repository tests. Final binary SHA-256: `bb96816337dc41bc201e74f38e1e9794dbc1fc8a6d3f9c03808ff654da766180`.

### Residual limits

This resolves the confirmed findings, not every possible Python semantic issue. Guard applicability is intentionally narrower without fuller scope analysis; header length is a conservative proxy, not a universal readability measure. Parsing does not replace the language compiler. Convergence has a pass bound, not a wall-clock guarantee. Concurrent filesystem mutation, crash/power-loss durability, exhaustive equivalence, additional source languages, and the full CPython/platform matrix remain unverified. The actual on-screen Astra interaction, final recording, playback QA, and submission are unfinished. Recording stays paused; feature work is not frozen.
