use crate::Edit;
use tree_sitter::Node;

pub(super) fn next_edit(root: Node<'_>, source: &str) -> Option<Edit> {
    // Iterative traversal avoids consuming the Rust stack on deeply nested input.
    let mut pending = vec![root];
    while let Some(node) = pending.pop() {
        if node.kind() == "if_statement"
            && safe_region(node, source)
            && let Some(edit) = redundant_else(node, source).or_else(|| merge_nested(node, source))
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
        if matches!(node.kind(), "comment" | "named_expression")
            || (node.kind() == "string" && node.start_position().row != node.end_position().row)
        {
            return false;
        }
        let mut cursor = node.walk();
        pending.extend(node.named_children(&mut cursor));
    }
    true
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

fn dedent_block(block: Node<'_>, source: &str, target: usize) -> Option<String> {
    let width = indent(block, source)?;
    let remove = width.checked_sub(target)?;
    if remove == 0 {
        return None;
    }
    let start = line_start(source, block.start_byte());
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
    let last = consequence.named_child(
        consequence
            .named_child_count()
            .checked_sub(1)?
            .try_into()
            .ok()?,
    )?;
    // Only explicit unconditional exits, never calls that merely look like exit/panic.
    if !matches!(
        last.kind(),
        "return_statement" | "raise_statement" | "break_statement" | "continue_statement"
    ) {
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
    let replacement = dedent_block(body, source, indent(node, source)?)?;
    Some(Edit {
        rule: "redundant-else",
        line: alternative.start_position().row + 1,
        range: line_start(source, alternative.start_byte())..alternative.end_byte(),
        replacement,
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
    let dedented = dedent_block(body, source, indent(inner, source)?)?;
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
