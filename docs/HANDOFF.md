# Shear: hackathon handoff

Updated September 8, 2026. This is a planning specification, not a record of implemented behavior. It supersedes earlier handoffs and their predetermined godotenv demo.

## 1. Outcome and scope

Build a structural formatter that produces a useful local refactor, checks it, and shows a reviewable patch. Astra participates in development and generates the runtime refactor. The main demo is one understandable change in **one file from a real project**.

Keep three scopes distinct:

| Scope | Plan |
|---|---|
| Inspection | Read the project, related declarations, and tests to understand the selected file. A repository scan can help find candidates. |
| Editing | One selected file. Start with one function body and optional same-file helpers; bounded related edits can follow. |
| Validation | Run applicable package/project checks, not a detached file that omits its real dependencies and callers. |

A single-file demo is a presentation choice, not a claim that all useful refactoring is single-file. Whole-project rewriting is unnecessary for this event. Retain a second candidate for evaluation/fallback, not another obligatory segment in the video.

No real example has qualified yet. Use `EXAMPLE_SELECTION.md` to find one. Generated fixtures exercise the tool and verifier; they are not evidence that a real project has maintainability problems.

## 2. Implementation choices

Use Go for both host and source support. Its native source tooling minimizes integration work for this build; this is not a model language-performance claim.

Use `go/parser`, `go/ast`, and `go/token` for source ranges, `go/format` for formatting, and a small `gocognit` integration for one recorded structural signal. Add `go/packages` only when package/type context helps. Use the official OpenAI Go SDK and the documented Responses interface. Verify the actual model ID, supported settings, structured-output handling, and event API access. Codex access alone does not establish product API access. Pin the dependencies that build successfully rather than copying version numbers from old notes. References: `SOURCES.md`.

One module, a small CLI entry point, and a few internal files are sufficient. Prefer a testable proposer interface to a provider framework. Keep the CLI easy to call from a script. The agent may simplify the proposed command surface below:

```text
shear scan --repo /path/to/project
shear refactor --repo /path/to/project --file relative/file.go
shear show RUN_ID
shear apply RUN_ID
```

These are interface sketches, not implemented commands. Explicit function selection may be the first implementation. A file-level command can select an eligible hotspot once that path works. Keep the executable generic: no repository-name branches, hardcoded patches, or special acceptance exceptions for the showcase.

## 3. Smallest complete loop

1. Select a clean, committed, trusted local checkout; record the revision, source hashes, and validation profile.
2. Establish a passing baseline and identify the permitted source region. Start from formatted code or disclose a separate formatting-only baseline.
3. Send the selected source and relevant read-only context to Astra. Request a body replacement, optional same-file private helpers, a concise explanation, or abstention. The program owns destinations and insertion points.
4. Assemble in scratch state, reparse, format, and enforce the final edit boundary. Check protected files again after test execution.
5. Run the same protected checks. Add a fixture-specific differential comparison when feasible.
6. Measure the entire changed unit, save the real patch and report, and return an explicit result.
7. Apply only the saved bytes after rechecking input identity. Do not regenerate during application or overwrite intervening user work.

Start with one proposal and at most one repair, bounded by explicit process/model deadlines. This is a practical starting budget, not an optimality claim. Treat incomplete output, refusal, timeout, missing checks, and API failure as non-success. A repair may use diagnostics but cannot change the tests or expand its write authority.

Protect original signatures, receiver/type parameters, imports in the initial path, test files, manifests, build settings, and unrelated declarations. Initially allow up to two private helper functions. Validate helper declarations and name collisions; reject extra declarations or directives disguised inside a replacement. Relax a restriction only for a concrete useful case with corresponding checks, not to make a failing demo appear successful.

## 4. What the checks mean

Use the project's applicable checks, with explicit package scope and time limits recorded before generation. Execute commands as fixed executable/argument arrays. Strip API credentials from child environments; this reduces accidental exposure but does not isolate the host filesystem. Support trusted local code, not arbitrary uploaded repositories.

Preserve test identities, outcomes, and protected source. Keep baseline failures blocked. Establish any new characterization checks against the original before using them to judge the candidate. Differential tests compare selected observations on fixed inputs; they are an optional, clearly named example-specific profile, not a universal feature or proof.

Measure nesting, complexity from a named provider/version, changed-unit size, helper count, and diff size. Include the original function versus its rewritten caller **plus every introduced helper**, and report any other edited functions. Extraction can still improve a metric merely by resetting nesting penalties. A human review of the full result decides whether it is worth showcasing.

A candidate must pass the hard checks and make a substantive, explained structural improvement to qualify for review. Avoid a universal weighted quality score or arbitrary reductions as the only goal. Report accepted-for-review, rejected, unchanged, unsupported, baseline-blocked, and execution-error separately.

Before recording, test known-bad candidates: changed observable output, protected-file edit, malformed response, no-op, and stale application. These are labeled verifier tests. Keep ordinary unit tests runnable without a model key.

## 5. Build sequence

Begin API/environment checks and real-file discovery in parallel where possible. Give initial example discovery roughly 20–30 minutes; this is a work allocation, not a reason to declare an unqualified winner. Use an existing analyzer or small source scan, not a new search platform.

The first implementation milestone is one real Astra request followed by a checked patch. Add reliable application, rejection cases, and readable output next. Additional scans, a second example, differential expansion, and model comparisons follow only if the core works. Keep optional UI to a local diff view backed by real artifacts; it must not become a dashboard-first project.

One main Codex/Astra session owns shared types, dependencies, and integration. A bounded worker may inspect candidates or write independent tests. A video worker can take over once real artifacts exist. Assign non-overlapping files and make the integration owner explicit.

## 6. Event and time budget

Source: the participant guide supplied by the user, not inferred from the public listing. Tuesday, September 8, 2026, Pacific time:

- Hacking: 10:30 AM–5:30 PM, including lunch and submission preparation.
- Required: public repository and accessible one-minute demo video.
- Screening: OpenAI selects five finalists; stage format is three minutes plus two minutes of questions.
- Four equal weights: Astra in development, Astra in the project, live demo, technical implementation.
- Demonstrate event-built functionality; identify upstream inputs and earlier planning. Respect source/asset rights. A dashboard cannot be the main feature.
- The guide lists stage demos at both 6:45 and 7:00 PM; confirm that detail separately.

Read the actual local time before allocating work. Preserve a 3:45 PM feature-freeze target, 4:30 PM recording-complete target, and 5:00 PM submission target, with 5:30 PM the deadline. If starting late, compress scope instead of following missed milestones or changing languages. Start a ten-second recording smoke test early; do not leave all media setup until feature freeze.

## 7. Evidence and submission

Record a few concrete Astra development contributions with their patches/checks, not generated-line counts. Save source revision, Shear revision, actual model/settings, attempts, measured timing, full patch, check logs, and a human readability decision for each showcased run. Separate local raw artifacts from sanitized public evidence.

Update the README only with supported behavior and tested commands. Record how to acquire the exact upstream input, its license/notice, and the chosen checks. Do not submit upstream changes without a separate request. Verify the public repo and video in a signed-out session. Do not claim novelty of the entire AI-refactoring category, arbitrary behavioral equivalence, or older-model inferiority without evidence.

Deliverables: working CLI, real-file selection record, checked patch, reproducible checks, one-minute video, sanitized development evidence, and an accurate README. The video plan and production prompt are in this directory.
