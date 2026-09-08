use crate::Edit;
use tree_sitter::Node;

pub(super) fn next_edit(root: Node<'_>, source: &str) -> Option<Edit> {
    let mut pending = vec![root];
    while let Some(node) = pending.pop() {
        if node.kind() == "if_statement"
            && let Some(edit) = redundant_else(node, source)
        {
            return Some(edit);
        }
        let mut cursor = node.walk();
        let children: Vec<_> = node.named_children(&mut cursor).collect();
        pending.extend(children.into_iter().rev());
    }
    None
}

fn text<'a>(node: Node<'_>, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}

fn line_start(source: &str, offset: usize) -> usize {
    source[..offset].rfind('\n').map_or(0, |n| n + 1)
}

fn last_statement(block: Node<'_>) -> Option<Node<'_>> {
    let mut cursor = block.walk();
    block
        .named_children(&mut cursor)
        .filter(|n| n.kind() != "comment")
        .last()
}

fn safe_to_unwrap(block: Node<'_>, source: &str) -> bool {
    if text(block, source).contains(['\t', '\r', '\x0c']) {
        return false;
    }
    let mut pending = vec![block];
    while let Some(node) = pending.pop() {
        // Removing a block must not widen lexical bindings or alter Annex B
        // function declaration behavior. Nested declarations are skipped too.
        if node.kind().starts_with("jsx_")
            || matches!(
                node.kind(),
                "lexical_declaration"
                    | "using_declaration"
                    | "class_declaration"
                    | "function_declaration"
                    | "generator_function_declaration"
            )
            || (matches!(node.kind(), "string" | "template_string" | "comment")
                && node.start_position().row != node.end_position().row)
        {
            return false;
        }
        if node.kind() == "comment" {
            let comment = text(node, source).to_ascii_lowercase();
            if [
                "eslint",
                "prettier",
                "istanbul",
                "c8",
                "sourceurl",
                "sourcemappingurl",
                "@ts-",
                "jshint",
            ]
            .iter()
            .any(|directive| comment.contains(directive))
            {
                return false;
            }
        }
        let mut cursor = node.walk();
        pending.extend(node.named_children(&mut cursor));
    }
    true
}

fn redundant_else(node: Node<'_>, source: &str) -> Option<Edit> {
    // Lifting from an unbraced parent would move statements outside its control.
    if !matches!(node.parent()?.kind(), "statement_block" | "program") {
        return None;
    }
    let consequence = node.child_by_field_name("consequence")?;
    let exit = if consequence.kind() == "statement_block" {
        last_statement(consequence)?
    } else {
        consequence
    };
    if !matches!(
        exit.kind(),
        "return_statement" | "throw_statement" | "break_statement" | "continue_statement"
    ) {
        return None;
    }
    if consequence.kind() != "statement_block" && !text(consequence, source).ends_with(';') {
        return None;
    }
    let alternative = node.child_by_field_name("alternative")?;
    if alternative.kind() != "else_clause" {
        return None;
    }
    let mut cursor = alternative.walk();
    let body = alternative
        .named_children(&mut cursor)
        .find(|n| n.kind() != "comment")?;
    if body.kind() != "statement_block" || !safe_to_unwrap(body, source) {
        return None;
    }
    // Explicit terminators prevent newly adjacent expressions from joining via
    // ASI, e.g. work() followed by a parenthesized call after the removed brace.
    if !text(last_statement(body)?, source).ends_with(';') {
        return None;
    }
    if source[consequence.end_byte()..body.start_byte()].trim() != "else" {
        return None;
    }
    let prefix = &source[line_start(source, node.start_byte())..node.start_byte()];
    if !prefix.bytes().all(|b| b == b' ') {
        return None;
    }
    let interior = &source[body.start_byte() + 1..body.end_byte() - 1];
    let (opening_tail, lines) = interior.split_once('\n')?;
    if !opening_tail.trim().is_empty() {
        return None;
    }
    let closing_start = line_start(source, body.end_byte() - 1);
    if !source[closing_start..body.end_byte() - 1]
        .bytes()
        .all(|b| b == b' ')
        || !source[node.end_byte()..]
            .split('\n')
            .next()?
            .trim()
            .is_empty()
    {
        return None;
    }
    let width = lines
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start_matches(' ').len())
        .min()?;
    let remove = width.checked_sub(prefix.len())?;
    if remove == 0 {
        return None;
    }
    let start = body.start_byte() + 1 + opening_tail.len() + 1;
    let mut lifted = String::new();
    for line in source[start..closing_start].split_inclusive('\n') {
        if line.trim().is_empty() {
            lifted.push_str(line);
        } else {
            lifted.push_str(line.get(remove..)?);
        }
    }
    lifted.pop().filter(|c| *c == '\n')?;
    Some(Edit {
        rule: "redundant-else",
        line: alternative.start_position().row + 1,
        range: consequence.end_byte()..node.end_byte(),
        replacement: format!("\n{lifted}"),
    })
}
