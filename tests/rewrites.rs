use shear::{Language, Outcome, simplify};

fn simplify_python(source: &str) -> anyhow::Result<Outcome> {
    simplify(source, Language::Python)
}
use std::{
    io::Write,
    process::{Command, Stdio},
};

fn transformed(source: &str) -> String {
    let result = simplify_python(source).expect("valid fixture");
    let second = simplify_python(&result.source).expect("valid transformed source");
    assert!(
        second.rewrites.is_empty(),
        "not idempotent: {}",
        result.source
    );
    assert_eq!(second.source, result.source);
    result.source
}

#[test]
fn removes_else_after_return() {
    let input = "def f(x):\n    if x:\n        return 1\n    else:\n        return 2\n";
    assert_eq!(
        transformed(input),
        "def f(x):\n    if x:\n        return 1\n    return 2\n"
    );
}

#[test]
fn merges_nested_conditions_and_converges() {
    let input = "if a:\n    if b:\n        if c:\n            run()\n";
    assert_eq!(transformed(input), "if a and b and c:\n    run()\n");
}

#[test]
fn turns_terminal_else_into_guard() {
    let source = "def f(valid):\n    if valid:\n        value = 3\n        use(value)\n    else:\n        return 0\n    return value\n";
    assert_eq!(
        transformed(source),
        "def f(valid):\n    if not (valid):\n        return 0\n    value = 3\n    use(value)\n    return value\n"
    );
}

#[test]
fn guard_then_flattens_nested_flow() {
    let source = "def f(a, b, c):\n    if a:\n        if b:\n            if c:\n                work()\n    else:\n        return\n";
    let outcome = simplify_python(source).unwrap();
    assert_eq!(
        outcome.rewrites.iter().map(|r| r.rule).collect::<Vec<_>>(),
        vec!["guard-clause", "merge-nested-if"]
    );
    assert_eq!(
        transformed(source),
        "def f(a, b, c):\n    if not (a):\n        return\n    if b and c:\n        work()\n"
    );
}

#[test]
fn combines_rules() {
    let input = "def f(a, b):\n    if a:\n        return 1\n    else:\n        if b:\n            if a or b:\n                return 2\n    return 3\n";
    let result = simplify_python(input).unwrap();
    assert_eq!(result.rewrites.len(), 2);
    assert_eq!(
        transformed(input),
        "def f(a, b):\n    if a:\n        return 1\n    if b and (a or b):\n        return 2\n    return 3\n"
    );
}

#[test]
fn skips_uncertain_regions() {
    for source in [
        "if a:\n    work()\nelse:\n    other()\n",
        "if a:\n    if b:\n        work()\n    else:\n        other()\n",
        "if a:\n    # why b matters\n    if b:\n        work()\n",
        "if a:\n    if (b := work()):\n        use(b)\n",
        "if a:\n\tif b:\n\t\twork()\n",
        "if a:\n    if b:\n        text = '''hello\n        world'''\n",
        "if a:\n    if b: work()\n",
        "def f(a):\n    if a:\n        return 1\n    else: return 2\n",
        "def f(a):\n    if a:\n        return 1\n    else: # important\n        return 2\n",
        "def f(a):\n    if a: # meaningful\n        work()\n    else:\n        return 0\n",
        "def f(a):\n    if a:\n        work()\n    else:\n        maybe_exit()\n",
        "def f(a):\n    if a: work()\n    else:\n        return 0\n",
        "def f(a):\n    if a:\n        work()\n    else:\n        return 0 # noqa\n",
        "if a:\n    if b: # header comment:\n        work()\n",
        "if a:\n    if b:\n        # fmt: off\n        work()\n",
        "def f(a):\n    if a:\n        return 0\n    else:\n    # ambiguous ownership\n        work()\n",
        "def f(a):\n    if a:\n        work()\n    # between suites\n    else:\n        return 0\n",
    ] {
        assert_eq!(transformed(source), source, "unexpected rewrite: {source}");
    }
}

