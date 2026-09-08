# Codex video-production prompt

Use after Shear has a real checked result. Run in the existing local project environment with any available screen/desktop permissions reviewed by the user.

---

Produce the one-minute Shear hackathon demo using the implemented tool and real run artifacts. Read `docs/DEMO_VIDEO.md`, `docs/SELECTED_EXAMPLE.md` if present, and the actual README/CLI. Confirm the chosen source revision, real diff, passing checks, and saved run identity. If those do not exist, report that blocker and help complete the minimum real run; do not fabricate footage or results.

Keep the story on one file from a real project. Show the concrete problem, invoke Shear, explain the actual structural improvement, show named checks and one preserved behavior, and finish with the reviewed patch. Include a concise, evidence-backed note on Astra's development contribution.

Use CLI tools to prepare a disposable input, capture execution, edit, encode, and inspect media. Use VHS only if a short smoke test works; otherwise use the existing screen recorder. Use desktop/browser tools for arranging the view and checking legibility when available. Do not assume desktop access, bypass OS permissions, or create a new video framework. Ask for human narration/permissions only when genuinely needed; captions are an acceptable fallback.

Retain the raw recording and logs. Label replay and accelerated/cut waits. Keep all depicted results tied to the same identified run, including full helper code in review. Never substitute printed successes or a hardcoded patch for execution.

Export a playable MP4 under sixty seconds, preferably 55–59 seconds, with readable code and intelligible narration or captions. Verify duration/streams using ffprobe and watch the full playback; inspect frames as needed. Save the capture/edit commands, narration/captions, source/run provenance, and QA notes. Do not claim recording or review you could not perform. Prepare the file for the user's sharing route; obtain approval before an unrequested external upload. Keep private credentials, unrelated screens, and large raw artifacts out of Git.
