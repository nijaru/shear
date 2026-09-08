use shear::simplify_python;
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
    assert_eq!(transformed(input), "if ((a) and (b)) and (c):\n    run()\n");
}

#[test]
fn combines_rules() {
    let input = "def f(a, b):\n    if a:\n        return 1\n    else:\n        if b:\n            if a or b:\n                return 2\n    return 3\n";
    let result = simplify_python(input).unwrap();
    assert_eq!(result.rewrites.len(), 2);
    assert_eq!(
        transformed(input),
        "def f(a, b):\n    if a:\n        return 1\n    if (b) and (a or b):\n        return 2\n    return 3\n"
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
        "def f(a):\n    if a:\n        return 1\n    elif b:\n        return 2\n    else:\n        return 3\n",
        "def f(a):\n    if a:\n        return 1\n    else:\n        return 2 # important\n",
    ] {
        assert_eq!(transformed(source), source, "unexpected rewrite: {source}");
    }
}

#[test]
fn malformed_input_fails() {
    assert!(simplify_python("def f(:\n").is_err());
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
