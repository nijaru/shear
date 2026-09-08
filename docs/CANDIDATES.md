# Candidate evidence

**Current qualified pair:** CPython `argparse.py` plus path-browserify `index.js`, checked in one invocation. See `MIXED_EXAMPLE.md` for the current eight-application result, source identities, full reading-path review, and all three unchanged test suites. JavaScript research found useful redundant alternatives in `relative()` and `basename()`; it did not justify forcing conjunction merges.

## Historical CPython discovery

**Historical discovery record.** The original counts below predate the fixes in `8cf6242`; `SELECTED_EXAMPLE.md` is authoritative for the current six-application argparse result. Other candidates must be revalidated before recording.

September 8, 2026. These candidates were evaluated using the implemented Python backend after the user specifically proposed the standard library. They supplement the separate session's historical Rust/Go/TypeScript suggestions in `TARGET_RESEARCH.md`; unsupported languages were not claimed to run.

All three installed files matched CPython **v3.14.7**, commit **`823f0323ee6ec1402088b73bce1a38473cac36dc`**, byte-for-byte. A shallow upstream clone supplied surrounding code and original tests. Source: https://github.com/python/cpython/tree/823f0323ee6ec1402088b73bce1a38473cac36dc . Upstream notices and derivative-change requirements are retained under `demo/evidence/cpython/`.

| File | Concrete opportunity/context | Actual results | Verdict |
|---|---|---|---|
| `Lib/argparse.py` | `_format_action_invocation`, lines 582–620: positional path returns, but optional handling remains inside a redundant else, then another else after a returning zero-argument path. Existing color helper and comments are material. Other hits affect default/optional/error paths. | 13 rule applications, 3 rule types. Exact `test.test_argparse` suite: 1,894 tests, no skips, before and after. Only target source changed; identical test bytes/outcomes; unchanged second run. | **Selected for demo preparation.** Useful visible flattening in mature code, with complete module tests; long combined conditions elsewhere are a recorded presentation trade-off. |
| `Lib/urllib/parse.py` | Port validation around lines 176–185 can lead with rejection; `_DefragResultBase` string/bytes `geturl` methods have redundant alternatives around lines 323–349. No algorithm replacement or port-policy change. | 3 applications, 2 rule types; 4 added/7 removed lines. `test.test_urlparse`: 77 tests, five non-ASCII-bytes subtest skips before and after; no new skips. Only target changed; stable second run. | **Checked fallback.** Compact readable diff but less substantial than argparse. |
| `Lib/configparser.py` | `get`, lines 838–859: missing-section/option fallbacks and interpolation return paths contain redundant alternatives. Additional nested checks around lines 714 and 1254 and `_options` around 1336. | 6 applications on a disposable installed-file copy. Compilation, finite observation comparison, idempotence passed. Original source participated in the combined upstream baseline below; no isolated candidate upstream suite was run. | Validation input, **not independently qualified**. No need to spend more selection time while a stronger example is checked. |

## Baselines and limitations

Initial original-only command:

```sh
PYTHONPATH=/tmp/shear-cpython-3.14.7/Lib \
  python3 -S -m unittest test.test_argparse test.test_configparser test.test_urlparse
```

Exit 0; 2,328 tests in 1.438 seconds, ten skips. This aggregate does not imply candidate coverage for configparser. Subsequent isolated argparse and urlparse baseline/candidate runs used separate disposable worktrees and `-v`, preserving named outcomes and test-file hashes. Exact commands, identities, results, hashes, and reproduction script are in `SELECTED_EXAMPLE.md` and `BUILD_LOG.md`.

The standard library's reputation is not evidence that any change is needed or preferable. Our selected claim is a tested structural alternative with a concrete readability benefit, not a bug/performance fix or maintainer endorsement. Existing autofixers overlap these operations; that is not a disqualifier or a novelty claim. No full CPython/platform matrix was run.
