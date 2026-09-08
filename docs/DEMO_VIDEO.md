# One-minute demo workflow

## Current status and claim

The checked demo now uses **Python and JavaScript in one invocation**: CPython `argparse.py` and path-browserify `index.js`. These are two real projects assembled for the demo, not one existing mixed-language application. See [MIXED_EXAMPLE.md](MIXED_EXAMPLE.md) for complete reading-path review, checks, identities, limitations, and reproduction.

The claim is a consistent deterministic structural-cleanup step for humans and agents. Individual rules overlap Ruff/ESLint; do not claim exclusivity, measured agent savings, or general equivalence. Shear does not call a model internally.

The latest user instruction is to **focus on preparation**, with Codex potentially operating the desktop for the eventual recording. Do not build additional recording infrastructure or wait for CI on the recording path. Coordinate desktop access before recording. Codex CLI 0.153.4 is installed; desktop-control capability has not been established merely by finding the CLI.

## Prepared inputs

`demo/run_mixed.py --prepare-only` runs the same pinned checkout preparation and baseline suites as the full rehearsal, but leaves both files unmodified. It saves command logs and `prepared.json`. The output directory must be new. Use the full command in `MIXED_EXAMPLE.md`, add `--prepare-only`, and choose `/tmp/shear-demo-ready` for the local operator workspace.

Before capture, match the binary and input hashes to the checked evidence. Do not reuse an already-modified workspace and call it the original. A fresh full rehearsal succeeded at `/private/tmp/shear-mixed-checked`:

- Eight applications: six Python, two JavaScript.
- Python: 1,894 tests, no skips, before and after.
- JavaScript: 263 default assertions plus 247 additional parse/format assertions; unchanged 11/4 Windows suite skips.
- Same named outcomes; only selected tracked files changed; original tracked inputs preserved; second check unchanged.

## Story, approximately 58 seconds

| Time | Content |
|---|---|
| 0–6 s | “Two real projects. One structural cleanup step.” Name Python/CPython and JavaScript/path-browserify. |
| 6–15 s | Actual preview, then apply with `shear cpython/Lib/argparse.py path-browserify/index.js`; show both file names and the shared command. |
| 15–32 s | Readable Python before/after: optional-action formatting loses unnecessary alternatives. Clearly label a focused excerpt; existing helper remains unchanged. |
| 32–43 s | Readable JavaScript before/after: `relative()`'s final fallback is no longer nested under `else`. Disclose the second `basename()` cleanup in the full patch. |
| 43–52 s | Separately named project checks for the actual applied candidate. Baselines were completed in preparation; show actual candidate summaries, including the POSIX-only limitation. |
| 52–58 s | Actual shared `--check` returns unchanged. Closing claim: “Deterministic. No model inside Shear.” Credit Astra development separately from the recording operator. |

The source improvement is the centerpiece. Preserve full changed functions/helpers and patches in evidence; do not pretend short excerpts show all eight edits. Reading pauses and cuts between operations are fine; label replay/accelerated waits and make no unmeasured speed claim. Never splice one candidate into another run's checks or fabricate agent activity.

## Operator instructions

See [CODEX_DEMO_RUNBOOK.md](CODEX_DEMO_RUNBOOK.md). A desktop-capable Codex session may operate the clean terminal/editor and existing recorder after access is coordinated. If those tools are unavailable, report that limitation rather than inventing a computer-use capability or installing a new recording stack. Actual CLI execution matters more than simulated typing.

## Capture and delivery gates

- Use an isolated, readable terminal/editor view; hide private tabs, notifications, and unrelated work. Do not bypass OS permissions.
- Existing FFmpeg/ffprobe and macOS screen capture passed a ten-second smoke test. VHS/ttyd are absent; no need to install them.
- Retain raw footage in ignored `demo/raw/`; export to ignored `demo/output/`. Commit only small scripts and sanitized evidence.
- Export a playable 55–59-second MP4. Captions may replace narration. Verify streams/duration and watch the complete export at normal size; metadata alone is not playback QA.
- No video exists yet. A prepared workspace or runbook is not a finished video.
- Keep the 4:30 PM recording / 5:00 PM preparation targets and 5:30 PM Pacific deadline visible. These are scheduling checkpoints, not a feature freeze. External upload requires an authorized destination.
