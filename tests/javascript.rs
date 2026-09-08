use shear::{Language, simplify};
use std::{
    io::Write,
    process::{Command, Stdio},
};

fn execute(source: &str, mode: &str) -> Vec<u8> {
    let mut child = Command::new("node")
        .args(["--input-type", mode, "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Node is required for JavaScript semantic regressions");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(source.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}\n{source}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn checked(source: &str, mode: &str) -> String {
    let before = execute(source, mode);
    let result = simplify(source, Language::JavaScript).unwrap();
    assert!(
        !result.rewrites.is_empty(),
        "fixture must exercise a rewrite"
    );
    assert_eq!(before, execute(&result.source, mode));
    let second = simplify(&result.source, Language::JavaScript).unwrap();
    assert!(second.rewrites.is_empty());
    assert_eq!(result.source, second.source);
    result.source
}

#[test]
fn deep_inline_conditionals_do_not_overflow() {
    let source = format!(
        "function f() {{{}return 1;{}}}\n",
        "if (true) {".repeat(256),
        "}".repeat(256)
    );
    let result = simplify(&source, Language::JavaScript).unwrap();
    assert_eq!(result.source, source);
    assert!(result.rewrites.is_empty());
}

#[test]
fn removes_braced_else_and_preserves_comments() {
    let source = "function f(flag) {\n  if (flag) {\n    return 1;\n  } else {\n    // Ordinary fallback.\n    return 2;\n  }\n}\nconsole.log(f(false), f(true));\n";
    let result = checked(source, "commonjs");
    assert_eq!(
        result,
        "function f(flag) {\n  if (flag) {\n    return 1;\n  }\n  // Ordinary fallback.\n  return 2;\n}\nconsole.log(f(false), f(true));\n"
    );
}

#[test]
fn unbraced_returns_preserve_var_hoisting_and_short_circuit_effects() {
    checked(
        r#"function f(flag) {
  if (flag)
    return typeof value;
  else {
    var value = 'x';
    if (value.charCodeAt(0) === 120 /* x */)
      value += '!';
    return value;
  }
}
console.log(f(false), f(true));
"#,
        "commonjs",
    );
}

#[test]
fn exits_preserve_loop_targets_throw_values_and_cleanup() {
    checked(
        r#"const events = [];
outer: for (let n = 0; n < 5; n++) {
  try {
    if (n === 1) {
      continue outer;
    } else {
      events.push(n);
    }
    if (n === 3) {
      break outer;
    } else {
      events.push('next');
    }
  } finally { events.push('cleanup'); }
}
function fail(flag) {
  if (flag) {
    throw 'error';
  } else {
    return 'normal';
  }
}
for (const flag of [false, true]) {
  try { events.push(fail(flag)); } catch (error) { events.push(error); }
}
console.log(JSON.stringify(events));
"#,
        "commonjs",
    );
}

#[test]
fn async_and_generator_suspension_preserve_finally_order() {
    checked(
        r#"const events = [];
async function f(flag) {
  try {
    if (flag) {
      return await Promise.resolve('early');
    } else {
      events.push(await Promise.resolve('late'));
      return 'normal';
    }
  } finally { events.push('async cleanup'); }
}
function* g(flag) {
  try {
    if (flag) {
      return 'early';
    } else {
      yield 'late';
      return 'normal';
    }
  } finally { events.push('generator cleanup'); }
}
for (const flag of [false, true]) {
  events.push(await f(flag));
  const iterator = g(flag);
  events.push(iterator.next());
  events.push(iterator.return('closed'));
}
console.log(JSON.stringify(events));
"#,
        "module",
    );
}

#[test]
fn skips_scope_widening_and_annex_b_declarations() {
    for declaration in [
        "let x = 2;",
        "const x = 2;",
        "class x {}",
        "function x() {}",
        "function* x() {}",
        "{ function x() {} }",
    ] {
        let source = format!(
            "function f(flag) {{\n  if (flag) {{\n    return 1;\n  }} else {{\n    {declaration}\n    return 2;\n  }}\n}}\nconsole.log(f(false), f(true));\n"
        );
        execute(&source, "commonjs");
        assert_eq!(
            simplify(&source, Language::JavaScript).unwrap().source,
            source
        );
    }
}

#[test]
fn skips_asi_hazards_on_both_removed_boundaries() {
    for source in [
        "function f(flag) {\n  if (flag) return 1;\n  else {\n    console.log('body')\n  }\n  (() => console.log('tail'))();\n}\nf(false);\n",
        "function f(flag) {\n  if (flag)\n    return 1\n  else {\n    (() => console.log('body'))();\n    return 2;\n  }\n}\nconsole.log(f(false), f(true));\n",
    ] {
        execute(source, "commonjs");
        assert_eq!(
            simplify(source, Language::JavaScript).unwrap().source,
            source
        );
    }
}

#[test]
fn skips_unbraced_parent_and_else_if_attachment() {
    for source in [
        "function f(a, b) {\n  if (a)\n    if (b) return 1;\n    else {\n      console.log('conditional');\n      return 2;\n    }\n}\nconsole.log(f(false, false));\n",
        "function f(a, b) {\n  if (a) return 1;\n  else if (b) return 2;\n  else {\n    return 3;\n  }\n}\nconsole.log(f(false, false));\n",
    ] {
        assert_eq!(
            simplify(source, Language::JavaScript).unwrap().source,
            source
        );
    }
}

#[test]
fn skips_multiline_literals_directives_and_ambiguous_headers() {
    for body in [
        "return `first\n      second`;",
        "/* first\n      second */\n    return 2;",
        "// eslint-disable-next-line\n    return 2;",
        "return <div>\n      text\n    </div>;",
    ] {
        let source = format!(
            "function f(flag) {{\n  if (flag) {{\n    return 1;\n  }} else {{\n    {body}\n  }}\n}}\n"
        );
        assert_eq!(
            simplify(&source, Language::JavaScript).unwrap().source,
            source
        );
    }
    let source = "function f(flag) {\n  if (flag) return 1;\n  else { // header ownership\n    return 2;\n  }\n}\n";
    assert_eq!(
        simplify(source, Language::JavaScript).unwrap().source,
        source
    );
}

#[test]
fn using_declarations_keep_disposal_order_and_binding_scope() {
    for (declaration, symbol) in [("using", "dispose"), ("await using", "asyncDispose")] {
        let source = format!(
            r#"const events = [];
async function f(flag) {{
  if (flag) {{
    return;
  }} else {{
    {declaration} resource = {{ [Symbol.{symbol}]() {{ events.push('dispose'); }} }};
    events.push('body');
  }}
  events.push(typeof resource);
  events.push('tail');
}}
await f(false);
console.log(events.join(','));
"#
        );
        assert_eq!(execute(&source, "module"), b"body,dispose,undefined,tail\n");
        assert_eq!(
            simplify(&source, Language::JavaScript).unwrap().source,
            source
        );
    }
}

#[test]
fn refuses_malformed_javascript() {
    assert!(simplify("function f( {", Language::JavaScript).is_err());
}

#[test]
fn exhaustive_nested_exits_lift_fallback() {
    let source = r#"function f(a, b, c) {
  if (a) {
    if (b) {
      return 'b';
    } else if (c) {
      throw 'c';
    } else {
      return 'a';
    }
  } else {
    console.log('fallback');
    return 'normal';
  }
}
for (const a of [false, true]) for (const b of [false, true]) for (const c of [false, true]) {
  try { console.log(f(a, b, c)); } catch (e) { console.log(e); }
}
"#;
    let result = checked(source, "commonjs");
    assert!(result.contains("  }\n  console.log('fallback');"));
}

