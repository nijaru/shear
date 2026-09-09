# Shear

A structural formatter: deterministically simplify code structure, not whitespace.

Shear runs locally without a model or API key. Humans and coding agents invoke it after editing code. The Rust/tree-sitter prototype supports **Python and JavaScript in one invocation**, with separate semantic safety rules:

| Language | Current transformations |
|---|---|
| Python (`.py`) | Shared branch tails, redundant `else`/`elif`, exiting alternatives into guards, nested conditions |
| JavaScript (`.js`, `.mjs`, `.cjs`) | Redundant `else`, exiting alternatives into guards, nested conditions, with scope and statement-boundary checks |

## Build and run

Requires Rust/Cargo. Development has been checked with Rust 1.98.1 on macOS; behavioral tests also require `python3` and Node 26 (including explicit resource-management regressions). Neither interpreter is required to run the built formatter.

```sh
cargo build --release
./target/release/shear --diff --explain path/to/code.py path/to/code.js
./target/release/shear path/to/code.py
./target/release/shear --check path/to/project
```

Default invocation writes changed files. `--diff` previews without writing. `--check` writes nothing and exits 1 if simplifications are available, 0 otherwise; errors exit 2. With no paths, Shear scans the current directory. Directory discovery respects ignore files and skips hidden files, symlinks, and nested `node_modules`; explicitly named files bypass ignore rules.

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

Run your native formatter and normal compiler/linter/tests afterward. Shear does not invoke them internally.

## Current limits

The [review](docs/REVIEW.md) found declaration-order and path-policy defects, now protected by regression tests: guards skip declaration-containing regions and swaps whose local-slot order cannot be established, and explicit paths reject symlinked components before canonicalization. The guard restriction also preserves CPython local-variable/finalizer ordering; form-feed regions are skipped to avoid indentation corruption. This includes aliases such as macOS `/tmp`; supply the real path (for example `/private/tmp`) or run from the physical directory. Concurrent path replacement remains unsupported. Merged conditions avoid redundant grouping and are skipped when the resulting header exceeds an 88-byte budget. JavaScript skips lexical/function/class/`using` declarations in lifted blocks, ambiguous layouts, and implicit-semicolon boundaries. Do not treat parsing as a substitute for native compilation and project tests. See the [language-specific contracts](docs/RULES.md).

This is an early prototype, not a general refactoring engine. It preserves ordinary suite comments but skips uncertain cases rather than reconstructing arbitrary code. TypeScript, JSX, helper extraction, and general refactoring are not supported. Rules reparse after each edit and converge to an unchanged second run. Inputs are limited to 2 MiB per file and 1,024 rewrites per file; pending original/result bytes are capped at 64 MiB per invocation. Parse checks are not type checking or proof of equivalence.

Writes preserve permissions and check for stale bytes before replacement. Stop concurrent writers; the comparison and replacement are not a filesystem transaction. A filesystem failure can leave an already-written portion of a multi-file batch. Use a version-controlled working tree.

A [checked normalization example](docs/NORMALIZATION_EXAMPLE.md) removes duplicated branch cleanup from CPython’s `locale._localize` after configured Ruff autofixes, with the unchanged locale suite passing at all three stages.

Source is public but has no project license yet. Dependencies retain their own licenses.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

- [Architecture and roadmap](docs/HANDOFF.md)
- [Rule safety contracts](docs/RULES.md)
- [Dependency strategy](docs/DEPENDENCIES.md)
- [Normalization example and reproduction](docs/NORMALIZATION_EXAMPLE.md)
- [Build evidence and limitations](docs/BUILD_LOG.md)
- [Review findings](docs/REVIEW.md)
- [Sources and provenance](docs/SOURCES.md)