#[test]
fn guard_preserves_declaration_order() {
    for source in [
        "x = 42\ndef f(flag):\n    if flag:\n        global x\n        x = 7\n    else:\n        return x\n    return x\nprint(f(False), f(True))\n",
        "def outer():\n    x = 42\n    def f(flag):\n        if flag:\n            nonlocal x\n            x = 7\n        else:\n            return x\n        return x\n    return f(False), f(True)\nprint(outer())\n",
        "x = 42\ndef f(flag):\n    if flag:\n        for _ in range(1):\n            global x\n            x = 7\n    else:\n        return x\n    return x\nprint(f(False), f(True))\n",
    ] {
        let result = transformed(source);
        assert_eq!(execute(source), execute(&result));
        assert_eq!(result, source);
    }
}

#[test]
fn preserves_form_feed_indentation() {
    let source = "def f(x):\n    if x:\n        return 1\n    else:\n        value = 1\n        \x0c        value = 2\n    return value\nprint(f(False))\n";
    let result = transformed(source);
    assert_eq!(execute(source), execute(&result));
    assert_eq!(result, source);
}

#[test]
fn guard_preserves_local_order_and_finalizers() {
    for source in [
        "def f(ok):\n    if ok:\n        a = 1\n    else:\n        b = 2\n        return []\n    b = 3\n    return list(locals())\nprint(f(True))\n",
        "events = []\nclass Resource:\n    def __init__(self, name):\n        self.name = name\n    def __del__(self):\n        events.append(self.name)\ndef f(ok):\n    if ok:\n        a = Resource('a')\n    else:\n        b = Resource('unused')\n        return\n    b = Resource('b')\nf(True)\nprint(events)\n",
        "def f(ok):\n    if ok:\n        a = 1\n    else:\n        print(b)\n        return []\n    b = 3\n    return list(locals())\nprint(f(True))\n",
    ] {
        let result = transformed(source);
        assert_eq!(execute(source), execute(&result));
        assert_eq!(result, source);
    }
}

#[test]
fn removing_else_does_not_duplicate_blank_padding() {
    let source = "def f(x):\n    if x:\n        return 1\n\n    else:\n\n        return 2\n";
    assert_eq!(
        transformed(source),
        "def f(x):\n    if x:\n        return 1\n\n    return 2\n"
    );
}

#[test]
fn keeps_long_conditions_nested() {
    let source = "if configuration.should_validate_the_current_request:\n    if request.has_all_required_credentials_and_permissions:\n        accept()\n";
    assert_eq!(transformed(source), source);
    let source = "if a:\n    if b:\n        if condition_with_a_name_that_would_make_the_combined_header_unnecessarily_long:\n            accept()\n";
    assert_eq!(
        transformed(source),
        "if a and b:\n    if condition_with_a_name_that_would_make_the_combined_header_unnecessarily_long:\n        accept()\n"
    );
}

#[test]
fn merged_conditions_preserve_precedence() {
    for (condition, expected) in [
        ("a or b", "(a or b)"),
        ("a and b", "a and b"),
        ("a if b else c", "(a if b else c)"),
        ("a < b < c", "a < b < c"),
        ("not a", "not a"),
        ("(a or b)", "(a or b)"),
        ("lambda: False", "(lambda: False)"),
    ] {
        let input = format!("if {condition}:\n    if d:\n        pass\n");
        assert_eq!(
            transformed(&input),
            format!("if {expected} and d:\n    pass\n")
        );
        let program = format!(
            "for a in [False, True]:\n for b in [False, True]:\n  for c in [False, True]:\n   for d in [False, True]:\n    if {condition}:\n        if d:\n            print(a, b, c, d)\n"
        );
        assert_eq!(execute(&program), execute(&transformed(&program)));
    }
}

