# One-minute demo workflow

## Story

Astra and other agents **invoke** Shear; Shear runs deterministically offline. Show useful structural simplification, not a model-proposal pipeline or dashboard. Keep one real file on screen and name its project/revision. The source improvement is the centerpiece.

| Time | Content |
|---|---|
| 0–8 s | Real before-code; explain the specific unnecessary structure |
| 8–18 s | Actual Shear command, optionally invoked by Astra |
| 18–38 s | Actual diff and the clearer control flow |
| 38–50 s | Named passing checks and a preserved edge case |
| 50–58 s | Unchanged second run; concise evidence-backed Astra development credit |

Use measured results, not invented timings or complexity numbers. Label replay/cut/accelerated waits. Do not splice a rewrite from one run into another run's checks.

## Early smoke test

Before feature freeze, test a ten-second capture of a harmless command. Check available VHS, ttyd, FFmpeg, ffprobe, and screen-recording capabilities. Use VHS only if it works quickly; otherwise use the existing screen recorder. Never bypass OS permissions. Stream metadata does not establish readable playback.

Current initial environment observation: FFmpeg and ffprobe available; VHS and ttyd absent. Actual capture status belongs in `BUILD_LOG.md`, not assumed here.

## Capture and edit

1. Prepare a disposable checkout of the selected revision. Save Shear revision, source identity, commands, rule explanations, diff, and full check logs.
2. Frame a readable code region, usually unified diff or before/after switching rather than tiny side-by-side text. Hide unrelated tabs, notifications, and private data.
3. Record actual execution using repeatable CLI commands. No injected patch or printed success substitute.
4. Retain raw footage locally. Cut to 55–59 seconds with FFmpeg. Human narration is welcome; captions are an acceptable fallback. Target roughly 100–120 spoken words.
5. Inspect MP4 streams/duration with ffprobe and watch full playback at normal size, with audio when present. Check the final frame and caption/narration alignment.

Keep raw files under ignored `demo/raw/` and exports under ignored `demo/output/`. Commit small capture/edit scripts and sanitized evidence only. CLI tools own repeatable execution and encoding; desktop/browser tools can handle framing and playback QA when available. Ask for permissions or narration only when needed.

## Deliverables and time

Deliver a playable MP4 under one minute, captions/narration, reproducible production commands, input/run provenance, and an honest QA record. A capture script alone is not a finished video.

Pacific targets: feature freeze 3:45 PM, recording complete 4:30 PM, submission ready 5:00 PM; event deadline 5:30 PM. Confirm actual time before reallocating work. Follow `VIDEO_PROMPT.md` after a result qualifies. External upload needs an authorized destination; verify published links signed out.

For a three-minute finalist demo, show the same functionality and keep a clearly labeled event-day recording as fallback. Do not claim unsubmitted capabilities.
