use std::{
    fs,
    process::{Command, Output},
};

const SOURCE: &str = "if a:\n    if b:\n        run()\n";
fn run(args: &[&str], dir: &std::path::Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_shear"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

#[test]
fn check_diff_write_and_idempotence() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("sample.py");
    fs::write(&path, SOURCE).unwrap();
    let check = run(&["--check", "sample.py"], dir.path());
    assert_eq!(check.status.code(), Some(1));
    assert_eq!(fs::read_to_string(&path).unwrap(), SOURCE);
    let diff = run(&["--diff", "--explain", "sample.py"], dir.path());
    assert!(diff.status.success());
    assert!(String::from_utf8_lossy(&diff.stdout).contains("+if a and b:"));
    assert!(
        String::from_utf8_lossy(&diff.stdout).starts_with("--- a/sample.py\n+++ b/sample.py\n")
    );
    assert!(String::from_utf8_lossy(&diff.stderr).contains("merge-nested-if"));
    assert_eq!(fs::read_to_string(&path).unwrap(), SOURCE);
    assert!(run(&["sample.py"], dir.path()).status.success());
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        "if a and b:\n    run()\n"
    );
    assert!(run(&["--check", "sample.py"], dir.path()).status.success());
    assert!(run(&["sample.py"], dir.path()).stderr.is_empty());
}

#[test]
fn invalid_batch_does_not_modify_valid_file() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.py"), SOURCE).unwrap();
    fs::write(dir.path().join("b.py"), "def broken(:\n").unwrap();
    assert_eq!(run(&["."], dir.path()).status.code(), Some(2));
    assert_eq!(fs::read_to_string(dir.path().join("a.py")).unwrap(), SOURCE);
}

#[test]
fn ignore_discovery_and_explicit_unsupported_input() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join(".ignore"), "ignored.py\n").unwrap();
    fs::write(dir.path().join("ignored.py"), SOURCE).unwrap();
    fs::write(dir.path().join("notes.txt"), SOURCE).unwrap();
    assert!(run(&["--check", "."], dir.path()).status.success());
    assert_eq!(run(&["notes.txt"], dir.path()).status.code(), Some(2));
    assert_eq!(
        run(&["--check", "ignored.py"], dir.path()).status.code(),
        Some(1)
    );
}

#[cfg(unix)]
#[test]
fn symlinks_are_not_written_and_permissions_survive() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("source.py");
    fs::write(&file, SOURCE).unwrap();
    fs::set_permissions(&file, fs::Permissions::from_mode(0o750)).unwrap();
    symlink(&file, dir.path().join("link.py")).unwrap();
    assert_eq!(run(&["link.py"], dir.path()).status.code(), Some(2));
    assert!(run(&["source.py"], dir.path()).status.success());
    assert_eq!(
        fs::metadata(&file).unwrap().permissions().mode() & 0o777,
        0o750
    );
}

#[cfg(unix)]
#[test]
fn rejects_symlinked_ancestors_before_any_batch_write() {
    use std::os::unix::fs::symlink;
    let dir = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(dir.path()).unwrap();
    fs::create_dir(root.join("outside")).unwrap();
    fs::write(root.join("outside/target.py"), SOURCE).unwrap();
    fs::write(root.join("local.py"), SOURCE).unwrap();
    symlink(root.join("outside"), root.join("linked")).unwrap();
    let absolute = root.join("linked/target.py");
    for path in [
        "linked/target.py",
        absolute.to_str().unwrap(),
        "linked/../local.py",
    ] {
        let output = run(&["local.py", path], &root);
        assert_eq!(output.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&output.stderr).contains("symlink"));
        assert_eq!(fs::read_to_string(root.join("local.py")).unwrap(), SOURCE);
        assert_eq!(
            fs::read_to_string(root.join("outside/target.py")).unwrap(),
            SOURCE
        );
    }
}

#[test]
fn rewrite_budget_failure_preserves_all_files() {
    let dir = tempfile::tempdir().unwrap();
    let many = "if a:\n    if b:\n        pass\n".repeat(1025);
    fs::write(dir.path().join("a.py"), SOURCE).unwrap();
    fs::write(dir.path().join("b.py"), &many).unwrap();
    let output = run(&["."], dir.path());
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("rewrite limit"));
    assert_eq!(fs::read_to_string(dir.path().join("a.py")).unwrap(), SOURCE);
    assert_eq!(fs::read_to_string(dir.path().join("b.py")).unwrap(), many);
}

#[test]
fn help_is_formatter_shaped() {
    let dir = tempfile::tempdir().unwrap();
    let output = run(&["--help"], dir.path());
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    for flag in ["--check", "--diff", "--explain"] {
        assert!(help.contains(flag));
    }
    assert!(!help.contains("--model"));
}

const JS: &str = "function f(flag) {\n  if (flag) return 1;\n  else {\n    return 2;\n  }\n}\n";

#[test]
fn mixed_languages_share_preview_write_and_idempotence() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.py"), SOURCE).unwrap();
    for extension in ["js", "mjs", "cjs"] {
        fs::write(dir.path().join(format!("b.{extension}")), JS).unwrap();
    }
    let preview = run(&["--diff", "--explain", "."], dir.path());
    assert!(preview.status.success());
    let explanation = String::from_utf8(preview.stderr).unwrap();
    assert_eq!(explanation.matches("redundant-else").count(), 3);
    assert_eq!(explanation.matches("merge-nested-if").count(), 1);
    assert_eq!(fs::read_to_string(dir.path().join("b.js")).unwrap(), JS);
    assert_eq!(run(&["--check", "."], dir.path()).status.code(), Some(1));
    assert!(run(&["."], dir.path()).status.success());
    assert!(run(&["--check", "."], dir.path()).status.success());
    assert!(
        !fs::read_to_string(dir.path().join("b.js"))
            .unwrap()
            .contains("else")
    );
}

#[test]
fn malformed_second_language_prevents_all_writes() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.py"), SOURCE).unwrap();
    fs::write(dir.path().join("b.js"), "function f( {").unwrap();
    let output = run(&["."], dir.path());
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("JavaScript"));
    assert_eq!(fs::read_to_string(dir.path().join("a.py")).unwrap(), SOURCE);
}

#[test]
fn discovery_excludes_node_modules_but_explicit_files_are_allowed() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("node_modules")).unwrap();
    fs::write(dir.path().join("node_modules/dependency.js"), JS).unwrap();
    assert!(run(&["--check", "."], dir.path()).status.success());
    assert_eq!(
        run(&["--check", "node_modules/dependency.js"], dir.path())
            .status
            .code(),
        Some(1)
    );
    fs::write(dir.path().join("unsupported.ts"), JS).unwrap();
    assert_eq!(run(&["unsupported.ts"], dir.path()).status.code(), Some(2));
}