#[test]
fn flattens_exhaustive_exit_trees_and_elif_chains() {
    let source = "def f(a, b):\n    if a:\n        if b:\n            return 1\n        elif b is None:\n            raise ValueError('missing')\n        else:\n            return 2\n    else:\n        result = 3\n    return result\n";
    let output = transformed(source);
    assert!(!output.contains("else:"), "{output}");
    assert!(!output.contains("elif "), "{output}");
    assert!(output.contains("\n    result = 3\n"));
    let cases = "\nfor a in [False, True]:\n for b in [False, True, None]:\n  try:\n   print(f(a, b))\n  except ValueError as error:\n   print(type(error).__name__, str(error))\n";
    assert_eq!(
        execute(&(source.to_owned() + cases)),
        execute(&(output + cases))
    );
}

#[test]
fn named_guards_preserve_local_slots_and_truth_tests() {
    let source = "events = []\nclass Flag:\n    def __init__(self, value):\n        self.value = value\n    def __bool__(self):\n        events.append('truth')\n        return self.value\ndef f(flag, message):\n    if flag:\n        result = message.upper()\n        events.append('work')\n    else:\n        raise ValueError(message)\n    return result, list(locals())\n";
    let output = transformed(source);
    assert!(output.contains("if not (flag):\n        raise ValueError(message)"));
    let cases = "\nprint(f.__code__.co_varnames)\nfor flag in [True, False]:\n try:\n  print(f(Flag(flag), 'bad')[0])\n except ValueError as error:\n  print(type(error).__name__, str(error))\nprint(events)\n";
    assert_eq!(
        execute(&(source.to_owned() + cases)),
        execute(&(output + cases))
    );
}

#[test]
fn named_guards_skip_later_bindings_and_nested_scopes() {
    for source in [
        "def f(flag):\n    if flag:\n        a = 1\n    else:\n        raise Error()\n    Error = ValueError\n    return list(locals())\n",
        "def f(flag):\n    if flag:\n        a = 1\n    else:\n        return b\n    for b in []:\n        pass\n    return list(locals())\n",
        "def f(flag):\n    if flag:\n        a = 1\n    else:\n        return b\n    try:\n        pass\n    except Exception as b:\n        pass\n    return list(locals())\n",
        "def f(flag):\n    def inner():\n        return 0\n    if flag:\n        a = 1\n    else:\n        return inner()\n    return a\n",
    ] {
        assert_eq!(transformed(source), source);
    }
}

#[test]
fn shares_branch_continuations_without_reordering_local_slots() {
    let source = "events = []\ndef f(flag, value=3):\n    if flag:\n        result = value + 1\n        events.append(result)\n        return result\n    else:\n        result = value - 1\n        events.append(result)\n        return result\n";
    let output = transformed(source);
    assert_eq!(output.matches("events.append(result)").count(), 1);
    assert_eq!(output.matches("return result").count(), 1);
    assert!(output.contains("\n    events.append(result)\n    return result"));
    let cases = "\nprint(f.__code__.co_varnames)\nprint(f(True), f(False), events)\n";
    assert_eq!(
        execute(&(source.to_owned() + cases)),
        execute(&(output + cases))
    );
}

#[test]
fn shared_tail_preserves_effects_exceptions_and_loop_destinations() {
    let source = "events = []\ndef f(flag):\n    if flag:\n        result = 1\n        events.append(result)\n        raise ValueError(result)\n    else:\n        result = 2\n        events.append(result)\n        raise ValueError(result)\ndef loop():\n    result = []\n    for flag in [True, False]:\n        if flag:\n            result.append(1)\n            events.append(flag)\n            continue\n        else:\n            result.append(2)\n            events.append(flag)\n            continue\n    return result\nfor flag in [True, False]:\n try:\n  f(flag)\n except ValueError as error:\n  print(str(error))\nprint(loop(), events)\n";
    let output = transformed(source);
    assert_ne!(source, output);
    assert_eq!(execute(source), execute(&output));
}

