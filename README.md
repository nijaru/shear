# Shear

[![Check](https://github.com/nijaru/shear/actions/workflows/check.yml/badge.svg)](https://github.com/nijaru/shear/actions/workflows/check.yml)

A deterministic, offline structural formatter for Python and JavaScript.

Shear removes mechanically unnecessary control structure — redundant `else` branches, nested conditions, duplicated branch tails — while preserving behavior. It runs locally with no model or API key, like any other formatter. Use it after editing code, then run your native formatter, linter, and tests.

## Rules

| Rule | Rewrites | Python | JavaScript |
|---|---|---|---|
| `redundant-else` | `else` after a branch that always returns, raises, breaks, or continues | Yes | Yes |
| `redundant-elif` | `elif` after an exiting branch | Yes | — |
| `merge-nested-if` | an `if` that is the only statement of another `if` | Yes | Yes |
| `guard-clause` | an exiting `else` into an early guard clause | Yes | Yes |
| `shared-branch-tail` | identical trailing statements from both branches, moved after the conditional | Yes | — |

Rules compose to a fixed point, so a `redundant-else` can expose a `merge-nested-if` on the next pass. Each rule owns its own safety conditions; uncertain cases are skipped rather than guessed. Per-rule contracts are in [docs/RULES.md](docs/RULES.md).

## Install

Requires a recent stable Rust toolchain (edition 2024; development is checked with Rust 1.98).

```sh
cargo install --path .
```

Or build in place:

```sh
cargo build --release
./target/release/shear --help
```

Neither Python nor Node is required to run the formatter. The behavioral test suite uses `python3` and Node.

## Usage

```text
shear [FLAGS] [PATHS]...
```

| Flag | Effect |
|---|---|
| *(none)* | Write simplifications to the named files or directory tree |
| `--check` | Write nothing; exit 1 when simplifications are available, 0 otherwise |
| `--diff` | Print unified diffs without writing |
| `--explain` | Report the rules applied and their original pass-local line numbers |

Errors exit 2. With no paths, Shear scans the current directory. Directory discovery respects ignore files and skips hidden files, symlinks, and nested `node_modules`; explicitly named files bypass ignore rules.

```sh
# Preview changes to one file
shear --diff --explain src/handler.py

# Check a whole project in CI without writing
shear --check .

# Simplify a mixed-language tree in one invocation
shear src/api.py src/client.js
```

For example:

```python
if authorized:
    if enabled:
        run()
```

becomes:

```python
if authorized and enabled:
    run()
```

After running Shear, run your native formatter and normal compiler, linter, and tests. Shear does not invoke them internally.

## Safety

Shear is built to be safe to run without an unsafe-fix gate:

- Malformed input is rejected before any write.
- Every edit triggers a full reparse; a rule that fails to converge fails the run without writing.
- The exact multiset of comment texts is compared on every reparse. A dropped, duplicated, or edited comment aborts with no write.
- Rules skip uncertain cases instead of reconstructing arbitrary code.
- Writes are stale-checked, preserve permissions, and use a temporary-file replacement.

A batch is prepared before any write, so a malformed file prevents the whole batch from being written. Writes are not a multi-file transaction: a filesystem failure can leave an already-written portion of a batch. Use a version-controlled working tree. Inputs are limited to 2 MiB per file and 1,024 rewrites per file.

Parsing is not type checking and is not a proof of equivalence. See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the engine guarantees and [docs/RULES.md](docs/RULES.md) for what each rule does and does not preserve.

## Limitations

- Python and JavaScript only. TypeScript, JSX, helper extraction, and general refactoring are not supported.
- Source-location introspection, debugger line numbers, traceback locations, and bytecode identity are not preserved: moving source inherently changes those observations. Do not apply Shear to code whose contract depends on source layout.
- The shared-branch-tail rule is Python-only.
- This is an early prototype, not a general refactoring engine.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

CI runs formatting, tests, Clippy, and a release build on Linux with pinned toolchain actions.

## Documentation

- [Architecture and engine guarantees](docs/ARCHITECTURE.md)
- [Roadmap and verified differentiation](docs/ROADMAP.md)
- [Rule safety contracts](docs/RULES.md)
- [Dependency strategy](docs/DEPENDENCIES.md)
- [Shared-continuation example](docs/NORMALIZATION_EXAMPLE.md)
- [References and provenance](docs/SOURCES.md)

## License

There is no project license yet. The source is public, but public visibility alone is not a grant to reuse or redistribute. Dependencies retain their own licenses.
