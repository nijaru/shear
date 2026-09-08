# Current checked example: CPython argparse

Updated after the correctness and output-quality fixes in Shear `8cf6242` on September 8, 2026. The earlier 13-application result remains in Git history (`8d055d3`) and the initial review; **the current result has six applications**. Recording is paused and no feature freeze is in force.

## Exact identity

- Repository: https://github.com/python/cpython
- Release: `v3.14.7`, commit `823f0323ee6ec1402088b73bce1a38473cac36dc`
- Target: `Lib/argparse.py`
- Original SHA-256: `b9f0fa53be3d7c9c10a71a8eecc538a27c32fbe7abe337818d0d36e138bfc5c5`
- Current candidate SHA-256: `5de7468ba1cdcee02d1d04bf6dbb4a235306d8e910265dd8e2de0c7481f67a3d`
- Unchanged test: `Lib/test/test_argparse.py`, SHA-256 `cb69b51429d3d5d0dda3b5ae04881f3db12eb91a66bcb021a9c07b96c01ac1a0`
- Shear implementation: `8cf6242`; release binary SHA-256 `bb96816337dc41bc201e74f38e1e9794dbc1fc8a6d3f9c03808ff654da766180`
- Runtime: Python 3.14.7 on macOS arm64, Clang 22.1.3 build dated August 7, 2026.
- Full current patch: [`demo/evidence/cpython/argparse.patch`](../demo/evidence/cpython/argparse.patch), SHA-256 `3a9140f1d9fc9fc05fde37a88e61eb8e731b2809d8df1302e5d67e67f7b9282e`.

## Improvement and full reading path

The main screen region remains `HelpFormatter._format_action_invocation`, original lines **582–620**. The positional-argument path returns immediately, yet optional handling is wrapped in an `else`. Within it, the zero-argument optional path returns, but value-taking options sit in another `else`. Removing both alternatives makes these paths less indented without adding abstractions.

Keep the existing `color_option_strings` helper visible. Its body, closure context, and comments are preserved. Callers include help-width calculation around original line 288 and `_format_action` around line 533. Help tests build real parsers and compare exact expected output through `TestHelpFormattingMetaclass`, test lines 4023 onward.

The **whole target file** is processed by the generic CLI. Current changes:

| Path | Change and judgment |
|---|---|
| `_format_action_invocation` | Two redundant alternatives removed; existing helper remains visible. Strongest readability benefit. |
| `_metavar_formatter.format` | Redundant else removed after a returning tuple path. Small straightforward cleanup. |
| Argument-consumption continuation | Redundant else removed after `continue`; control destination unchanged. |
| `_get_actions_usage_parts` | Short group/bracket checks joined without extra parentheses. |
| `_parse_known_args2` | First two default-eligibility checks joined; the third remains nested to avoid an overlong header. |

That is **four redundant-else removals and two nested-if merges**, 28 added/35 removed lines. No guard is applied to this file now: the unsafe name-reordering opportunities are intentionally skipped. The former 107–130-character merged headers are not produced. The extra blank line formerly left by deleting the outer else is also avoided. No native whole-file formatting was applied.

The full current patch was re-read, not only the main screen region. This is a modest, meaningful structural cleanup, not an algorithmic improvement, performance fix, proof of optimality, or maintainer endorsement. Six applications are not a universal quality score. Selection can change if further review identifies a better result.

## Actual current validation

Fresh execution:

```sh
cargo build --release
python3 demo/run_cpython.py \
  --repo /tmp/shear-cpython-3.14.7 \
  --shear target/release/shear \
  --out /tmp/shear-cpython-final-audit
```

The script created a new disposable checkout and ran actual baseline tests, preview, rewrite, candidate tests, outcome comparison, and `--check`. Both upstream argparse runs passed **1,894 tests, no skips**. Verbose test identities/outcomes match apart from measured duration. Import-origin checks select the disposable implementation; test bytes remain unchanged. Git reports only the target changed. Second run is unchanged; the original checkout is untouched.

Raw evidence is under `/tmp/shear-cpython-final-audit/`: `commands.json`, `baseline.stderr`, `candidate.stderr`, `preview.stdout`, `rules.log`, `patch.diff`, `summary.json`, and individual command outputs. The script does not inject a patch, requires Python 3.14.7 and the expected source revision/hash, refuses an existing output directory, and bounds each subprocess by 60 seconds.

This is the complete argparse module suite, **not all of CPython**, a platform matrix, or exhaustive equivalence. Earlier `_testcapi` imports failed on this installed interpreter; the selected module suite does not require it and had no skips. Broader compiler, binding-order, and behavioral checks plus the independent integration review are recorded in `REVIEW.md`.

## Acquire and reproduce

Using a new destination, the tested acquisition command is:

```sh
git clone --depth 1 --branch v3.14.7 --single-branch \
  https://github.com/python/cpython.git /tmp/shear-cpython-3.14.7
```

Then run the script above with a **new** `--out` directory. The script explicitly resolves its owned paths, so macOS `/tmp` aliases work for script arguments. Direct Shear inputs reject symlink components; use `/private/tmp/...` or the physical working directory for manual invocation.

The older `urllib.parse` fallback patch is retained as **historical evidence**, not a freshly requalified final-video alternative. Its original run used three rewrites and 77 upstream tests with five unchanged non-ASCII-bytes subtest skips. Revalidate it against the current binary before selecting it instead.

## Attribution and actor framing

Retain CPython's [LICENSE](../demo/evidence/cpython/LICENSE), copyright, author attribution, and [change notice](../demo/evidence/cpython/NOTICE.md) with published excerpts. This does not assign a license to Shear or imply PSF endorsement.

Astra develops and invokes Shear; the executable makes no model/API request. The actual command workflow has been rehearsed, but the final on-screen Astra interaction and video playback have **not**. Current source support is Python only; Rust is the implementation language, and cross-language rewriting remains a direction. Use `shear --check`, not `shear check`.