#[test]
fn shared_tail_keeps_new_bindings_comments_and_semicolon_boundaries() {
    for source in [
        "def f(flag):\n    if flag:\n        a = 1\n        common = 2\n    else:\n        b = 1\n        common = 2\n    return list(locals())\n",
        "def f(flag):\n    if flag:\n        a = 1; print('same')\n    else:\n        b = 1; print('same')\n    return list(locals())\n",
        "def f(flag):\n    if flag:\n        a = 1\n        # shared explanation\n        print('same')\n    else:\n        b = 1\n        # shared explanation\n        print('same')\n    return list(locals())\n",
    ] {
        let output = transformed(source);
        assert_eq!(output, source);
        let cases = "\nprint(f(True), f(False))\n";
        assert_eq!(
            execute(&(source.to_owned() + cases)),
            execute(&(output + cases))
        );
    }
}

#[test]
fn local_slot_analysis_respects_annotations_and_canonical_names() {
    for source in [
        "def f(flag):\n    x: int\n    if flag:\n        a = 1\n        x = 2\n    else:\n        b = 1\n        x = 2\n    return list(locals())\nprint(f(False), f.__code__.co_varnames)\n",
        "def f(flag):\n    x: int\n    if flag:\n        a = 1\n    else:\n        return x\n    x = 2\n    return list(locals())\nprint(f(True), f.__code__.co_varnames)\n",
        "class C:\n    def f(self, flag):\n        if flag:\n            a = 1\n        else:\n            return _C__value\n        __value = 2\n        return list(locals())\nprint(C().f(True), C.f.__code__.co_varnames)\n",
        "def f(flag):\n    if flag:\n        a = 1\n    else:\n        return K\n    K = 2\n    return list(locals())\nprint(f(True), f.__code__.co_varnames)\n",
    ] {
        let output = transformed(source);
        assert_eq!(execute(source), execute(&output));
        assert_eq!(output, source);
    }
}

#[test]
fn incomplete_exit_trees_keep_the_alternative() {
    let source = "def f(a, b):\n    if a:\n        if b:\n            return 1\n        work()\n    else:\n        other()\n    return 3\n";
    assert_eq!(transformed(source), source);
}

#[test]
fn malformed_input_fails() {
    assert!(simplify_python("def f(:\n").is_err());
}

#[test]
fn preserves_suite_comments_when_lifting_else() {
    let source = "def f(x):\n    if x:\n        # explain exit\n        return 1 # exit\n    else:\n        # explain value\n        value = 2 # value\n        # end of alternative\n    # outside\n    return value\n";
    let expected = "def f(x):\n    if x:\n        # explain exit\n        return 1 # exit\n    # explain value\n    value = 2 # value\n    # end of alternative\n    # outside\n    return value\n";
    assert_eq!(transformed(source), expected);
}

#[test]
fn preserves_comments_inside_merged_body() {
    let source = "if a:\n    if b:\n        # action\n        work() # trailing\n        # tail\n";
    assert_eq!(
        transformed(source),
        "if a and b:\n    # action\n    work() # trailing\n    # tail\n"
    );
}

#[test]
fn preserves_comment_ownership_when_making_guard() {
    let source = "def f(a):\n    if a:\n        # normal work\n        work()\n    else:\n        # explain rejection\n        return 0 # rejection\n";
    assert_eq!(
        transformed(source),
        "def f(a):\n    if not (a):\n        # explain rejection\n        return 0 # rejection\n    # normal work\n    work()\n"
    );
}

#[test]
fn oversized_input_is_rejected() {
    assert!(simplify_python(&" ".repeat(2 * 1024 * 1024 + 1)).is_err());
}

