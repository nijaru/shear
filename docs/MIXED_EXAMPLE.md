# Checked Python + JavaScript demonstration

Shear implementation `202dc47` processes two real upstream files in **one invocation**. These are two repositories assembled into a demo workspace, not one existing mixed-language application. The individual fixes overlap other linters; the demonstrated distinction from Ruff is the shared Python/JavaScript workflow, not exclusive rules or universal superiority.

## Inputs and result

| Input | Revision | Changes | Unchanged checks before and after |
|---|---|---|---|
| CPython `Lib/argparse.py` | `823f0323ee6ec1402088b73bce1a38473cac36dc` (v3.14.7) | 6 applications: 4 redundant alternatives, 2 conjunctions; 28 added / 35 removed lines | 1,894 unittest tests, no skips |
| browserify/path-browserify `index.js` | `872fec31a8bac7b9b43be0e54ef3037e0202c5fb` (package 1.0.1) | 2 redundant alternatives; 22 added / 25 removed lines | Default runner: 263 TAP assertions, 11 unchanged Windows suite skips; additional parse/format runner: 247 assertions, 4 unchanged Windows suite skips |

Both source hashes changed, all named test outcomes matched (only unittest duration normalized), only the selected tracked files changed, and a shared second `--check` returned zero. Original tracked sources remained unchanged. This is bounded evidence, not a full CPython/platform matrix or general equivalence proof.

The JavaScript source identifies itself as the POSIX portion of Node.js v8.11.1, transpiled with Babel. Do not describe its formatting as independently handwritten. The project exposes `win32: null`; Windows skips are expected, not failures hidden by Shear.

## Reading-path review

Python's useful focus remains `HelpFormatter._format_action_invocation`: remove redundant alternatives while preserving its helper and comments. The full Python result is documented in [SELECTED_EXAMPLE.md](SELECTED_EXAMPLE.md).

JavaScript's `relative()` keeps its path validation, normalization, common-prefix scan, and upward-path construction. Only its final fallback is lifted after a returning condition. `basename()` keeps its extension-matching algorithm; the ordinary no-extension loop is lifted after that branch's direct return. Neither removed block declares lexical bindings or has an implicit statement terminator. No helpers are introduced and no existing helper is edited. Both complete functions and the full file were reviewed, not just the short diff.

Video may focus on representative changed regions, clearly labeled as excerpts. The full patches remain available for review; do not imply the excerpts are the entire functions or all eight changes.

## Reproduce

Requires trusted clean tracked clones at the revisions above, Python 3.14.7, Node (checked with 26.8.1), npm, Git, and a release build. The Node dependency lock was generated for this validation; upstream has no committed lock and disables lock creation through `.npmrc`. `npm ci` uses the retained lock and disables installation scripts. Upstream test programs still execute and are not sandboxed.

```sh
cargo build --release
python3 demo/run_mixed.py \
  --python-repo /tmp/shear-cpython-3.14.7 \
  --javascript-repo /tmp/shear-path-browserify \
  --shear target/release/shear \
  --js-lock demo/evidence/path-browserify/package-lock.json \
  --out /tmp/shear-mixed-reproduction
```

The output directory must be new. The harness resolves macOS path aliases before invoking Shear. It clones fresh copies, verifies import origins, runs real checks, previews and applies the following single command, and checks idempotence/protected files. It never injects a saved patch:

```sh
shear cpython/Lib/argparse.py path-browserify/index.js
```

Actual completed rehearsal: `/private/tmp/shear-mixed-checked`. `commands.json` records command arguments, directories, exits, and elapsed times; separate logs retain every named outcome. Saved patches, licenses, dependency lock, and hash summary are in `demo/evidence/`.

Additional finite API comparison (actual original/candidate modules; values and error names/messages, not stacks):

```sh
node demo/compare_path.cjs \
  /private/tmp/shear-path-browserify/index.js \
  /private/tmp/shear-mixed-checked/path-browserify/index.js
```

Observed: 66,492 matching comparisons, fixed seed 42, 128 path strings plus invalid inputs. This is additional coverage of this result, not proof for arbitrary programs.

## Identities

- Binary SHA-256: `2a2b695077a0dbf6c0d368d161d2f23901de31d378452e8c562ae5de88eca496`.
- Python candidate: `5de7468ba1cdcee02d1d04bf6dbb4a235306d8e910265dd8e2de0c7481f67a3d`.
- JavaScript original: `f6d60f9e7cd83f65f947cda34ef846e82706c044d9a9fa7c5b69718b13b78e28`.
- JavaScript candidate: `248abc366fd1326edf762130c21698c08a9b7f284789e4c394e1fad6dd233712`.
- JavaScript patch: `215f3fa5f82e949ee32b6ecec48c4de5dac5d81a62196c9ca035881fb1e2ff62`.
- Dependency lock: `7b82abf49ac9835cd98f2dfbccda13ad2a98288f05a3a4a381a3dae9fd1b14d8`.

Retain CPython notices and both path-browserify's MIT license and its Node/Joyent source notice. These are checked local edits, not endorsed or submitted upstream contributions.
