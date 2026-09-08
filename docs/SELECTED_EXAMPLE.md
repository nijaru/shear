# Selected example: CPython argparse

Selected September 8, 2026 for demo preparation after a real deterministic rewrite and unchanged upstream tests. **Selection remains provisional:** the subsequent [review](REVIEW.md) found mixed readability in the full patch and a separate engine correctness defect. The recorded test results below remain valid, but recording is paused pending improvements and revalidation. This is an agent readability judgment, not a CPython maintainer endorsement or proof of arbitrary equivalence.

## Identity and claim

- Repository: https://github.com/python/cpython
- Release: `v3.14.7`
- Exact commit: `823f0323ee6ec1402088b73bce1a38473cac36dc`
- File: `Lib/argparse.py`
- Original SHA-256: `b9f0fa53be3d7c9c10a71a8eecc538a27c32fbe7abe337818d0d36e138bfc5c5`
- Checked candidate SHA-256: `cacfb95dd8d27c3d6ef50565a2c9bbaa8ba03428996b90be76de1594e237e7be`
- Test source: `Lib/test/test_argparse.py`, SHA-256 `cb69b51429d3d5d0dda3b5ae04881f3db12eb91a66bcb021a9c07b96c01ac1a0` before and after.
- Shear implementation: `f573755`; release binary SHA-256 `f6c8adf7f94626733ce5fc65d2915e7897d1708db0dc8f6af74bf77b894d9e66`.
- Runtime: CPython 3.14.7, macOS arm64, Clang 22.1.3 build dated August 7, 2026.

**Claim:** Shear produces a reviewable, stable structural normalization of mature Python code, with the applicable module's upstream tests passing unchanged. Do not say the standard library was already optimal, that maintainers wrote bad code, or that this improves runtime speed.

## Visible problem and full reading path

Primary screen region: `HelpFormatter._format_action_invocation`, original lines **582–620**. It returns immediately for positional arguments, but the complete optional-argument path remains wrapped in an `else`. A second returning branch adds another unnecessary `else` around value-taking options. Removing these alternatives makes the normal flow less indented without inventing abstractions.

Keep the existing local `color_option_strings` helper visible: it is preserved, not newly extracted. It uses the already-initialized theme and `_is_long_option`; moving it out of the redundant branch does not create a new Python scope. Callers include help-width calculation around original line 288 and `_format_action` around line 533. Help tests build parsers and compare exact expected output (`TestHelpFormattingMetaclass`, test lines 4023 onward).

The **entire file**, not only this method, is rewritten by the generic CLI: 13 applications across all three current rule types (6 nested-if merges, 4 redundant-else removals, 3 guards). The full diff has 52 added and 65 removed lines. Other affected paths include namespace defaults, negative-option handling, option consumption, choice suggestions, and normal/intermixed argument errors. All changed function contexts were inspected, including existing nested helpers; no helper was introduced.

Readability trade-off: some combined conditions are longer and parenthesized. Shear has not run a native formatter over the file, and this selection does not assert every hunk is universally preferable. Show the useful help-formatting flow and make the full patch available; do not hide the other edits. Optional presentation improvements must be rechecked and recorded as a new result rather than silently altering this patch.

## Actual checks

In a clean disposable CPython worktree, before and after the rewrite:

```sh
PYTHONPATH=/tmp/shear-cpython-argparse/Lib \
  python3 -S -m unittest -v test.test_argparse
```

Both runs exited 0: **1,894 tests, no skips**, 1.180 seconds before and 1.186 seconds after on the first check. The complete verbose transcripts match after excluding only unittest's measured-duration summary. Import-origin checks confirmed both the implementation and tests came from the disposable checkout, not the installed stdlib.

`shear --check Lib/argparse.py` exited 0 afterward. Git reported only `Lib/argparse.py` changed. Test bytes stayed identical; the original checkout and installed interpreter source stayed unchanged. No source signatures, dependency files, assertions, or test identities were edited.

This is the complete `test.test_argparse` module, **not all of CPython**, all operating systems, or exhaustive behavioral proof. Initial attempts to import `_testcapi` failed because this interpreter omits that extension; the selected argparse suite does not require it and had no skips. No new characterization test was used to replace missing upstream tests.

## Reproduce and capture

Acquire an untouched input (use a new destination):

```sh
git clone --depth 1 --branch v3.14.7 --single-branch \
  https://github.com/python/cpython.git /tmp/shear-cpython-3.14.7
git -C /tmp/shear-cpython-3.14.7 rev-parse HEAD
cargo build --release
```

With Python 3.14.7, run the checked script using a new output directory:

```sh
python3 demo/run_cpython.py \
  --repo /tmp/shear-cpython-3.14.7 \
  --shear target/release/shear \
  --out /tmp/shear-cpython-demo-run
```

The script was actually rehearsed successfully at `/tmp/shear-cpython-demo-rehearsal`. It creates its own checkout, runs baseline checks, previews and executes Shear, repeats checks, compares outcomes, checks idempotence and protected files, and saves command logs/hashes/diff. Commands are real, bounded by 60-second deadlines; no patch is injected. The source clone must be clean and at the pinned revision. The script rejects the wrong interpreter version or source hash.

Raw first-run evidence: `/tmp/shear-cpython-evidence/argparse-*`. Full checked patch: [`demo/evidence/cpython/argparse.patch`](../demo/evidence/cpython/argparse.patch). Patch SHA-256: `95759be8a132570ce26bb88b4b08000be7de0afc10e8d8cecf3843060fa30f85`.

## Fallback

Same CPython revision, `Lib/urllib/parse.py`: three real rewrites (one guard and two redundant-else removals), four added/seven removed lines. The port-validation property and string/bytes defragmentation returns give a shorter but less substantial diff.

Original SHA-256 `484b633b81d024da52649c7ff775c81b338de9f60585c94e5adfe335a27d9509`; candidate `89a2bdfa3b4ecc14a5a5b447f8e2e25fb9dc326c5f81d18e52c9343a7cd8797a`. `test.test_urlparse` ran 77 tests, with five non-ASCII-bytes subtest skips, before and after; both exit 0. The skip reasons are unchanged. Verbose logs differ in process-specific object addresses as well as elapsed time; do not claim byte-identical transcripts. Test-file SHA-256 stayed `c1b4bf192569399b268d310dd8f4983c5cbce66d182ee2af29fc047b61897730`. Only the target source changed; second run was unchanged.

Fallback patch: [`demo/evidence/cpython/urlparse.patch`](../demo/evidence/cpython/urlparse.patch). Raw logs: `/tmp/shear-cpython-evidence/urlparse-*`.

## Publication and actor framing

Upstream material remains under CPython's applicable licenses. Retain [`LICENSE`](../demo/evidence/cpython/LICENSE), copyright, author attribution, and the [change summary](../demo/evidence/cpython/NOTICE.md) with published excerpts/patches. This does not assign a license to Shear or imply endorsement.

Show Astra initiating the real CLI/check commands where practical. The executable runs offline and currently supports Python only. Say “Rust implementation, Python backend, cross-language direction,” not that Rust/Go/TypeScript rewriting is already supported. `shear --check` is the actual syntax, not `shear check`.