#[test]
fn recursive_exit_analysis_does_not_cross_consuming_controls() {
    for body in [
        "while (flag) { break; }",
        "label: { break label; }",
        "try { return 1; } finally { if (flag) return 2; }",
        "switch (flag) { default: break; }",
        "if (flag) { return 1; }",
    ] {
        let source = format!(
            "function f(flag) {{\n  if (flag) {{\n    {body}\n  }} else {{\n    console.log('fallback');\n  }}\n}}\nf(false);\n"
        );
        execute(&source, "commonjs");
        assert_eq!(
            simplify(&source, Language::JavaScript).unwrap().source,
            source
        );
    }
}

#[test]
fn nested_conditions_preserve_grouping_short_circuit_and_comments() {
    let source = r#"const events = [];
function probe(x) { events.push(x); return x; }
function f(a, b, c) {
  if (probe(a) || probe(b)) {
    if (probe(c), c ? a : b) {
      if (probe('last')) {
        // Body ownership stays here.
        events.push('body');
      }
    }
  }
}
for (const a of [0, 1]) for (const b of [0, 1]) for (const c of [0, 1]) f(a, b, c);
console.log(JSON.stringify(events));
"#;
    let result = checked(source, "commonjs");
    assert!(
        result.contains(
            "if (((probe(a) || probe(b)) && (probe(c), c ? a : b)) && (probe('last'))) {"
        )
    );
    assert!(result.contains("    // Body ownership stays here.\n    events.push('body');"));
}

