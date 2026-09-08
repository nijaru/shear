use anyhow::{Context, Result, ensure};
use ignore::WalkBuilder;
use similar::TextDiff;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};
use usage::Cli;

/// Deterministic structural simplification. Currently supports Python.
#[derive(Cli)]
#[usage(bin = "shear", version = "0.1.0")]
struct Arguments {
    /// Check without writing; exit 1 when simplifications are available.
    #[usage(long)]
    check: bool,
    /// Show unified diffs without writing.
    #[usage(long)]
    diff: bool,
    /// Report the rules applied and their original pass-local line numbers.
    #[usage(long)]
    explain: bool,
    /// Files or directories to simplify. Defaults to the current directory.
    paths: Vec<PathBuf>,
}

fn main() -> ExitCode {
    match run(Arguments::parse()) {
        Ok(true) => ExitCode::from(1),
        Ok(false) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("shear: {error:#}");
            ExitCode::from(2)
        }
    }
}

fn reject_symlink_components(path: &Path) -> Result<()> {
    // Inspect lexical ancestors before canonicalization: normalizing `link/..`
    // first would conceal the very traversal this policy rejects.
    let ancestors: Vec<_> = path
        .ancestors()
        .filter(|p| !p.as_os_str().is_empty())
        .collect();
    for component in ancestors.into_iter().rev() {
        ensure!(
            !fs::symlink_metadata(component)
                .with_context(|| format!("inspect {}", component.display()))?
                .file_type()
                .is_symlink(),
            "symlink path component unsupported: {}",
            component.display()
        );
    }
    Ok(())
}

fn discover(paths: &[PathBuf]) -> Result<BTreeSet<PathBuf>> {
    let mut files = BTreeSet::new();
    for path in paths {
        reject_symlink_components(path)?;
        let metadata =
            fs::symlink_metadata(path).with_context(|| format!("inspect {}", path.display()))?;
        if metadata.is_file() {
            ensure!(
                path.extension().is_some_and(|ext| ext == "py"),
                "unsupported source language: {}",
                path.display()
            );
            files.insert(fs::canonicalize(path)?);
            continue;
        }
        ensure!(
            metadata.is_dir(),
            "not a regular file or directory: {}",
            path.display()
        );
        for entry in WalkBuilder::new(path).follow_links(false).build() {
            let entry = entry?;
            if entry.file_type().is_some_and(|kind| kind.is_file())
                && entry.path().extension().is_some_and(|ext| ext == "py")
            {
                files.insert(fs::canonicalize(entry.path())?);
            }
        }
    }
    Ok(files)
}

fn run(mut args: Arguments) -> Result<bool> {
    if args.paths.is_empty() {
        args.paths.push(PathBuf::from("."));
    }
    // Prepare every file before writing any: malformed input cannot cause a partial batch.
    let mut changes = Vec::new();
    let mut pending_bytes = 0;
    for path in discover(&args.paths)? {
        ensure!(
            fs::metadata(&path)?.len() <= 2 * 1024 * 1024,
            "source exceeds 2 MiB: {}",
            path.display()
        );
        let original = fs::read_to_string(&path)?;
        let outcome =
            shear::simplify_python(&original).with_context(|| path.display().to_string())?;
        if outcome.source != original {
            pending_bytes += original.len() + outcome.source.len();
            ensure!(
                pending_bytes <= 64 * 1024 * 1024,
                "pending changes exceed 64 MiB; use a smaller file selection"
            );
            changes.push((path, original, outcome));
        }
    }
    for (path, original, outcome) in &changes {
        if args.diff {
            print!(
                "{}",
                TextDiff::from_lines(original, &outcome.source)
                    .unified_diff()
                    .header(
                        &format!("a/{}", path.display()),
                        &format!("b/{}", path.display())
                    )
            );
        }
        if args.explain {
            for rewrite in &outcome.rewrites {
                eprintln!("{}:{}: {}", path.display(), rewrite.line, rewrite.rule);
            }
        }
        if !args.check && !args.diff {
            write_checked(path, original, &outcome.source)?;
            eprintln!(
                "modified {} ({} simplifications)",
                path.display(),
                outcome.rewrites.len()
            );
        } else if !args.diff {
            eprintln!("would simplify {}", path.display());
        }
    }
    Ok(args.check && !changes.is_empty())
}

fn write_checked(path: &Path, original: &str, replacement: &str) -> Result<()> {
    use std::io::Write;
    let metadata = fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_file() && !metadata.file_type().is_symlink(),
        "target is no longer a regular file"
    );
    ensure!(
        fs::read_to_string(path)? == original,
        "stale source: {}",
        path.display()
    );
    let mut temporary =
        tempfile::NamedTempFile::new_in(path.parent().context("target has no parent")?)?;
    temporary
        .as_file()
        .set_permissions(metadata.permissions())?;
    temporary.write_all(replacement.as_bytes())?;
    temporary.as_file().sync_all()?;
    ensure!(
        fs::read_to_string(path)? == original,
        "source changed before replacement: {}",
        path.display()
    );
    temporary.persist(path).context("replace source file")?;
    Ok(())
}
