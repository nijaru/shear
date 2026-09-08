# Shear kickoff prompt

Build Shear as a deterministic, offline structural formatter in Rust with tree-sitter. Read `AGENTS.md`, `docs/HANDOFF.md`, `docs/EXAMPLE_SELECTION.md`, and `docs/DEMO_VIDEO.md`. The current user explicitly rejected the earlier runtime Astra-proposal design. Astra develops and invokes Shear; Shear never calls Astra.

Implement formatter-style paths, default safe rewrites, check/diff/explain modes, surgical edits, language-specific safety rules, and fixed-point convergence. Preserve comments, behavior, and unrelated code. Use native formatting separately rather than implementing presentation rules. Source-language choice is independent of Rust implementation; begin with thoroughly tested rules in one language, then validate shared mechanics with a second backend.

The user has a separate session investigating demo targets. Do not duplicate it. Qualify a real one-file example only after an actual useful deterministic transformation and unchanged project checks. Synthetic fixtures exercise safety and idempotence; do not present them as real-code discovery or novelty evidence.

Test recording early, preserve actual commands/failures/results and development provenance, and follow `docs/VIDEO_PROMPT.md` once a useful checked result exists. Check Pacific time: feature freeze 3:45 PM, recording complete 4:30 PM, submission preparation 5:00 PM, deadline 5:30 PM.
