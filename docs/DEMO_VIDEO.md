# Demo video: real execution, scripted production

## 1. Recommended division of work

Use **Codex with CLI tools for the repeatable work**, plus **desktop/browser use for framing and visual QA** when available. This is a production workflow, not a reason to add computer use to Shear itself.

| Work | Preferred route |
|---|---|
| Prepare a clean example checkout, run Shear, save evidence | CLI scripts with checked exit codes |
| Capture terminal interaction repeatedly | VHS if already working, otherwise ordinary screen recording |
| Present the source diff | Readable terminal/editor, or a minimal local HTML view of real saved artifacts |
| Trim, join, encode, inspect duration/streams | FFmpeg and ffprobe |
| Arrange windows, inspect readability, play the result | Desktop/browser tools when enabled; human fallback |
| Approve OS permissions, record narration, approve publication | User involvement as needed |

VHS scripts terminal interaction and supports MP4 output; it requires ttyd and FFmpeg. It executes the scripted commands rather than merely drawing a fake terminal. Inspect its installed documentation and test a short tape before adopting it. A tape is not evidence that any particular model request succeeded. Sources: `SOURCES.md`.

Computer Use availability depends on the installed app, plugin, account, and permissions. Official documentation describes Screen Recording and Accessibility permissions on macOS. Do not assume a CLI/cloud Codex session has desktop control or that screenshots automatically provide a recording pipeline. Prefer the available structured tool for each task.

## 2. Test the media path early

Before the final recording, make a ten-second capture of a harmless command, encode it, and play it at normal size. Verify available `vhs`, `ttyd`, `ffmpeg`, and `ffprobe` commands without printing secrets. Confirm microphone and screen permissions through the actual recording app. Use an existing screen recorder if VHS setup is troublesome; the media stack must not displace the product.

A 1920x1080, 30 fps video with large text is a proposed production target, not an event requirement. Frame roughly one readable code region at a time. Use a focused unified diff or before/after switching when side-by-side text becomes too small. Keep helper code available and show it when material to the explanation.

Use a clean terminal/editor layout, disabled notifications, synthetic credentials, and no unrelated tabs. The screen should contain the product and example, not the entire development conversation. Human narration is the simplest option; captions are a viable fallback. Target about 100–120 spoken words and time the actual reading. Music and elaborate motion graphics are unnecessary.

## 3. Capture one real run

Use the selected example's exact snapshot and a disposable working checkout. Save the full raw recording, command transcript, source/Shear revisions, model settings, actual timings, checks, diff, and final report. Do not log the API key. Build and dependency caches can be warm; model generation must be real for a segment presented as a fresh run.

Write a small capture/rehearsal script during implementation once the CLI exists. Wait on command completion or verified terminal output instead of guessed model delays. A VHS tape may invoke that script. Hidden setup may select the disposable workspace; it must not inject a successful patch or echo fabricated validation output.

Record the refactor first and cut a clear story from its actual result. When showing a saved result, label it as a recorded run/replay. Label accelerated waits or cuts, and retain real duration in the evidence. Do not quietly splice generation from one run into validation from another. If multiple takes are used, preserve run identities and do not claim first-try reliability.

## 4. One-minute storyboard

Target about 55–59 seconds to leave a small encoding margin. Use one selected file throughout.

| Time | Screen | Purpose |
|---|---|---|
| 0–7 s | Real file, project/revision attribution, one highlighted region | Explain its concrete structural problem |
| 7–17 s | Actual Shear invocation and relevant progress | Show that this is a working tool |
| 17–37 s | Actual before/after diff, with the important helper/flow visible | Explain what became easier to follow |
| 37–49 s | Named passing checks and one preserved edge case | Establish the evidence and its limits |
| 49–58 s | Apply the checked patch or inspect the final file | Close on useful output; briefly credit actual Astra development work |

These are edit allocations, not assumed API latency. The source improvement is the centerpiece. A seeded rejection belongs in the longer stage demo unless it is exceptionally quick to explain. Show 'injected verifier test' when using one. Use only measured counts and features actually implemented.

## 5. Editing and export

Let Codex prepare an edit plan from the real footage and measured timings, then use FFmpeg for deterministic cuts/concatenation and encoding. Keep commands in a small script so the export can be repeated. Avoid building a custom video application. Record narration separately when that is easier; align it with the actual edit and verify the ending is intact.

The following is an export example **after** an edited source file exists. Adapt input/output paths, confirm encoder availability, and create the output directory. It preserves an existing audio stream when present; it does not create narration. It intentionally does not silently truncate to sixty seconds.

```sh
mkdir -p demo/output
ffmpeg -n -i demo/raw/edited.mov \
  -map 0:v:0 -map '0:a:0?' \
  -vf 'scale=trunc(iw/2)*2:trunc(ih/2)*2' \
  -c:v libx264 -crf 18 -preset medium -pix_fmt yuv420p \
  -c:a aac -b:a 160k -movflags +faststart \
  demo/output/shear-demo.mp4

ffprobe -v error \
  -show_entries format=duration,size:stream=index,codec_type,codec_name,width,height,pix_fmt \
  -of json demo/output/shear-demo.mp4
```

If the duration exceeds the one-minute requirement, revise the edit and export again. Inspect start, middle, and final frames, then watch the entire result at normal playback size with audio. Stream metadata alone cannot confirm that text is legible, narration matches, or the demonstration is truthful.

Keep raw video, credentials, and local source artifacts out of normal Git commits. Commit small recording/edit scripts, narration text, input provenance, and sanitized evidence when appropriate. Use the user's chosen sharing route; verify the uploaded video in a signed-out browser. Ask before a new external upload when no destination/authorization has been given.

## 6. Production deliverables and stage fallback

Produce the final MP4, narration/captions, capture/edit instructions, selected input and run identity, and a short QA record with duration, readability, audio, and link-access checks. A source script without a playable video is not a completed recording task.

For the three-minute stage demo, invoke Shear early, explain its contract while it runs, then show the same file's result and checks. Keep an honestly labeled event-day recording for network failure. Stick to submitted functionality and confirm any post-deadline changes with organizers.

The official guide's one-minute submission, public repository, and four judging categories are summarized in `HANDOFF.md`. Screen access credentials from that guide do not belong in the repository or video.
