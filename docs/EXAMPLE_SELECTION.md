# Find the real file worth demonstrating

## Decision

No repository or file is selected. **Godotenv is not the default or presumed to need cleanup.** Earlier notes identified testable parsing code but did not establish a worthwhile structural improvement. Do not reuse that conclusion as evidence.

The goal is one file from a real project whose problem and improvement can be explained on screen. Keep the project intact for context and validation. Copying one file out of its package just to make it easy to edit weakens the test.

## 1. Discover candidates from source

Inspect three to five concrete files across a small number of accessible Go projects. Public, reusable inputs with a simple local test path are preferable for the submission. A user's project may be a candidate only when its chosen input can be disclosed; private access does not imply permission to publish source.

Useful search areas include CLI argument/configuration handling, request validation, import/export transformations, and data normalization. These are search directions, not assertions that those categories or particular repositories have bad code.

Use metrics to narrow the search, then read the full functions and nearby tests. Look for a precise issue: the same validation implemented in several branches, duplicated decisions with synchronized state, normal flow obscured by avoidable nesting, or two responsibilities whose interleaving makes changes hard to follow.

Distinguish intrinsic complexity from unnecessary structure. A parser state machine, exhaustive dispatch, or long table may be the clearest faithful implementation. File size, popularity, project age, and a high complexity score are not enough to nominate it. A single combined conditional, whitespace change, or extraction with no clearer reading path is too slight for the headline demo.

## 2. Produce a small evidence table

Create `docs/CANDIDATES.md` during discovery. For every candidate, record:

| Field | Evidence needed |
|---|---|
| Identity | Repository URL, exact commit, file, symbols/line ranges, source hash |
| Concrete issue | A short source excerpt or exact line references and a plain-language explanation |
| Improvement hypothesis | What could become easier to understand without prescribing the final implementation |
| Context | Relevant types/callers, related files, public interface, effects or special build needs |
| Baseline | Exact commands, test scope, exit status, duration, and meaningful coverage limitations |
| Validation opportunity | Existing cases plus a small frozen characterization/differential check if practical |
| Presentation | The before/after region that can be read at normal video playback size |
| Provenance | License, notice path, acquisition method, publication permissions |
| Verdict | Explore, ready for a model trial, rejected with reason, or qualified after an actual result |

Use 'not run' and 'unknown' where appropriate. Do not invent scores, tests, timestamps, or a successful refactor. Pin a current usable revision rather than choosing an old one solely to make the code look worse. A historical revision is acceptable when clearly disclosed and selected for a defensible reason.

## 3. Qualify before committing the story

For the best two candidates, reproduce applicable baseline checks in the actual environment and inspect whether they exercise the targeted behavior. Prefer a file whose test path is fast enough to repeat and does not depend on external services. If only a subset runs, name it and disclose the untested scope.

Write the problem in one sentence before generating a candidate. Identify an ordinary example input and an edge case that a careless refactor could break. Determine whether native formatting or an available mechanical fix already handles the proposed change; do not claim Astra is required for a trivial existing fix.

Try the same general Shear workflow on the candidate. Judge the full diff, including helpers and any moved logic. Choose the showcase only after obtaining a meaningful improvement that passes the fixed checks and is preferable to read. Retain another independently selected file as a fallback; it need not appear in the video. Preserve unsuccessful attempts in the evaluation record.

Do not force a predetermined repository or lower acceptance requirements to manufacture a win. If no real example qualifies in the available time, report that limitation and continue the tool's integration tests; a synthetic exercise must remain explicitly synthetic.

## 4. What generated code is for

Use small generated fixtures for source-boundary tests, no-op/abstention cases, malformed output, stale patches, and intentionally wrong behavior. Generate inputs from a written behavior contract and establish expectations against the original before candidate evaluation.

A differential adapter belongs to the selected example/profile, separate from the generic refactorer. Compare the relevant outputs, errors, and explicit effects on identical inputs. Treat crashes, missing results, and timeouts as failures or documented unsupported cases, not matches. Preserve existing quirks: this task is structural cleanup, not an unannounced bug fix.

Cases used to repair a candidate are not held out. Finite generated checks are useful evidence, not exhaustive proof. Do not deliberately worsen real source or submit an agent-invented 'realistic file' as independently existing project code.

## 5. Selected example manifest

Once qualified, create `docs/SELECTED_EXAMPLE.md` with the chosen evidence, exact acquisition and validation commands, before/after explanation, raw-result locations, full diff, limitations, and fallback identity. Keep cloned projects and large raw media outside tracked source or explicitly ignored. Retain required notices in any published excerpts/copies.

The video should stay on this file: identify the real project, point to the specific structural issue, run Shear, show the relevant changed code, and show the checks. Additional project scans can run off-camera or appear briefly only if they help explain target selection.
