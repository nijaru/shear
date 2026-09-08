# Build evidence — September 8, 2026

This records actual local results, separately from the roadmap. Times are Pacific. CPython argparse is now selected with upstream module tests passing; the final video is not complete. Earlier entries below retain their original evidence limits.

## Two-language implementation and preparation, from 14:39

- User approved an actual Python/JavaScript workflow rather than claiming novel Python rules. Plan commit `ded3760`; implementation `202dc47`; CI `31aea7a`.
- Queried `cargo search tree-sitter-javascript --limit 1`, then used `cargo add tree-sitter-javascript@0.25.0`. Node is 26.8.1. Candidate research identified path-browserify's real `relative()` and `basename()` alternatives; the parent read the complete source and retained both licenses.
- Cloned `https://github.com/browserify/path-browserify.git` at `872fec31a8bac7b9b43be0e54ef3037e0202c5fb`. Initial `npm install --ignore-scripts --no-audit --no-fund` and `npm test` passed 263 assertions. Its omitted parse/format module separately passed 247. Windows skips are 11 and four, respectively. A demonstration-owned lock now pins the existing dev dependency graph for fresh `npm ci`; no upstream manifest was changed.
- Implemented scope/ASI-aware JavaScript redundant-else, shared enum dispatch, mixed preflight tests, `.js/.mjs/.cjs` recognition, `node_modules` discovery exclusion, and relative display paths. No runtime model or JavaScript interpreter was added to Shear.
- Independent review caught a disposal-scope bug involving `using`; parent reproduced the changed event order before fixing it. New sync/async resource regressions pass. Follow-up independent review found no additional P0–P2 issues.
- `cargo test` passed all 43 tests; `cargo fmt --check`, all-target/all-feature Clippy with warnings denied, and release build passed. Linux CI independently passed in about two minutes, asynchronously. CI is not a dependency of the recording workflow.
- `demo/run_mixed.py` completed two fresh rehearsals. Final evidence at `/private/tmp/shear-mixed-checked`: one invocation, eight applications, both complete source hashes verified, unchanged named outcomes in all three suites, target-only tracked changes, original tracked inputs preserved, idempotence. The existing Python patch differs from the new Git `diff HEAD` artifact only in Git's mnemonic source-prefix metadata; candidate bytes are identical.
- Fixed-seed actual path API comparison: 66,492 observations matched across 128 paths plus invalid arguments. This does not establish arbitrary JavaScript equivalence.
- Harness Ruff initially reported dictionary-style/iteration lint findings; corrected, then checks passed. Workflow validated with actionlint v1.7.12 before its successful remote run.
- `--prepare-only` was executed successfully at `/private/tmp/shear-demo-ready`: all baseline suites passed and both tracked input trees/source hashes remain original. No rewrite was applied there; `prepared.json` records the exact binary and inputs.
- The user prefers Codex potentially operating the desktop and explicitly requested preparation first. No desktop takeover or final recording has occurred. The synthetic delivery-policy preparation is retired in favor of the qualified real inputs.

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

## Upstream qualification and rehearsal, 1:09 PM onward

Verified the user's other session had a stale remote view: `git ls-remote origin refs/heads/main` returned `f573755ea9e1cc1e98b8f8009a4afc76f11eb694`, not `36436ef`.

Cloned CPython tag v3.14.7; its dereferenced commit is `823f0323ee6ec1402088b73bce1a38473cac36dc`. The three installed stdlib source hashes matched that revision exactly. The clone reported the expected annotated-tag/detached-HEAD notices. `_testcapi` import failed on the installed interpreter, but this did not prevent the selected module tests from running.

Original combined baseline (`test.test_argparse`, `test.test_configparser`, `test.test_urlparse`): exit 0, 2,328 tests in 1.438 seconds with ten skips. Then created isolated worktrees, editing one source file per worktree:

- **argparse:** baseline 1,894 tests/1.180s; candidate 1,894 tests/1.186s; both exit 0 with no skips. Complete verbose outcomes identical except measured duration. 13 applications across three rule types; 52 lines added/65 removed. Only `Lib/argparse.py` changed; test-file hash unchanged; original checkout clean; second Shear run unchanged.
- **urllib.parse:** baseline 77 tests/0.074s; candidate 77 tests/0.073s; both exit 0 with the same five non-ASCII-bytes subtest skips. A direct full-log comparison also exposed process-specific object addresses, so no byte-identical-log claim is made. Three applications, four lines added/seven removed. Target-only changes, identical test bytes, and idempotence verified.

Read all changed argparse function contexts (610-line full-function diff) and the relevant help-test machinery. Selected the help-formatting path for its concrete removal of redundant nesting, preserving the existing local helper and comments. Some other merged conditions are long; that readability trade-off is disclosed rather than hidden. This is not a claim that stdlib was wrong/slow or that all changes are objectively superior.

