# Shear

A structural formatter: deterministically simplify code structure, not whitespace.

Shear runs locally without a model or API key. Humans and coding agents invoke it after editing code. The product is cross-language; the current Rust/tree-sitter prototype supports **Python only**, with three conservative rules: remove redundant `else` after an explicit exit, normalize exiting alternatives into guards, and merge nested conditions without alternatives.

## Build and run

Requires Rust/Cargo. Development has been checked with Rust 1.98.1 on macOS; behavioral tests also require `python3`.

```sh
cargo build --release
./target/release/shear --diff --explain path/to/code.py
./target/release/shear path/to/code.py
./target/release/shear --check path/to/project
```

Default invocation writes changed files. `--diff` previews without writing. `--check` writes nothing and exits 1 if simplifications are available, 0 otherwise; errors exit 2. With no paths, Shear scans the current directory. Directory discovery respects ignore files and skips hidden files and symlinks; explicitly named files bypass ignore rules.

For example:

```python
if authorized:
    if enabled:
        run()
```

becomes:

```python
if (authorized) and (enabled):
    run()
```

Run your native formatter and normal compiler/linter/tests afterward. Shear does not invoke them internally.

## Current limits

This is an early prototype, not a general refactoring engine. It skips uncertain syntax such as comments in rewrite regions, multiline strings, tabs, and named expressions. Rules reparse after each edit and converge to an unchanged second run. Inputs are limited to 2 MiB per file and 1,024 rewrites per file; pending original/result bytes are capped at 64 MiB per invocation. Parse checks are not type checking or proof of equivalence.

Writes preserve permissions and check for stale bytes before replacement. Stop concurrent writers; the comparison and replacement are not a filesystem transaction. A filesystem failure can leave an already-written portion of a multi-file batch. Use a version-controlled working tree.

No real-project showcase has qualified yet. The goal is a coherent cleanup pass for humans and agents, not exclusive rules; overlap with existing autofixers is expected. No universal readability or measured agent-time-saving claim is made.

Source is public but has no project license yet. Dependencies retain their own licenses.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

- [Architecture and roadmap](docs/HANDOFF.md)
- [Rule safety contracts](docs/RULES.md)
- [Dependency and Ruff integration strategy](docs/DEPENDENCIES.md)
- [Real-example selection](docs/EXAMPLE_SELECTION.md)
- [Video workflow](docs/DEMO_VIDEO.md)
- [Build evidence and limitations](docs/BUILD_LOG.md)
- [Sources and provenance](docs/SOURCES.md)
