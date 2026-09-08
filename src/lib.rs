//! Deterministic structural rewrites. Syntax coverage is not a semantic safety claim.
mod python;

use anyhow::{Result, bail, ensure};
use std::ops::Range;
use tree_sitter::Parser;

#[derive(Debug)]
pub struct Rewrite {
    pub rule: &'static str,
    pub line: usize,
}

struct Edit {
    rule: &'static str,
    line: usize,
    range: Range<usize>,
    replacement: String,
}

#[derive(Debug)]
pub struct Outcome {
    pub source: String,
    pub rewrites: Vec<Rewrite>,
}

/// Reparse after each surgical edit. Each rule removes one `if` or `else`,
/// so the finite pass budget is a failure boundary, not a normal stopping point.
pub fn simplify_python(source: &str) -> Result<Outcome> {
    ensure!(
        source.len() <= 2 * 1024 * 1024,
        "source exceeds 2 MiB limit"
    );
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_python::LANGUAGE.into())?;
    let mut current = source.to_owned();
    let mut rewrites = Vec::new();
    for _ in 0..=1024 {
        let tree = parser
            .parse(&current, None)
            .ok_or_else(|| anyhow::anyhow!("parser cancelled"))?;
        ensure!(
            !tree.root_node().has_error(),
            "malformed Python source; no changes written"
        );
        let Some(edit) = python::next_edit(tree.root_node(), &current) else {
            return Ok(Outcome {
                source: current,
                rewrites,
            });
        };
        ensure!(
            rewrites.len() < 1024,
            "rewrite limit reached; no changes written"
        );
        ensure!(
            current.is_char_boundary(edit.range.start) && current.is_char_boundary(edit.range.end),
            "invalid edit boundary"
        );
        let old = current.clone();
        current.replace_range(edit.range.clone(), &edit.replacement);
        ensure!(current != old, "rule emitted a no-op");
        rewrites.push(Rewrite {
            rule: edit.rule,
            line: edit.line,
        });
    }
    bail!("rewrite limit reached")
}