Added and successfully ran `demo/run_cpython.py` against a fresh clone at `/tmp/shear-cpython-demo-rehearsal`. The script requires the recorded interpreter/revision/hash, uses real bounded commands, checks import origin, baseline/candidate verbose outcomes, protected files, idempotence and original preservation, and saves raw logs plus summary. No patch is injected. Its rehearsal again passed 1,894 tests in both phases. Ruff lint/format checks pass for the script.

Retained the exact upstream LICENSE, author/change notices, and both patches under `demo/evidence/cpython/`. `SELECTED_EXAMPLE.md` records acquisition, exact commands/hashes, readable region, full diff, test limitations, and fallback. No full CPython suite, platform matrix, upstream acceptance, or final recording is claimed.

## Next work

1. Produce the one-minute recording using the qualified argparse run and an honest Astra-invocation view; inspect full playback and attribution.
2. Keep feature additions bounded. The useful first-language result is now checked; a second backend is optional, not a reason to jeopardize recording time.
3. Preserve current evidence identity; any change to rules or presentation source requires a newly checked run.

## Follow-up review before recording

The user rejected a feature freeze and requested thorough review/testing of actual code quality. Recording remains paused. `REVIEW.md` supersedes the earlier readiness judgment: an independent reviewer found guard declaration-order corruption and an ancestor-symlink policy mismatch; the parent reproduced both. No implementation fix was applied as part of this report-mode review.

Fresh release build, formatting, Clippy and 19 repository tests passed. Reran the actual demo script in `/tmp/shear-cpython-review-run`: 1,894 upstream tests before/after, no skips, identical named outcomes, same candidate hash, idempotence and original preservation. Additional generated testing covered 240 changed programs and 23,040 finite scenario comparisons. A 721-file non-test CPython corpus produced 220 changed/501 unchanged files, all compiling and idempotent; the three upstream module suites against that rewritten corpus passed 2,328 tests with the same ten skips. These passing checks did not catch the confirmed declaration-order counterexamples.

Full-patch review found meaningful help-path flattening but four merged headers of 107–130 characters. A Ruff-formatting experiment preserved Shear idempotence but caused excessive unrelated whole-file churn. No replacement demo patch was selected. Exact findings and evidence paths are in `REVIEW.md`.

Operational diagnostics: `tk create` was rejected (the actual command is `tk add`); the first header-length measurement hit an empty condition list from another same-named method and was rerun filtering empty lists. A no-index diff across CPython paths emitted attribute-macro warnings but returned its diff statistics. No source changes resulted from those diagnostic failures. The next implementation work should address the confirmed bugs and mixed-quality output, not record around them.

## Confirmed fixes and integration audit

Implemented and pushed `6629139` (safety) and `8cf6242` (output quality and failure-path tests). New declaration-order and ancestor-symlink regressions failed against the old implementation before passing with the fixes. A deeper independent audit additionally reproduced form-feed indentation corruption and changed CPython `locals()`/finalizer order; those also failed compiler/behavior tests before correction. Guards now avoid name reordering rather than silently excluding finalizer effects from the contract. Explicit symlink components are rejected before canonicalization, including `link/..`; owned smoke/demo paths are explicitly resolved.

The new output avoids gratuitous grouping, skips merges exceeding an 88-byte header budget, and avoids duplicated blank padding after else removal. Current argparse result is six applications, 28 added/35 removed lines—not thirteen applications. No named guard is forced into the demo for feature coverage. The complete revised patch was inspected and saved with current hashes in `SELECTED_EXAMPLE.md`.

Final checks: 30 repository tests, formatting, all-target/all-feature Clippy, release build, and harness Ruff checks pass. A final compiler-backed regression additionally exercises async context exits and synchronous/asynchronous generator cleanup, including an injected exception and explicit close. The real rewrite-limit failure test takes 20–30 seconds in a debug build; bounded passes are not a wall-clock performance guarantee. Fresh actual demo `/tmp/shear-cpython-final-audit` passed 1,894 tests in each phase without skips; target-only change, preserved test bytes/original, and idempotence verified. Final corpus: 721 modules, 210 changed, all compile/idempotent and compiled local/cell/free-variable order unchanged. Final generated comparisons: 240 programs, 227 changed, 23,040 finite scenarios matching. Three upstream suites against that corpus passed 2,328 tests with the same ten skips. Installed-stdlib smoke passed. A final independent integration reviewer reported no additional evidence-backed P0–P2 finding; its modeled probes are distinguished from parent-run actual-binary checks in `REVIEW.md`.

Recording remains paused. There is no feature freeze. Confirmed findings are fixed, but finite verification, narrower applicability, and unfinished video/actor QA are not presented as universal readiness.
