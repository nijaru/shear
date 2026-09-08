# Qualify the real example

The user's separate session owns discovery. No repository or language is presumed to need cleanup, and no file is selected until a useful deterministic Shear rewrite passes applicable checks. Godotenv has no preferred status.

## Discovery handoff

Inspect three to five concrete files in a few real projects. Source-language choice is independent of Rust implementation; check actual Shear support before expecting a rule to run. Prefer a specific avoidable control-flow problem that is understandable on screen, not a long but inherently complex parser or dispatch table.

Record findings in `docs/CANDIDATES.md`:

| Field | Required evidence |
|---|---|
| Identity | Public repository URL, exact commit, file, symbols/line ranges, source hash |
| Problem | Precise source references and why the structure is unnecessarily difficult |
| Rewrite opportunity | Applicable deterministic rule or proposed rule with explicit safety conditions |
| Context | Callers, types, effects, language version, build needs |
| Baseline | Exact command/arguments, scope, exit status, elapsed time; say “not run” when applicable |
| Comparison | Does a native formatter or existing autofixer already perform this change? |
| Provenance | License/notice paths, acquisition method, excerpt/publication requirements |
| Verdict | Explore, ready for trial, rejected with reason, or qualified after checked result |

Do not choose on complexity score alone, intentionally worsen code, or present a generated fixture as an independently existing project.

## Qualification

1. Preserve the original checkout. Create a disposable copy at the exact revision.
2. Write the problem in one sentence. Identify an ordinary case and a behavior-sensitive edge case.
3. Run the project's applicable checks before rewriting. Freeze any new characterization checks against the original first.
4. Run Shear on the selected file. Preserve command output and the entire diff.
5. Run the same checks and Shear again. Confirm no second-run changes and no test/manifests/unrelated-file mutations.
6. Inspect the complete changed reading path. A smaller metric alone is not readability evidence.
7. Qualify only a useful result. Keep unsuccessful attempts and one fallback candidate.

Use the actual project for validation rather than detaching a file from its package. Trusted local projects only; test commands are not sandboxed. Do not submit upstream changes without separate authority.

## Selected example

After qualification, create `docs/SELECTED_EXAMPLE.md` with exact acquisition/validation commands, source and Shear revisions, rule names, before/after explanation, full diff location, test logs, limitations, and attribution. No runtime model ID belongs in Shear's transformation evidence; record Astra's development or invocation contribution separately.

The one-minute video stays on this one file. Existing tools overlap these initial transformations; novelty must be argued from demonstrated differences, not claimed for structural rewriting as a category.
