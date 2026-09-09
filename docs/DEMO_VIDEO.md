# One-minute demo

The current take uses **VHS to type real terminal commands automatically**. It shows CPython `locale.py` after configured Ruff autofixes and original path-browserify `index.js`, processed by one Shear invocation. These are two separate real projects, not one existing mixed-language application.

Run and verification instructions: [TERMINAL_DEMO.md](TERMINAL_DEMO.md). Tape: `demo/terminal.tape`. Local output: `demo/output/shear-terminal.mp4`.

## Story

- Read the complete Python function before cleanup.
- Run `shear --explain cpython/Lib/locale.py path-browserify/index.js`: one shared Python branch tail and two JavaScript redundant alternatives.
- Read the Python result and the first genuine JavaScript diff hunk.
- Run unchanged upstream suites: Python 66 tests/one skip; JavaScript 263 and 247 assertions with 11/4 unchanged Windows-suite skips.
- Run the shared `--check`: no further changes.

Hidden preparation creates fresh copies, verifies input/binary hashes and runs baselines. Hidden final verification compares named outcomes and previously qualified candidate hashes. Reading pauses are scripted; output comes from the actual commands. No custom slide renderer or simulated agent interaction is used.

## Evidence and limits

The Python comparison is documented in [NORMALIZATION_EXAMPLE.md](NORMALIZATION_EXAMPLE.md). JavaScript source identity, complete reading paths and earlier mixed evidence are in [MIXED_EXAMPLE.md](MIXED_EXAMPLE.md). Its argparse counts belong to the older fallback, not the current locale take.

Shear is a deterministic local structural formatter, without a model inside. Overlap with existing autofixers is acceptable; this recording does not establish general superiority, arbitrary equivalence, or measured agent savings. Credit Astra's development role separately rather than fabricating on-screen agent activity.

## Delivery gates

- Confirm the fresh workspace's `verified.txt`, full logs and unchanged prepared-input hashes.
- Inspect all source/test screenshots and the complete exported playback. Verify MP4 streams and duration; keep the visible take under one minute.
- Keep CPython PSF and path-browserify/Node MIT notices with shared source excerpts. No Shear project license has been selected.
- Preserve media locally in ignored `demo/output/`; commit only scripts, docs and small evidence.
- No upload or submission destination is authorized. The September 8 deadline is 5:30 PM Pacific.

The user-selected VHS approach supersedes the unavailable Codex desktop-control plan and the rejected slideshow. Earlier recording tools and the argparse rehearsal remain historical fallback evidence, not the current demo.
