use crate::Edit;
use tree_sitter::Node;

pub(super) fn next_edit(root: Node<'_>, source: &str) -> Option<Edit> {
    let mut pending = vec![root];
    while let Some(node) = pending.pop() {
        if node.kind() == "if_statement"
            && let Some(edit) = redundant_else(node, source)
                .or_else(|| merge_nested_if(node, source))
                .or_else(|| guard_clause(node, source))
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
    let alternative = node.child_by_field_name("alternative")?;
    if alternative.kind() != "else_clause" {
        return None;
    }
    let consequence = node.child_by_field_name("consequence")?;
    if !always_exits(consequence) {
        return None;
    }
    if consequence.kind() != "statement_block" && !text(consequence, source).ends_with(';') {
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
    let prefix = indentation(node, source)?;
    if !line_tail_is_empty(node, source) {
        return None;
    }
    let lifted = lift_block(body, prefix, source)?;
    Some(Edit {
        rule: "redundant-else",
        line: alternative.start_position().row + 1,
        range: consequence.end_byte()..node.end_byte(),
        replacement: format!("\n{lifted}"),
    })
}

// Only blocks and exhaustive conditionals propagate exits. In particular, a
// loop, try, switch, or label may consume or replace an apparent nested exit.
fn always_exits(node: Node<'_>) -> bool {
    let mut pending = vec![node];
    while let Some(node) = pending.pop() {
        match node.kind() {
            "return_statement" | "throw_statement" | "break_statement" | "continue_statement" => {}
            "statement_block" => {
                let Some(last) = last_statement(node) else {
                    return false;
                };
                pending.push(last);
            }
            "if_statement" => {
                let Some(consequence) = node.child_by_field_name("consequence") else {
                    return false;
                };
                let Some(alternative) = node.child_by_field_name("alternative").and_then(else_body)
                else {
                    return false;
                };
                pending.extend([consequence, alternative]);
            }
            _ => return false,
        }
    }
    true
}

fn else_body(alternative: Node<'_>) -> Option<Node<'_>> {
    if alternative.kind() != "else_clause" {
        return None;
    }
    let mut cursor = alternative.walk();
    alternative
        .named_children(&mut cursor)
        .find(|n| n.kind() != "comment")
}

fn indentation<'a>(node: Node<'_>, source: &'a str) -> Option<&'a str> {
    let prefix = &source[line_start(source, node.start_byte())..node.start_byte()];
    prefix.bytes().all(|b| b == b' ').then_some(prefix)
}

fn line_tail_is_empty(node: Node<'_>, source: &str) -> bool {
    source[node.end_byte()..]
        .split('\n')
        .next()
        .is_some_and(|s| s.trim().is_empty())
}

fn lift_block(body: Node<'_>, prefix: &str, source: &str) -> Option<String> {
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
    Some(lifted)
}

fn single_line_condition<'a>(node: Node<'_>, source: &'a str) -> Option<&'a str> {
    let condition = node.child_by_field_name("condition")?;
    if source[node.start_byte()..condition.start_byte()].trim() != "if"
        || node.start_position().row != condition.end_position().row
    {
        return None;
    }
    let value = text(condition, source);
    // Comments in a rewritten header have ambiguous ownership; line comments
    // could also swallow the newly appended operator or closing delimiter.
    let mut pending = vec![condition];
    while let Some(child) = pending.pop() {
        if child.kind() == "comment" {
            return None;
        }
        let mut cursor = child.walk();
        pending.extend(child.named_children(&mut cursor));
    }
    (!value.contains(['\n', '\r', '\t', '\x0c'])).then_some(value)
}

fn merge_nested_if(node: Node<'_>, source: &str) -> Option<Edit> {
    if node.child_by_field_name("alternative").is_some() {
        return None;
    }
    let body = node.child_by_field_name("consequence")?;
    if body.kind() != "statement_block"
        || body.start_position().row == body.end_position().row
        || !safe_to_unwrap(body, source)
    {
        return None;
    }
    let mut cursor = body.walk();
    let children: Vec<_> = body.named_children(&mut cursor).collect();
    let [inner] = children.as_slice() else {
        return None;
    };
    if inner.kind() != "if_statement"
        || inner.child_by_field_name("alternative").is_some()
        || inner.child_by_field_name("consequence")?.kind() != "statement_block"
    {
        return None;
    }
    let prefix = indentation(node, source)?;
    let outer_condition = single_line_condition(node, source)?;
    let inner_condition = single_line_condition(*inner, source)?;
    if !source[node.child_by_field_name("condition")?.end_byte()..body.start_byte()]
        .trim()
        .is_empty()
        || !line_tail_is_empty(node, source)
    {
        return None;
    }
    let merged = format!("({outer_condition} && {inner_condition})");
    let mut lifted = lift_block(body, prefix, source)?;
    // The byte offset below is relative to the first physical header line.
    // Skip blank padding instead of assuming it disappeared during dedenting.
    if !lifted.starts_with(&format!("{prefix}if")) {
        return None;
    }
    let condition = inner.child_by_field_name("condition")?;
    let start = prefix.len() + condition.start_byte() - inner.start_byte();
    lifted.replace_range(start..start + inner_condition.len(), &merged);
    if lifted.lines().next()?.len() > 88 {
        return None;
    }
    Some(Edit {
        rule: "merge-nested-if",
        line: node.start_position().row + 1,
        range: node.start_byte()..node.end_byte(),
        replacement: lifted[prefix.len()..].to_owned(),
    })
}

fn guard_clause(node: Node<'_>, source: &str) -> Option<Edit> {
    if !matches!(node.parent()?.kind(), "statement_block" | "program") {
        return None;
    }
    let normal = node.child_by_field_name("consequence")?;
    let exit = else_body(node.child_by_field_name("alternative")?)?;
    if normal.kind() != "statement_block"
        || exit.kind() != "statement_block"
        || always_exits(normal)
        || !always_exits(exit)
        || !safe_to_unwrap(node, source)
        || !text(last_statement(normal)?, source).ends_with(';')
        || source[normal.end_byte()..exit.start_byte()].trim() != "else"
        || !line_tail_is_empty(node, source)
    {
        return None;
    }
    // Reordering var declarations can change global property instantiation
    // order. Reject them even in nested scopes rather than infer a scope model.
    let mut pending = vec![node];
    while let Some(child) = pending.pop() {
        if child.kind() == "variable_declaration" {
            return None;
        }
        let mut cursor = child.walk();
        pending.extend(child.named_children(&mut cursor));
    }
    let prefix = indentation(node, source)?;
    let condition = single_line_condition(node, source)?;
    if !source[node.child_by_field_name("condition")?.end_byte()..normal.start_byte()]
        .trim()
        .is_empty()
    {
        return None;
    }
    let header = format!("if (!{condition}) {{");
    if prefix.len() + header.len() > 88 {
        return None;
    }
    let lifted = lift_block(normal, prefix, source)?;
    // Validate the exiting block's multiline layout, but retain its braces and
    // bytes: abrupt completion and disposal stay in the original nesting.
    lift_block(exit, prefix, source)?;
    Some(Edit {
        rule: "guard-clause",
        line: node.start_position().row + 1,
        range: node.start_byte()..node.end_byte(),
        replacement: format!("if (!{condition}) {}\n{lifted}", text(exit, source)),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn exhaustive_exit_analysis_uses_a_bounded_call_stack() {
        let source = format!(
            "function f() {{{}return 1;{}}}",
            "if (true) {".repeat(12_000),
            "} else { return 0; }".repeat(12_000)
        );
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&crate::Language::JavaScript.grammar())
            .unwrap();
        let tree = parser.parse(&source, None).unwrap();
        assert!(!tree.root_node().has_error());
        let body = tree
            .root_node()
            .named_child(0)
            .unwrap()
            .child_by_field_name("body")
            .unwrap();
        assert!(super::always_exits(body));
    }
}
