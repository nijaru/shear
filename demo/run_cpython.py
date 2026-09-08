"""Reproduce the checked CPython argparse demo in a new disposable checkout.

Requires the trusted local CPython v3.14.7 clone, Python 3.14.7, and a built Shear.
Runs real commands and saves their outputs; never installs or injects a patch.
"""

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

REVISION = "823f0323ee6ec1402088b73bce1a38473cac36dc"
SOURCE = "Lib/argparse.py"
TEST = "Lib/test/test_argparse.py"
SOURCE_HASH = "b9f0fa53be3d7c9c10a71a8eecc538a27c32fbe7abe337818d0d36e138bfc5c5"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True, type=Path)
    parser.add_argument("--shear", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    if sys.version_info[:3] != (3, 14, 7):
        parser.error("this recorded validation profile requires Python 3.14.7")
    repo = args.repo.resolve(strict=True)
    binary = args.shear.resolve(strict=True)
    args.out.mkdir(parents=True, exist_ok=False)
    out = args.out.resolve()
    checkout = out / "checkout"
    env = {
        key: os.environ[key]
        for key in ("PATH", "HOME", "TMPDIR", "LANG")
        if key in os.environ
    }
    env["GIT_TERMINAL_PROMPT"] = "0"
    commands = []

    def run(name, argv, cwd=out, extra_env=None, expected=0):
        print("$", " ".join(map(str, argv)), flush=True)
        started = time.monotonic()
        result = subprocess.run(
            argv,
            cwd=cwd,
            env=env | (extra_env or {}),
            capture_output=True,
            text=True,
            timeout=60,
            check=False,
        )
        (out / f"{name}.stdout").write_text(result.stdout)
        (out / f"{name}.stderr").write_text(result.stderr)
        commands.append(
            {
                "name": name,
                "argv": list(map(str, argv)),
                "cwd": str(cwd),
                "exit": result.returncode,
                "seconds": time.monotonic() - started,
            }
        )
        (out / "commands.json").write_text(json.dumps(commands, indent=2) + "\n")
        if result.returncode != expected:
            raise RuntimeError(f"{name}: exit {result.returncode}\n{result.stderr}")
        return result

    if run("original-status", ["git", "status", "--porcelain"], repo).stdout:
        raise RuntimeError("original CPython checkout must be clean")
    if (
        run("original-revision", ["git", "rev-parse", "HEAD"], repo).stdout.strip()
        != REVISION
    ):
        raise RuntimeError("original CPython revision does not match the profile")
    run(
        "clone",
        ["git", "clone", "--quiet", "--no-hardlinks", "--", str(repo), str(checkout)],
    )
    run("checkout", ["git", "checkout", "--quiet", "--detach", REVISION], checkout)
    if digest(checkout / SOURCE) != SOURCE_HASH:
        raise RuntimeError("source hash mismatch")
    test_hash = digest(checkout / TEST)
    test_env = {"PYTHONPATH": str(checkout / "Lib"), "PYTHONDONTWRITEBYTECODE": "1"}
    test_command = [sys.executable, "-S", "-m", "unittest", "-v", "test.test_argparse"]
    origin = run(
        "import-origin",
        [sys.executable, "-S", "-c", "import argparse; print(argparse.__file__)"],
        checkout,
        test_env,
    ).stdout.strip()
    if Path(origin).resolve() != (checkout / SOURCE).resolve():
        raise RuntimeError("tests would import the wrong argparse module")
    baseline = run("baseline", test_command, checkout, test_env)
    print("\n".join(baseline.stderr.splitlines()[-4:]), flush=True)
    preview = run("preview", [str(binary), "--diff", "--explain", SOURCE], checkout)
    (out / "rules.log").write_text(preview.stderr)
    print(preview.stderr, end="", flush=True)
    if not preview.stdout:
        raise RuntimeError("no structural change; this is not a successful demo")
    applied = run("rewrite", [str(binary), SOURCE], checkout)
    print(applied.stderr, end="", flush=True)
    candidate = run("candidate", test_command, checkout, test_env)
    print("\n".join(candidate.stderr.splitlines()[-4:]), flush=True)

    # Only unittest's measured duration may differ; compare every named outcome.
    def normalized(log):
        return re.sub(
            r"^Ran (\d+) tests in [0-9.]+s$", r"Ran \1 tests", log, flags=re.MULTILINE
        )

    if normalized(baseline.stderr) != normalized(candidate.stderr):
        raise RuntimeError("upstream test identities or outcomes changed")
    run("idempotence", [str(binary), "--check", SOURCE], checkout)
    changed = run(
        "changed-files", ["git", "diff", "--name-only"], checkout
    ).stdout.splitlines()
    if changed != [SOURCE] or digest(checkout / TEST) != test_hash:
        raise RuntimeError("protected tracked files changed")
    patch = run("patch", ["git", "diff", "--", SOURCE], checkout).stdout
    (out / "patch.diff").write_text(patch)
    if (
        digest(repo / SOURCE) != SOURCE_HASH
        or run("final-original-status", ["git", "status", "--porcelain"], repo).stdout
    ):
        raise RuntimeError("original checkout changed")
    summary = {
        "source_revision": REVISION,
        "source_sha256": SOURCE_HASH,
        "candidate_sha256": digest(checkout / SOURCE),
        "test_sha256": test_hash,
        "shear_binary_sha256": digest(binary),
        "python": sys.version,
        "rule_applications": len(preview.stderr.splitlines()),
        "test_outcomes_identical": True,
        "idempotent": True,
        "original_unchanged": True,
        "changed_files": changed,
    }
    (out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(
        "Checked rewrite saved; second run made no changes. Original checkout unchanged.",
        flush=True,
    )
    print(f"Full evidence: {out}", flush=True)


if __name__ == "__main__":
    main()
