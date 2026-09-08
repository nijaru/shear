# Shear: independently researched demo targets

> **Historical research, not the current execution plan.** This independent report arrived during the September 8 product correction. Its source identities and reported baseline observations are retained as research evidence, not locally reverified results. Its model trials, extraction proposals, Go-only ranking, and CLI assumptions are superseded by `HANDOFF.md`. Current Shear is an offline Rust/tree-sitter formatter with Python rules; these Go targets require a future supported deterministic rule before they can qualify. Do not execute the old proposed model workflow.

Reviewed September 8, 2026 by a separate research session.

## Historical recommendation for the former model trial

**Try `dustin/go-humanize/number.go` -> `FormatFloat` first. Try `google/go-querystring/query/encode.go` -> `reflectValue` second.** Keep each surrounding repository intact for the actual Shear run. The video can stay on one changed file.

These are candidates, not already successful refactors. No Astra request, rewritten candidate, cognitive-complexity measurement, or before/after quality claim was produced in this research. The ranking concerns specific structural opportunities, validation effort, and presentation. Godotenv was not used as the default.

## Five files compared

| Input | Actual source observation | Improvement hypothesis | Verdict |
|---|---|---|---|
| `dustin/go-humanize`, `number.go`, `FormatFloat` | One function handles special values, format directives, sign normalization, rounding, grouping, and fractional padding. Directive parsing is deeply nested inside the renderer. | Separate interpreting formatting choices from applying them, and simplify the directive logic while preserving the numerical behavior. | First trial: clear input/output; focused upstream tests pass here. |
| `google/go-querystring`, `query/encode.go`, `reflectValue` | Field discovery, embedded-field scheduling, custom encoding, pointer handling, slice modes, and recursion coexist in one traversal. | Give collection encoding a coherent boundary without changing ordering or custom-encoder behavior. | Second trial: larger structural opportunity, more reflection-related validation. |
| `kelseyhightower/envconfig`, `envconfig.go`, `gatherInfo` | Pointer initialization, acronym/name splitting, prefixes, decoder detection, and recursive traversal share one function. | Separate name derivation from traversal; reduce nesting while preserving effects. | Reserve: meaningful opportunity but more mutable state to check. |
| `peterbourgon/ff`, `parse.go`, `parse` | Nested callbacks, precedence bookkeeping, config-file policy, and deferred file closure. Source already names its precedence stages. | Phase-local cleanup may help, but extraction alone is less persuasive. | Defer: callbacks and resource lifetime are poor first-MVP risks. |
| `hashicorp/go-version`, `version.go`, constructors | NewVersion/newVersion already have guards, a straight conversion loop, padding, and direct result construction. | No substantial structural improvement identified in the inspected constructors. | Reject these constructors for the showcase; not a judgment on the whole repo. |

## 1. First trial: human-readable number formatting

### Input identity

- Repository: https://github.com/dustin/go-humanize
- Default-branch snapshot: `4d1d9082551ec085912e7d2253a33ae547fca000`.
- Source: `number.go`, `FormatFloat`, **lines 66-187**: 122 physical lines including comments and blanks.
- Source blob: `e578ad329d3baeb8beda98c96cef64a57f50797f`.
- Source SHA-256: `aa7fcd58e0f8a119074eb8f3d8bce132cf7850c48c525767520b5c0ba48461b0`.
- Existing tests: `number_test.go`, blob `f4767c31cf023667881ded0579be0527cc408735`.
- Test SHA-256: `12f578129f3a7346c86ce07388324bf4dbba0dc2ac6891f87097424ba144bb5f`.
- go.mod declares Go 1.16, no third-party requirements.
- LICENSE is MIT. Retain it and the existing gorhill attribution in number.go when publishing copies/excerpts. Existing source is evaluation input, not original Shear implementation.

