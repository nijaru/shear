# Build evidence — September 8, 2026

This records actual local results, separately from the roadmap. Times are Pacific. No real-project showcase or final video has qualified.

## Product correction, 12:14–12:29

The user confirmed deterministic formatter behavior, Rust + tree-sitter, cross-language scope, and no runtime Astra integration. The uncommitted Go proposal prototype was set aside. `AGENTS.md` and every existing planning/video document were reconciled with that contract. Independent Go target research arrived in commit `36436ef`; its observations were preserved and its obsolete model-trial recommendation marked historical.

Implementation now contains:

- Rust package with tree-sitter Python backend and usage-rs CLI.
- Default write, check, diff, and explain; ignore-aware directory selection.
- Redundant-else removal and nested-if merging with conservative applicability.
- Surgical edits and reparsing to a fixed point.
- Stale-byte checks, temporary-file replacement, and permission preservation.
- Thirteen integration/regression tests, including Python behavioral comparisons.

## Toolchain and checks

Observed Rust 1.98.1, Cargo 1.98.1, macOS arm64. Selected dependency versions are locked in `Cargo.lock`: tree-sitter 0.27.0, Python grammar 0.25.0, usage-rs 6.8.0, plus filesystem/diff/error utilities.

| Actual command | Result |
|---|---|
| `cargo test` | Pass: 5 CLI tests and 8 rewrite tests; Python subprocess comparisons included |
| `cargo fmt --check` | Pass after formatting |
| `cargo clippy --all-targets --all-features -- -D warnings` | Pass after collapsing a nested Rust conditional |
| `cargo build --release` | Pass |
| `cargo machete` | No unused dependencies |
| `git diff --check` | Pass |
| `cargo run -- --help` | Formatter-shaped help rendered correctly |

Earlier failures: the first Rust compile passed a `usize` child index to tree-sitter's `u32` API; corrected with checked conversion. Clippy then flagged a collapsible conditional in the implementation; corrected using a let-chain. Before the product reset, synthetic Go tests encountered global commit signing and macOS `/var` symlink canonicalization; those belonged to the discarded prototype, not current functionality.

No tests are represented as exhaustive semantic proof. Symlink and permission behavior was exercised on macOS, not Windows. The CLI does not run native checks itself. Writes are not a multi-file transaction and simultaneous writers are unsupported.

## Synthetic comparison with Ruff

Input: tracked `examples/control_flow.py`, explicitly synthetic. Local artifacts: `/tmp/shear-rule-comparison.KroGzH`.

Executed:

```sh
shear --diff --explain shear.py
shear shear.py
python3 -I original.py > before.txt
python3 -I shear.py > after.txt
diff -u before.txt after.txt
ruff format --isolated shear.py
shear --check shear.py
ruff check --isolated --select SIM102,RET505 --fix ruff.py
ruff check --isolated --select SIM102,RET505 --fix --unsafe-fixes ruff.py
python3 -I ruff.py > ruff.txt
diff -u before.txt ruff.txt
shear --check ruff.py
```

Here `shear` was `target/debug/shear`; inputs were disposable copies, not the tracked example. Shear applied two rules, preserved all four observed outputs, and made no second-run changes. Ruff format left Shear's result unchanged. Ruff 0.16.6's ordinary fix mode removed the else but exited 1 with SIM102 remaining; adding unsafe fixes applied SIM102 and exited 0. Its result had matching output and Shear found nothing more to simplify.

**Conclusion: these initial rules overlap existing fixes.** They validate an implementation path, not a novel capability. Ruff's current [SIM102 documentation](https://docs.astral.sh/ruff/rules/collapsible-if/) describes safe fixes in preview and unsafe classification outside preview; do not interpret the flag difference as demonstrated Shear superiority.

## Media smoke test

VHS and ttyd were not installed. FFmpeg, ffprobe, and macOS screencapture were available. `screencapture -h` returned usage with an illegal-option message; the displayed usage established the video flags.

Executed:

```sh
/usr/sbin/screencapture -x -v -V 10 demo/smoke/capture.mov
ffmpeg -v error -n -i demo/smoke/capture.mov -an \
  -vf 'scale=1920:-2' -c:v libx264 -crf 18 -pix_fmt yuv420p \
  -movflags +faststart demo/smoke/capture.mp4
ffprobe -v error -show_entries \
  format=duration,size:stream=codec_name,width,height,pix_fmt \
  -of json demo/smoke/capture.mp4
ffmpeg -v error -i demo/smoke/capture.mp4 -f null -
```

MOV: H.264, 3456×2234, 9.993333 seconds. MP4: H.264/yuv420p, 1920×1242, 10.000000 seconds, 726052 bytes; full decode succeeded. No audio was captured. These are local private desktop smoke artifacts, not demo footage. Full-size playback, framing, and readability were **not reviewed**. Repeatable commands are in `demo/smoke.sh`; raw/output directories are ignored.

