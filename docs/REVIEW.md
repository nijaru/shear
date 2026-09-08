# Correctness and code-quality review

September 8, 2026. Reviewed implementation `8d055d3` (engine unchanged since `f573755`). This review supersedes any implication that a passing argparse demonstration establishes general readiness. **No feature freeze is in force. Recording remains paused at the user's request.** No formatter implementation was changed during the initial review. Follow-up implementation now skips declaration-containing guard regions and rejects explicit symlinked path components. Both findings first failed new regressions, then passed after the fixes; the original findings below remain as provenance. The deeper audit additionally reproduced form-feed indentation corruption and CPython local-slot/finalizer-order changes. Form-feed regions are now skipped, and guards cannot move names across names; new compiler/behavior regressions failed before these fixes and pass afterward. This deliberately reduces guard applicability rather than redefining observable effects out of the contract. All 23 repository tests and the fresh upstream argparse rehearsal pass. Output-quality work continues.

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

`src/python.rs:231–236`, visible in `demo/evidence/cpython/argparse.patch`: nested merges accumulate parentheses and place the entire conjunction on one physical line. Four changed condition headers reach **114, 126, 107, and 130 characters**, in `_add_action`, `_parse_known_args2`, `_parse_optional`, and `_check_value` respectively. The three-level defaults check becomes a double-parenthesized horizontal condition. This reduces indentation, not the underlying number of decisions, and can make scanning harder.

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

## Remaining work

Correct the declaration-order defect and protect it with regression tests; resolve the path-policy mismatch; improve and re-review mixed-quality generated output; then rerun the actual demonstration against the new binary and retain new hashes. Feature work remains open to improvements justified by this evidence. No final recording, playback QA, or submission is complete. Concurrent writers, exhaustive semantic equivalence, additional source-language support, and the full CPython/platform test matrix remain outside this review's verified result.
