# Shear

A structural formatter for code.

Shear is being built for the September 8, 2026 GPT-6 Astra hackathon. The intended workflow is to find a worthwhile structural cleanup, ask Astra for a bounded refactor, check the result, and present the exact patch for review.

**Status at this handoff:** planning documents only. No implementation, qualified demo file, successful refactor, benchmark result, or demo video is claimed.

## Start here

Open this repository in a fresh local Codex session, select the available Astra model, and use [the kickoff prompt](docs/KICKOFF_PROMPT.md).

| Document | Purpose |
|---|---|
| [Handoff](docs/HANDOFF.md) | Product scope, build sequence, checks, and event constraints |
| [Example selection](docs/EXAMPLE_SELECTION.md) | Find and qualify a real file rather than assume a repository needs cleanup |
| [Demo video](docs/DEMO_VIDEO.md) | CLI-first capture, optional desktop use, editing, and submission QA |
| [Video agent prompt](docs/VIDEO_PROMPT.md) | Produce the recording once the tool has a qualified result |
| [Sources](docs/SOURCES.md) | Event-source distinction and primary tool references |

The main demo will follow one file from a real project. Repository inspection provides context; validation uses its real package/project. Broader project scanning is useful when it saves selection effort, but a multi-file demonstration is not required.

These documents consolidate earlier planning and the latest scope decisions. Keep that provenance distinct from implementation and recorded results produced during the event. Update this README with tested commands and actual limitations as the product becomes usable.
