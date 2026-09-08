# Sources and evidence boundaries

Updated September 8, 2026. These references support tooling capabilities. The proposed architecture, search time boxes, and storyboard are design choices, not empirical results.

## User-provided basis

The participant guide pasted in the planning conversation supplies the September 8 event schedule, 10:30 AM–5:30 PM build window, public-repository requirement, one-minute submission video, three-minute finalist demo plus two-minute Q&A, and four equal judging weights. It prohibits a dashboard as the main feature and requires identifying event-built work and respecting source/asset rights. It gives two different finalist start times; that remains an organizer question. Private venue/access details are intentionally omitted.

The user's latest decisions are: the name is Shear; the repository already exists; explore real files without assuming godotenv is good; one file is sufficient for the demo; repository-wide inspection may provide context; and use Codex with CLI/desktop capabilities to produce the video.

Public event and submission reference: https://cerebralvalley.ai/e/openai-gpt-6-astra-sf . This handoff does not claim the public page independently confirms every detail of the supplied guide.

## Primary tooling references inspected

- Go AST: https://pkg.go.dev/go/ast
- Typed package loading: https://pkg.go.dev/golang.org/x/tools/go/packages
- Gocognit and native-AST metrics: https://pkg.go.dev/github.com/uudashr/gocognit
- Official OpenAI SDKs: https://developers.openai.com/api/docs/libraries
- Codex project instructions: https://developers.openai.com/codex/guides/agents-md/ (currently redirects to https://learn.chatgpt.com/docs/agent-configuration/agents-md)
- Computer Use availability/permissions: https://developers.openai.com/codex/app/computer-use/ (currently redirects to https://learn.chatgpt.com/docs/computer-use)
- VHS terminal scripting, dependencies, output formats, and waits: https://github.com/charmbracelet/vhs/blob/08ec20e6cc826970cde5e34c9596dfe67cf0dcbc/README.md
- FFmpeg encoding, mapping, and editing options: https://ffmpeg.org/ffmpeg.html
- ffprobe metadata and duration inspection: https://ffmpeg.org/ffprobe.html

Choose and record actual working dependency/model versions on the build machine. Documentation support does not establish availability in a specific Codex session.

## Not established by this handoff

No candidate repository has qualified; no baseline, generated refactor, benchmark, user-machine setup, capture pipeline, or finished video has been tested here. Old godotenv source inspection is not a selection decision. The local implementation session must supply the candidate evidence and actual results.

The supplied earlier handoffs were reviewed for context, but the documents in this repository replace their conflicting name, fixture, scope, and workflow instructions. The repository update is documentation-only and retains the planning provenance.