## Coherent-pass iteration, 12:50 PM onward

The user clarified that useful structural cleanup per agent invocation is the objective, not novelty of individual rules. Updated the roadmap and dependency strategy accordingly. Ruff remains an external reference and comparison tool; no new dependencies were added and no Ruff source was copied. Shear remains public with no project license chosen.

Added `guard-clause`: a terminal alternative becomes an early exit, allowing the normal path to be lifted and further simplified by the existing rules. Negation uses `not (condition)` rather than inverting comparisons, preserving NaN and rich-comparison behavior.

Actual development checks:

- `cargo test --test rewrites guard`: two expected failures before implementation (guard absent; only nested-if rule ran).
- `cargo test`: all 16 tests pass afterward (5 CLI, 11 rewrite).
- `cargo clippy --all-targets --all-features -- -D warnings`: pass.
- Differential Python fixtures preserve effectful truth tests, raised exceptions, NaN classification, `finally` effects, returns, break, and continue. Composition and unchanged second runs are checked.

## Suite comments and real-source smoke, 12:56–1:05 PM

Removed the blanket comment exclusion in favor of suite-aware source slices. Leading comments are children of tree-sitter headers rather than block nodes; a local syntax-tree probe established that ownership before implementation. Ordinary suite comments now move with their statements. Tooling directives, modified-header comments, and ambiguous under-indented/inter-suite comments remain conservative skips. The engine enforces exact comment-text multiplicity across every reparse.

Three new comment fixtures first failed as expected because no rewrite was offered. After implementation, all **19 tests** pass (5 CLI, 14 rewrite), including exact comment placement and idempotence. `cargo fmt --check`, Clippy with warnings denied, and release build pass. The temporary syntax-tree probe was removed.

Added `scripts/stdlib_smoke.py`, a no-dependency Python 3.11+ harness. It copies modules from the active interpreter to a new output directory, establishes original observations, runs Shear, checks compilation and matching observations, checks an unchanged second run, and rechecks original bytes. Child commands have 30-second deadlines. This is trusted local stdlib execution, not a sandbox.

Actual final command:

```sh
python3 scripts/stdlib_smoke.py --shear target/release/shear \
  --out /tmp/shear-stdlib-smoke-20260908-final
```

Interpreter: `3.14.7 (main, Aug 7 2026, 02:15:30) [Clang 22.1.3]`. Binary SHA-256: `f6c8adf7f94626733ce5fc65d2915e7897d1708db0dc8f6af74bf77b894d9e66`.

| Installed input | Original SHA-256 | Rules applied | Result |
|---|---|---:|---|
| `configparser.py` | `de8ddb6cdaa3bb51885ff47a768a81cb32bd279dd5a5eaf5388cde48b7f264a9` | 6 | Compile, observation comparison, idempotence, original preservation passed |
| `argparse.py` | `b9f0fa53be3d7c9c10a71a8eecc538a27c32fbe7abe337818d0d36e138bfc5c5` | 13 | Same checks passed |
| `urllib/parse.py` | `484b633b81d024da52649c7ff775c81b338de9f60585c94e5adfe335a27d9509` | 3 | Same checks passed |

Artifacts include source copies, actual patches, rule explanations, before/after JSON observations, candidate hashes, and elapsed times. The observations exercise configuration interpolation/fallback/errors, argument conversion/choices/help, and URL port errors/defragmentation. They are not held-out or exhaustive tests. The installed interpreter does not include `test/test_pathlib`; **no full CPython test suite was run**, no upstream Git revision is asserted, and no source license/publication qualification was performed. These are validation inputs, not selected showcase candidates. A read-only pathlib preview also succeeded with two rewrites.

The harness initially triggered Ruff's broad-exception, loop-binding, and explicit-subprocess-check diagnostics. Loop arguments are now bound/passed directly, `check=False` is explicit because exit status is inspected, and the exception observer has a narrow explained suppression because exception type/message is deliberately compared. `ruff check --isolated scripts/stdlib_smoke.py` and `ruff format --isolated --check scripts/stdlib_smoke.py` pass.

## Next work

1. Validate the combined control-flow pass on a useful real-code example; existing autofixer overlap is acceptable and should be documented.
2. Improve conservative applicability where real code exposes friction, preserving comment and safety contracts.
3. Requalify incoming targets under the deterministic product contract; existing Go research is not runnable with the Python-only prototype.
4. Add a second backend after the first-language pass provides useful evidence, rather than chasing grammar count.
5. Run unchanged real-project checks and prepare the one-minute video only after useful source evidence exists.