fn execute(source: &str) -> Vec<u8> {
    let mut child = Command::new("python3")
        .args(["-I", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("python3 is required for behavioral regression tests");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(source.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

#[test]
fn differential_truthiness_effects_exceptions_and_loop_exits() {
    let source = r#"events = []
class Value:
    def __init__(self, name, value):
        self.name, self.value = name, value
    def __bool__(self):
        events.append(self.name)
        return self.value

def nested(a, b):
    if a:
        if b:
            events.append('body')

def branch(x):
    if x < 0:
        raise ValueError(x)
    else:
        y = x + 1
    return y

def loop():
    out = []
    for x in range(5):
        if x == 1:
            continue
        else:
            out.append(x)
        if x == 3:
            break
        else:
            out.append(-x)
    return out

for a in [False, True]:
    for b in [False, True]:
        nested(Value('a', a), Value('b', b))
for x in [-2, 0, 2]:
    try:
        events.append(branch(x))
    except ValueError as e:
        events.append(str(e))
print(events, loop())
"#;
    let result = transformed(source);
    assert_ne!(source, result);
    assert_eq!(execute(source), execute(&result));
}

#[test]
fn guard_preserves_truthiness_nan_and_finally_effects() {
    let source = r#"events = []
class Value:
    def __init__(self, value):
        self.value = value
    def __bool__(self):
        events.append('truth-test')
        if self.value == 'raise':
            raise ValueError('truth')
        return self.value

def choose(value):
    try:
        if value:
            events.append('work')
            result = 3
        else:
            events.append('exit')
            return 0
        return result
    finally:
        events.append('finally')

def compare(x):
    if x < 0:
        result = 'negative'
    else:
        return 'not-negative'
    return result

def loop():
    out = []
    for x in range(5):
        if x != 1:
            out.append(x)
        else:
            continue
        if x < 3:
            out.append(-x)
        else:
            break
    return out

for value in [False, True, 'raise']:
    try:
        events.append(choose(Value(value)))
    except ValueError as error:
        events.append(str(error))
print(events, [compare(x) for x in [-1, 0, float('nan')]], loop())
"#;
    let result = simplify_python(source).unwrap();
    assert_eq!(
        result
            .rewrites
            .iter()
            .filter(|r| r.rule == "guard-clause")
            .count(),
        4
    );
    assert_eq!(execute(source), execute(&result.source));
    assert_eq!(transformed(source), result.source);
}

#[test]
fn guards_preserve_async_contexts_and_generator_cleanup() {
    let source = r#"import asyncio
events = []
class Scope:
    async def __aenter__(self):
        events.append('enter')
    async def __aexit__(self, kind, value, traceback):
        events.append('exit')

async def choose(flag):
    async with Scope():
        if flag:
            events.append('work')
            await asyncio.sleep(0)
        else:
            return 0
    return 1

def generate(flag):
    try:
        if flag:
            yield 1
            yield 2
        else:
            return 0
    finally:
        events.append('generator-finally')

async def async_generate(flag):
    try:
        if flag:
            yield 1
        else:
            return
    finally:
        events.append('async-generator-finally')

async def main():
    for flag in [False, True]:
        events.append(await choose(flag))
        events.append(list(generate(flag)))
        async for value in async_generate(flag):
            events.append(value)
    generator = generate(True)
    events.append(next(generator))
    try:
        generator.throw(ValueError('injected'))
    except ValueError:
        events.append('caught')
    generator = generate(True)
    events.append(next(generator))
    generator.close()
    asynchronous = async_generate(True)
    events.append(await anext(asynchronous))
    await asynchronous.aclose()

asyncio.run(main())
print(events)
"#;
    let result = simplify_python(source).unwrap();
    assert_eq!(
        result
            .rewrites
            .iter()
            .filter(|r| r.rule == "guard-clause")
            .count(),
        3
    );
    assert_eq!(execute(source), execute(&result.source));
    assert_eq!(transformed(source), result.source);
}

#[test]
fn preserves_unrelated_bytes_unicode_and_blank_lines() {
    let source = "# untouched\ndef café(x):\n    if x:\n        return 'é'\n    else:\n\n        return '☃'\n\n# footer\n";
    let result = transformed(source);
    assert!(result.starts_with("# untouched\ndef café(x):"));
    assert!(result.ends_with("\n\n# footer\n"));
    assert_eq!(
        execute(&(source.to_owned() + "print(café(0), café(1))\n")),
        execute(&(result + "print(café(0), café(1))\n"))
    );
}