[Source](https://github.com/dustin/go-humanize/blob/4d1d9082551ec085912e7d2253a33ae547fca000/number.go) | [Tests](https://github.com/dustin/go-humanize/blob/4d1d9082551ec085912e7d2253a33ae547fca000/number_test.go) | [License](https://github.com/dustin/go-humanize/blob/4d1d9082551ec085912e7d2253a33ae547fca000/LICENSE)

### Specific opportunity

The format path (lines 89-146) mutates precision, decimal/thousands separators, positive-sign handling, and a remaining directive-index slice. Rendering follows at lines 148-186. Interpreting formatting choices and applying those choices is a meaningful boundary, not arbitrary extraction every twenty lines.

A good result makes the main flow easier to follow AND improves the extracted logic. Reject a result that merely moves the same tangle into a large helper with opaque return values. Preserve the numerical algorithm: replacing it with a supposedly equivalent formatter, changing rounding, or correcting historical quirks expands this task into behavior changes.

The ordinary demo input is `FormatFloat("#,###.##", 12345.6789)` -> `"12,345.68"`. Explain the transformation as making the formatting rules and rendering steps independently readable. Do not treat physical line count as evidence of bad code.

### What was actually executed

Direct network retrieval into the container failed, so **the complete repository was not cloned or tested**. Exact source/test texts were retrieved through the GitHub connector, copied locally, and checked against Git blob hashes from the pinned tree. Both matched; no upstream code/assertion was edited.

Environment: **Go 1.23.2, Linux amd64**. GOTOOLCHAIN=local, GOPROXY=off, GOSUMDB=off, GOWORK=off.

```sh
go test -count=1 -json -timeout=30s \
  -coverprofile=coverage.out number.go number_test.go
```

**Exit 0; TestFormatFloat passed in both initial and repeated runs.** One top-level test contains 19 formatting rows, one FormatInteger check, and two panic-existence checks. Do not call this 22 independently named tests.

Coverage: **100.0% statements in the explicitly compiled number.go**. This is not full-repository coverage, branch/path completeness, equivalence, or evidence that any candidate passed. The existing panic cases test panic occurrence, not its exact message. A measured repeat took 0.152 seconds wall time with warm build caches here; it is not predicted model/refactor latency.

### Original behavior observed separately

A research-only probe made 13 original calls, without a rewritten candidate. Useful results to freeze before a model trial:

| Original call | Observed result |
|---|---|
| Empty format, 12345.6789 | `"12,345.68"` |
| `"#\u202f###,##"`, 12345.6789 | `"12 345,68"` |
| `"#,###"`, 12345.6789 | `"12345,679"`: the one comma directive is decimal |
| Empty format, -2e-9 | `"-0.00"` |
| Empty format, -5e-10 or negative zero | `"0.00"` |
| `"-"`, finite 1 | String panic: `FormatFloat(): invalid positive sign directive` |
| `"-"`, NaN | `"NaN"`, no panic |
| `"-"`, positive/negative infinity | `"Infinity"` / `"-Infinity"`, no panic |
| `"0.01"`, 12345.6789 | Thousands-separator validation panic |
| `"#.##########"`, finite 1 | Runtime bounds panic: precision 10 exceeds the lookup arrays |
| Same precision-10 format, NaN | `"NaN"`, no panic |

Moving format validation before the special-number checks changes observable behavior. Use this as a preservation case, not an excuse to fix the original. Some descriptive comments differ from observed rounding/infinity spelling; derive the oracle from actual behavior.

For differential input, represent floats by exact bits (for example, hex strings decoded with math.Float64frombits), not ordinary JSON numbers. This supports NaN/infinity and preserves signed zero. Compare returned strings and panic occurrence/type/message under the same toolchain. Crashes/timeouts without a valid observation are not matches. Go runtime panic wording can differ across versions.

### Next step in the full repository

Use a new destination, preserving existing work:

```sh
git clone https://github.com/dustin/go-humanize.git /tmp/shear-demo-humanize
git -C /tmp/shear-demo-humanize checkout --detach 4d1d9082551ec085912e7d2253a33ae547fca000
cd /tmp/shear-demo-humanize
go test -mod=readonly -count=1 -json -timeout=60s ./...
go test -mod=readonly -count=1 -run '^TestFormatFloat$' \
  -coverprofile=/tmp/shear-humanize.cover .
```

These full-repository/package commands are **next checks, not runs performed here**. Then use the actual implemented Shear CLI on number.go/FormatFloat. Freeze characterization checks before generating a candidate; keep FormatInteger, globals, imports, and attribution unchanged. Include every new helper in the readability/metrics review.

## 2. Second trial: URL query encoding

- Snapshot: `965d79f2113ea0ff039d29828a08a616a0223d48` in `google/go-querystring`.
- Target: `query/encode.go`, `reflectValue`; blob `ffb2097f56914c289e50b3c4c899716af51a559f`.
- Tests: `query/encode_test.go`, blob `241e4f1eeef86ae7ab602324d5b2400a59591223`.
- LICENSE: BSD-3-Clause. Retain notices; do not imply Google endorsement.
- Status: full target source and relevant test sections inspected; **no test or refactor executed here**. Tests import google/go-cmp/cmp; retain the dependency and original assertions.

[Source](https://github.com/google/go-querystring/blob/965d79f2113ea0ff039d29828a08a616a0223d48/query/encode.go) | [Tests](https://github.com/google/go-querystring/blob/965d79f2113ea0ff039d29828a08a616a0223d48/query/encode_test.go) | [License](https://github.com/google/go-querystring/blob/965d79f2113ea0ff039d29828a08a616a0223d48/LICENSE)

The collection path combines empty-value handling, prioritized delimiter selection, key suffixing, a first-element flag, and joined/repeated-value loops. Extracting a coherent collection-encoding operation could simplify the main traversal. Replacing only the first-element flag is too slight for the showcase.

Inspected test sections include TestValues_BasicTypes, TestValues_Pointers, TestValues_Slices, TestValues_NestedTypes, TestValues_OmitEmpty, and part of TestValues_EmbeddedStructs. Preserve:

- Empty slice versus array-of-empty-strings behavior, and nil pointer-to-slice versus empty slice.
- Non-nil pointer to empty string under omitempty.
- Comma/space/semicolon/brackets/custom-delimiter/numbered option precedence.
- Duplicate-key value ordering: the embedded test expects outer V="b" before embedded V="a".
- Custom Encoder dispatch, pointer behavior, and processing order.

Exercise through exported query.Values. Compare url.Values semantically while retaining each value slice's order; do not rely on map iteration order. Run the unchanged full baseline with `go test -mod=readonly -count=1 -json -timeout=60s ./...` in the pinned repository. No substitutions for unavailable test dependencies.

## Reserve / rejected source snapshots

**Envconfig reserve:** `7834011875d613aec60c606b52c2b0fe8949fe91`, `envconfig.go`, gatherInfo, blob `3a2aac7adaa33a1be6f06d40c2313afdafb32ef1`. Source and envconfig_test.go lines 1-240 inspected, including TestProcess/TestParseErrorBool. Tests exercise acronym splitting, alternate names, nested prefixes, and decoder-bearing structs; tests mutate the child process environment. No run here. Source header identifies MIT; inspect/retain LICENSE before publication. Preserve nil-pointer initialization and decoder precedence. [Source](https://github.com/kelseyhightower/envconfig/blob/7834011875d613aec60c606b52c2b0fe8949fe91/envconfig.go)

**FF deferred:** `012e82713b8ac4f2a59810ababd78e034c57f8c6`, `parse.go`, parse, blob `a18fc0782a6b9bc1f9068fba4119b0e6f43f2f9b`. Full source inspected; no tests run or license qualification. Existing named stages already explain precedence; callbacks, repeated SetValue, and deferred close make this a riskier first target. [Source](https://github.com/peterbourgon/ff/blob/012e82713b8ac4f2a59810ababd78e034c57f8c6/parse.go)

**Go-version constructors rejected:** `4f9dad4645ee2935418c5eb169ab05976abeaf6b`, `version.go`, source lines 1-180, blob `34735323e919fabf2ac62785949fc44a574e67f1`. No sufficiently substantial cleanup identified in NewVersion/newVersion. No tests run. Header identifies MPL-2.0; full licensing not reviewed because this candidate was rejected on demo grounds. [Source](https://github.com/hashicorp/go-version/blob/4f9dad4645ee2935418c5eb169ab05976abeaf6b/version.go)

## Historical integration instructions (superseded)

Fetch origin and read `git show origin/main:docs/TARGET_RESEARCH.md`; this does not require switching/resetting the active checkout. Integrate documentation normally when convenient.

The active session owns the full repository baseline, actual Astra trial, readability judgment, and video. Try the general workflow on the first candidate, then the second if needed. Keep failed attempts and the complete changed code. Select the showcase only after a checked result earns it; do not hard-code a known patch or tune the tool to one file.
