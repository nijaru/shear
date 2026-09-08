//! Deterministic structural rewrites. Syntax coverage is not a semantic safety claim.
mod javascript;
mod python;

use anyhow::{Result, bail, ensure};
use std::ops::Range;
use tree_sitter::Parser;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Language {
    Python,
    JavaScript,
}

impl Language {
    pub fn from_path(path: &std::path::Path) -> Option<Self> {
        match path.extension()?.to_str()? {
            "py" => Some(Self::Python),
            "js" | "mjs" | "cjs" => Some(Self::JavaScript),
            _ => None,
        }
    }

    fn grammar(self) -> tree_sitter::Language {
        match self {
            Self::Python => tree_sitter_python::LANGUAGE.into(),
            Self::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
        }
    }
}

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
pub fn simplify(source: &str, language: Language) -> Result<Outcome> {
    ensure!(
        source.len() <= 2 * 1024 * 1024,
        "source exceeds 2 MiB limit"
    );
    let mut parser = Parser::new();
    parser.set_language(&language.grammar())?;
    let mut current = source.to_owned();
    let mut rewrites = Vec::new();
    let mut original_comments = None;
    for _ in 0..=1024 {
        let tree = parser
            .parse(&current, None)
            .ok_or_else(|| anyhow::anyhow!("parser cancelled"))?;
        ensure!(
            !tree.root_node().has_error(),
            "malformed {language:?} source; no changes written"
        );
        let comments = comment_texts(tree.root_node(), &current);
        if let Some(expected) = &original_comments {
            ensure!(
                &comments == expected,
                "rewrite changed comment text; no changes written"
            );
        } else {
            original_comments = Some(comments);
        }
        let edit = match language {
            Language::Python => python::next_edit(tree.root_node(), &current),
            Language::JavaScript => javascript::next_edit(tree.root_node(), &current),
        };
        let Some(edit) = edit else {
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

// Preserve the multiset: guards can reorder suites, but never remove, duplicate,
// or edit comment text. Rule-specific fixtures additionally enforce ownership.
fn comment_texts(root: tree_sitter::Node<'_>, source: &str) -> Vec<String> {
    let mut comments = Vec::new();
    let mut pending = vec![root];
    while let Some(node) = pending.pop() {
        if node.kind() == "comment" {
            comments.push(source[node.byte_range()].to_owned());
        }
        let mut cursor = node.walk();
        pending.extend(node.named_children(&mut cursor));
    }
    comments.sort_unstable();
    comments
}
