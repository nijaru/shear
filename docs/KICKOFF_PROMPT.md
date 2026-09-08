# Codex kickoff prompt

Use this in a fresh local Codex session opened at the Shear repository, with the available Astra model selected. These are development instructions, not the runtime refactoring prompt.

---

Build Shear for the September 8 GPT-6 Astra hackathon. Read `AGENTS.md`, `docs/HANDOFF.md`, `docs/EXAMPLE_SELECTION.md`, and `docs/DEMO_VIDEO.md`. Inspect the current checkout and local changes first; preserve existing work. Use Go for the implementation and Go source support. Optimize for a useful running demo, not a long-term framework or another language comparison.

Find a strong real-code example rather than assuming godotenv or any other repository is suitable. Inspect three to five candidate files in a small number of real Go projects. For each, record exact source/revision, a concrete structural issue with line references, relevant context, applicable tests and actual baseline results, license/publication requirements, and whether the change would be understandable on screen. Use a bounded discovery pass, ideally alongside environment/API setup. Put the evidence in `docs/CANDIDATES.md`. Select one primary file only after a useful checked refactor, and keep one fallback. Do not choose on complexity score alone, manufacture a bad input, or force an unhelpful extraction.

Inspect the surrounding project for context and run its applicable package/project checks. Keep the showcased edit in one file. Start with one function and optional private same-file helpers; broaden to related functions only when useful. The core path is a real Astra proposal, host-enforced edit boundaries, scratch assembly, unchanged passing checks, full changed-unit measurements, an actual readable diff, and explicit application of the exact checked patch. Preserve tests, interfaces, dependencies, meaningful comments, and unrelated code. No-op, missing checks, stale input, or API failure must not count as success.

Verify product API access separately from Codex access. Keep the module small and the command scriptable. Use existing Go tooling and a bounded proposal/repair loop. Test the verifier with generated fixtures and clearly labeled invalid patches. Freeze any new behavior checks against the original first. Evaluate the complete reading path, including helpers, and report evidence rather than universal equivalence or unmeasured model superiority.

Begin a short video-path smoke test early. For the final one-minute demo, use CLI tools for repeatable execution, capture/editing and media checks; use desktop/browser tools for framing and playback QA when actually available. Show one file's real before/after and checks, label cut/accelerated waits or replay, and save the raw run evidence. Use `docs/VIDEO_PROMPT.md` for the production task once the result exists.

Check the actual Pacific time and preserve recording/submission time: feature-freeze target 3:45 PM, submission target 5:00 PM, deadline 5:30 PM. Adapt scope if late. Keep sanitized evidence of Astra's development contributions and distinguish earlier planning, upstream inputs, and event-built implementation. One main session owns integration; delegate only bounded, disjoint work. Start executing, report findings and real blockers as you go, and do not stop at another plan or ask me to reconfirm these decisions.
