use crate::Edit;
use tree_sitter::Node;

pub(super) fn next_edit(root: Node<'_>, source: &str) -> Option<Edit> {
    // Iterative traversal avoids consuming the Rust stack on deeply nested input.
    let mut pending = vec![root];
    while let Some(node) = pending.pop() {
        if node.kind() == "if_statement"
            && safe_region(node, source)
            && let Some(edit) = redundant_else(node, source)
                .or_else(|| guard_clause(node, source))
                .or_else(|| merge_nested(node, source))
        {
            return Some(edit);
        }
        let mut cursor = node.walk();
        let children: Vec<_> = node.named_children(&mut cursor).collect();
        pending.extend(children.into_iter().rev());
    }
    None
}

fn safe_region(root: Node<'_>, source: &str) -> bool {
    if source[root.byte_range()].contains(['\t', '\r']) {
        return false;
    }
    let mut pending = vec![root];
    while let Some(node) = pending.pop() {
        if node.kind() == "named_expression"
            || (node.kind() == "comment" && is_directive(text(node, source)))
            || (node.kind() == "string" && node.start_position().row != node.end_position().row)
        {
            return false;
        }
        let mut cursor = node.walk();
        pending.extend(node.named_children(&mut cursor));
    }
    true
}

fn is_directive(comment: &str) -> bool {
    let content = comment
        .trim_start_matches('#')
        .trim_start()
        .to_ascii_lowercase();
    [
        "type:", "fmt:", "ruff:", "noqa", "nosec", "pylint:", "pyright:", "mypy:", "isort:",
        "pragma:", "doctest:",
    ]
    .iter()
    .any(|prefix| content.starts_with(prefix))
}

fn text<'a>(node: Node<'_>, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}
fn line_start(source: &str, offset: usize) -> usize {
    source[..offset].rfind('\n').map_or(0, |n| n + 1)
}
fn indent(node: Node<'_>, source: &str) -> Option<usize> {
    let prefix = &source[line_start(source, node.start_byte())..node.start_byte()];
    prefix.bytes().all(|b| b == b' ').then_some(prefix.len())
}

// Tree-sitter attaches leading suite comments to the header, not the block.
// Slice from the header's newline so those comments travel with their suite.
fn suite_start(header: Node<'_>, block: Node<'_>, source: &str) -> Option<usize> {
    if block.start_position().row <= header.start_position().row {
        return None;
    }
    let mut cursor = header.walk();
    if header.named_children(&mut cursor).any(|child| {
        child.kind() == "comment" && child.start_position().row == header.start_position().row
    }) {
        return None;
    }
    let start = header.start_byte() + source[header.start_byte()..].find('\n')? + 1;
    let width = indent(block, source)?;
    // Under-indented comments have ambiguous suite ownership: leave them alone.
    for line in source[start..block.end_byte()].lines() {
        if line.trim_start().starts_with('#')
            && line.len() - line.trim_start_matches(' ').len() < width
        {
            return None;
        }
    }
    Some(start)
}

fn dedent_suite(header: Node<'_>, block: Node<'_>, source: &str, target: usize) -> Option<String> {
    let width = indent(block, source)?;
    let remove = width.checked_sub(target)?;
    if remove == 0 {
        return None;
    }
    let start = suite_start(header, block, source)?;
    let mut output = String::new();
    for line in source[start..block.end_byte()].split_inclusive('\n') {
        if line.trim().is_empty() {
            output.push_str(line);
            continue;
        }
        if !line.as_bytes().get(..remove)?.iter().all(|b| *b == b' ') {
            return None;
        }
        output.push_str(&line[remove..]);
    }
    Some(output)
}