#[test]
fn merge_skips_comments_alternatives_declarations_and_long_headers() {
    for source in [
        "if (a) {\n  // Between conditions.\n  if (b) {\n    work();\n  }\n}\n",
        "if /* ownership */ (a) {\n  if (b) {\n    work();\n  }\n}\n",
        "if (a /* ownership */) {\n  if (b) {\n    work();\n  }\n}\n",
        "if (a) {\n  if (b) {\n    let x = 1;\n    work(x);\n  }\n}\n",
        "if (a) {\n  if (b) {\n    work();\n  } else {\n    other();\n  }\n}\n",
        "if (a) {\n  if (b) {\n    work();\n  }\n} else {\n  other();\n}\n",
        "if (veryLongOuterConditionNameThatExceedsReadabilityBudget()) {\n  if (anotherVeryLongConditionNameThatExceedsBudget()) {\n    work();\n  }\n}\n",
    ] {
        assert_eq!(
            simplify(source, Language::JavaScript).unwrap().source,
            source
        );
    }
}

#[test]
fn guard_normalization_preserves_throw_effects_and_comment_ownership() {
    let source = r#"const events = [];
function condition(x) { events.push('condition'); return x; }
function failure() { events.push('throw'); return 'failure'; }
function f(x) {
  if (condition(x)) {
    // Normal branch.
    events.push('body');
  } else {
    // Failure branch.
    throw failure();
  }
  events.push('tail');
}
for (const x of [false, true, NaN, {}, 0, '']) {
  try { f(x); } catch (e) { events.push(e); }
}
console.log(JSON.stringify(events));
"#;
    let result = checked(source, "commonjs");
    assert!(result.contains("if (!(condition(x))) {\n    // Failure branch.\n    throw failure();\n  }\n  // Normal branch.\n  events.push('body');"));
}

#[test]
fn guard_and_recursive_exits_keep_labeled_loop_destinations_and_finally() {
    let source = r#"const events = [];
outer: for (let i = 0; i < 4; i++) {
  for (let j = 0; j < 3; j++) {
    try {
      if (i === 0) {
        events.push('body');
      } else {
        if (i === 1) {
          continue outer;
        } else {
          break outer;
        }
      }
      events.push('tail');
    } finally { events.push('finally'); }
  }
}
console.log(JSON.stringify(events));
"#;
    let result = checked(source, "commonjs");
    assert!(result.contains("if (!(i === 0))"));
}

