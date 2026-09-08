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
    assert!(String::from_utf8_lossy(&diff.stdout).contains("+if (a) and (b):"));
    assert!(String::from_utf8_lossy(&diff.stderr).contains("merge-nested-if"));
    assert_eq!(fs::read_to_string(&path).unwrap(), SOURCE);
    assert!(run(&["sample.py"], dir.path()).status.success());
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        "if (a) and (b):\n    run()\n"
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