fn redundant_else(node: Node<'_>, source: &str) -> Option<Edit> {
    let alternative = node.child_by_field_name("alternative")?;
    if alternative.kind() != "else_clause" {
        return None;
    }
    let consequence = node.child_by_field_name("consequence")?;
    if !directly_terminates(consequence) {
        return None;
    }
    let body = alternative.child_by_field_name("body")?;
    if body.start_position().row == alternative.start_position().row {
        return None;
    }
    // Inline comments outside the syntax node range would change ownership when lifted.
    let tail = source[node.end_byte()..].split('\n').next()?;
    if !tail.trim().is_empty() {
        return None;
    }
    let replacement = dedent_suite(alternative, body, source, indent(node, source)?)?;
    Some(Edit {
        rule: "redundant-else",
        line: alternative.start_position().row + 1,
        range: line_start(source, alternative.start_byte())..alternative.end_byte(),
        replacement,
    })
}

// Only explicit unconditional exits, never calls that merely look like exit/panic.
fn directly_terminates(block: Node<'_>) -> bool {
    let mut cursor = block.walk();
    block
        .named_children(&mut cursor)
        .filter(|child| child.kind() != "comment")
        .last()
        .is_some_and(|last| {
            matches!(
                last.kind(),
                "return_statement" | "raise_statement" | "break_statement" | "continue_statement"
            )
        })
}

fn guard_clause(node: Node<'_>, source: &str) -> Option<Edit> {
    let alternative = node.child_by_field_name("alternative")?;
    if alternative.kind() != "else_clause" {
        return None;
    }
    let consequence = node.child_by_field_name("consequence")?;
    let exit_body = alternative.child_by_field_name("body")?;
    if directly_terminates(consequence) || !directly_terminates(exit_body) {
        return None;
    }
    let condition = node.child_by_field_name("condition")?;
    if !source[consequence.end_byte()..alternative.start_byte()]
        .trim()
        .is_empty()
    {
        return None;
    }
    if condition.start_position().row != condition.end_position().row
        || consequence.start_position().row == node.start_position().row
        || exit_body.start_position().row == alternative.start_position().row
        || !source[node.end_byte()..]
            .split('\n')
            .next()?
            .trim()
            .is_empty()
    {
        return None;
    }
    let lifted = dedent_suite(node, consequence, source, indent(node, source)?)?;
    let exit_suite = &source[suite_start(alternative, exit_body, source)?..exit_body.end_byte()];
    // `not` performs one truth test; do not invert comparison operators (NaN,
    // overloaded comparisons, and rich truthiness make that a different operation).
    Some(Edit {
        rule: "guard-clause",
        line: node.start_position().row + 1,
        range: node.byte_range(),
        replacement: format!(
            "if not ({}):\n{}\n{}",
            text(condition, source),
            exit_suite,
            lifted
        ),
    })
}

fn merge_nested(node: Node<'_>, source: &str) -> Option<Edit> {
    if node.child_by_field_name("alternative").is_some() {
        return None;
    }
    let outer_body = node.child_by_field_name("consequence")?;
    if outer_body.named_child_count() != 1 {
        return None;
    }
    let inner = outer_body.named_child(0)?;
    if inner.kind() != "if_statement" || inner.child_by_field_name("alternative").is_some() {
        return None;
    }
    let outer_condition = node.child_by_field_name("condition")?;
    let inner_condition = inner.child_by_field_name("condition")?;
    // Keep multiline expression indentation and line-continuation semantics out of V1.
    if outer_condition.start_position().row != outer_condition.end_position().row
        || inner_condition.start_position().row != inner_condition.end_position().row
    {
        return None;
    }
    let body = inner.child_by_field_name("consequence")?;
    if body.start_position().row == inner.start_position().row {
        return None;
    }
    // A comment explaining the inner condition cannot be silently reassigned
    // to the combined condition or body.
    if !source[suite_start(node, outer_body, source)?..inner.start_byte()]
        .trim()
        .is_empty()
    {
        return None;
    }
    let dedented = dedent_suite(inner, body, source, indent(inner, source)?)?;
    // Python short-circuit `and` tests each operand in the same order as nested ifs.
    let replacement = format!(
        "if ({}) and ({}):\n{}",
        text(outer_condition, source),
        text(inner_condition, source),
        dedented
    );
    Some(Edit {
        rule: "merge-nested-if",
        line: node.start_position().row + 1,
        range: node.byte_range(),
        replacement,
    })
}