#[test]
fn guards_skip_declaration_reordering_disposal_asi_and_header_comments() {
    for statement in [
        "var first = 1;",
        "let first = 1;",
        "const first = 1;",
        "class First {}",
        "function first() {}",
        "function* first() {}",
        "using first = null;",
        "await using first = null;",
        "{ var first = 1; }",
    ] {
        for in_exit in [false, true] {
            let (normal, exit) = if in_exit {
                ("work();", statement)
            } else {
                (statement, "work();")
            };
            let source = format!(
                "async function f(flag) {{\n  if (flag) {{\n    {normal}\n    work();\n  }} else {{\n    {exit}\n    return;\n  }}\n}}\n"
            );
            execute(&source, "module");
            assert_eq!(
                simplify(&source, Language::JavaScript).unwrap().source,
                source
            );
        }
    }
    for source in [
        "function f(a) {\n  if (a) {\n    work()\n  } else {\n    return;\n  }\n  (() => tail())();\n}\n",
        "function f(a) {\n  if /* note */ (a) {\n    work();\n  } else {\n    return;\n  }\n}\n",
        "function f(a) {\n  if (a) {\n    work();\n  } else { // note\n    return;\n  }\n}\n",
        "function f(a) {\n  label: if (a) {\n    work();\n  } else {\n    break label;\n  }\n}\n",
    ] {
        assert_eq!(
            simplify(source, Language::JavaScript).unwrap().source,
            source
        );
    }
}

#[test]
fn guard_then_merge_converges() {
    let source = r#"function f(a, b, c) {
  if (a) {
    if (b) {
      if (c) {
        console.log('body');
      }
    }
    console.log('normal');
  } else {
    return 'early';
  }
  return 'end';
}
for (const a of [false, true]) for (const b of [false, true]) for (const c of [false, true]) console.log(f(a, b, c));
"#;
    checked(source, "commonjs");
    let outcome = simplify(source, Language::JavaScript).unwrap();
    assert_eq!(
        outcome.rewrites.iter().map(|r| r.rule).collect::<Vec<_>>(),
        ["guard-clause", "merge-nested-if"]
    );
}

#[test]
fn merge_skips_padded_or_split_headers_without_invalid_byte_edits() {
    for source in [
        "if (a) {\n\n  if (b) {\n    work();\n  }\n}\n",
        "if (a) {\n  if\n    (b) {\n    work();\n  }\n}\n",
        "if\n  (a) {\n  if (b) {\n    work();\n  }\n}\n",
    ] {
        assert_eq!(
            simplify(source, Language::JavaScript).unwrap().source,
            source
        );
    }
    let source = "const café = true;\nif (café) {\n  if ('☕') {\n    console.log('☕');\n  }\n}\n";
    checked(source, "module");
}

#[test]
fn guard_skip_preserves_global_var_instantiation_order() {
    let source = "if (true) {\n  var normal = 1;\n  console.log('normal');\n} else {\n  var failure = 2;\n  throw 'failure';\n}\n";
    let result = simplify(source, Language::JavaScript).unwrap();
    assert!(result.rewrites.is_empty());
    let run_global = |code: &str| {
        let script = format!(
            "{code}\nconsole.log(Object.keys(globalThis).filter(k => ['normal', 'failure'].includes(k)).join(','));"
        );
        execute(
            &format!("require('node:vm').runInNewContext({script:?}, {{console}});"),
            "commonjs",
        )
    };
    assert_eq!(run_global(source), b"normal\nnormal,failure\n");
    assert_eq!(run_global(source), run_global(&result.source));
}

#[test]
fn guard_preserves_async_and_generator_suspension() {
    let source = r#"const events = [];
async function f(flag) {
  try {
    if (await Promise.resolve(flag)) {
      events.push(await Promise.resolve('body'));
    } else {
      return await Promise.resolve('early');
    }
    return 'tail';
  } finally { events.push('cleanup'); }
}
function* g(flag) {
  if (flag) {
    yield 'body';
  } else {
    return 'early';
  }
  yield 'tail';
}
for (const flag of [false, true]) {
  events.push(await f(flag));
  events.push([...g(flag)]);
}
console.log(JSON.stringify(events));
"#;
    let result = checked(source, "module");
    assert!(result.contains("if (!(await Promise.resolve(flag)))"));
    assert!(result.contains("if (!(flag))"));
}
