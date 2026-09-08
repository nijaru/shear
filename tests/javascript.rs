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
