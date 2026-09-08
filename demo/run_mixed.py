"""Check one real Python + JavaScript Shear invocation in disposable clones.

Trusted local CPython/path-browserify inputs only. Tests execute upstream code;
this is not a sandbox. Each command has a 60-second deadline.
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

import run_cpython as cpython

JS_REVISION = "872fec31a8bac7b9b43be0e54ef3037e0202c5fb"
JS_HASH = "f6d60f9e7cd83f65f947cda34ef846e82706c044d9a9fa7c5b69718b13b78e28"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python-repo", required=True, type=Path)
    parser.add_argument("--javascript-repo", required=True, type=Path)
    parser.add_argument("--shear", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--js-lock", required=True, type=Path)
    parser.add_argument(
        "--prepare-only",
        action="store_true",
        help="Run baselines and leave original inputs ready for a live demonstration",
    )
    args = parser.parse_args()
    if sys.version_info[:3] != (3, 14, 7):
        parser.error("this validation profile requires Python 3.14.7")
    binary = args.shear.resolve(strict=True)
    lock = args.js_lock.resolve(strict=True)
    originals = [
        args.python_repo.resolve(strict=True),
        args.javascript_repo.resolve(strict=True),
    ]
    args.out.mkdir(parents=True, exist_ok=False)
    out = args.out.resolve()
    clones = [out / "cpython", out / "path-browserify"]
    sources = [cpython.SOURCE, "index.js"]
    hashes = [cpython.SOURCE_HASH, JS_HASH]
    revisions = [cpython.REVISION, JS_REVISION]
    env = {
        key: os.environ[key]
        for key in ("PATH", "HOME", "TMPDIR", "LANG")
        if key in os.environ
    }
    env["GIT_TERMINAL_PROMPT"] = "0"
    commands = []

    def run(name, argv, cwd=out, extra_env=None):
        argv = list(map(str, argv))
        print("$", " ".join(argv), flush=True)
        start = time.monotonic()
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
                "argv": argv,
                "cwd": str(cwd),
                "exit": result.returncode,
                "seconds": time.monotonic() - start,
            }
        )
        (out / "commands.json").write_text(json.dumps(commands, indent=2) + "\n")
        if result.returncode:
            raise RuntimeError(f"{name}: exit {result.returncode}\n{result.stderr}")
        return result

    for index, (original, clone, revision, source, expected_hash) in enumerate(
        zip(originals, clones, revisions, sources, hashes, strict=True)
    ):
        run(f"original-{index}-clean", ["git", "diff", "HEAD", "--exit-code"], original)
        observed = run(
            f"original-{index}-revision", ["git", "rev-parse", "HEAD"], original
        ).stdout.strip()
        if observed != revision or cpython.digest(original / source) != expected_hash:
            raise RuntimeError("upstream identity mismatch")
        run(
            f"clone-{index}",
            ["git", "clone", "--quiet", "--no-hardlinks", "--", original, clone],
        )
        run(
            f"checkout-{index}",
            ["git", "checkout", "--quiet", "--detach", revision],
            clone,
        )

    shutil.copyfile(lock, clones[1] / "package-lock.json")
    run(
        "npm-ci",
        [
            "npm",
            "ci",
            "--package-lock=true",
            "--ignore-scripts",
            "--no-audit",
            "--no-fund",
        ],
        clones[1],
    )
    run("node-version", ["node", "--version"])
    run("npm-version", ["npm", "--version"])
    py_env = {"PYTHONPATH": str(clones[0] / "Lib"), "PYTHONDONTWRITEBYTECODE": "1"}
    origin = run(
        "python-import-origin",
        [sys.executable, "-S", "-c", "import argparse; print(argparse.__file__)"],
        clones[0],
        py_env,
    ).stdout.strip()
    if Path(origin).resolve() != clones[0] / cpython.SOURCE:
        raise RuntimeError("wrong Python module under test")
    js_origin = run(
        "javascript-import-origin",
        ["node", "-e", "console.log(require.resolve('.'))"],
        clones[1],
    ).stdout.strip()
    if Path(js_origin).resolve() != clones[1] / "index.js":
        raise RuntimeError("wrong JavaScript module under test")
    checks = [
        (
            "python",
            [sys.executable, "-S", "-m", "unittest", "-v", "test.test_argparse"],
            clones[0],
            py_env,
        ),
        ("javascript", ["npm", "test"], clones[1], {}),
        ("javascript-extra", ["node", "test/test-path-parse-format.js"], clones[1], {}),
    ]
    baseline = {
        name: run(f"baseline-{name}", command, cwd, extra)
        for name, command, cwd, extra in checks
    }
    if args.prepare_only:
        for index, (clone, source, expected_hash) in enumerate(
            zip(clones, sources, hashes, strict=True)
        ):
            run(
                f"prepared-{index}-clean", ["git", "diff", "HEAD", "--exit-code"], clone
            )
            if cpython.digest(clone / source) != expected_hash:
                raise RuntimeError("baseline changed the demonstration input")
        prepared = {
            "source_revisions": revisions,
            "original_sha256": hashes,
            "shear": str(binary),
            "shear_binary_sha256": cpython.digest(binary),
            "python_executable": sys.executable,
            "baseline_passed": True,
            "rewrites_applied": False,
        }
        (out / "prepared.json").write_text(json.dumps(prepared, indent=2) + "\n")
        print(
            f"Baseline passed; original inputs ready, no rewrites applied: {out}",
            flush=True,
        )
        return
    selected = [
        str(clone.relative_to(out) / source)
        for clone, source in zip(clones, sources, strict=True)
    ]
    preview = run("preview", [binary, "--diff", "--explain", *selected])
    print(preview.stderr, end="", flush=True)
    run("rewrite", [binary, *selected])
    candidate = {
        name: run(f"candidate-{name}", command, cwd, extra)
        for name, command, cwd, extra in checks
    }
    for name, before in baseline.items():
        after = candidate[name]

        def normalize(text):
            return re.sub(
                r"^Ran (\d+) tests in [0-9.]+s$",
                r"Ran \1 tests",
                text,
                flags=re.MULTILINE,
            )

        if normalize(before.stdout + before.stderr) != normalize(
            after.stdout + after.stderr
        ):
            raise RuntimeError(f"{name}: named outcomes differ")
        print(
            name, "\n".join((after.stdout + after.stderr).splitlines()[-5:]), flush=True
        )
    run("idempotence", [binary, "--check", *selected])
    for index, (original, clone, source, expected_hash) in enumerate(
        zip(originals, clones, sources, hashes, strict=True)
    ):
        changed = run(
            f"changed-{index}", ["git", "diff", "--name-only", "HEAD"], clone
        ).stdout.splitlines()
        if changed != [source] or cpython.digest(original / source) != expected_hash:
            raise RuntimeError("no useful change or protected source changed")
        run(
            f"original-{index}-final-clean",
            ["git", "diff", "HEAD", "--exit-code"],
            original,
        )
        run(f"patch-{index}", ["git", "diff", "HEAD", "--", source], clone)
    summary = {
        "source_revisions": revisions,
        "original_sha256": hashes,
        "candidate_sha256": [
            cpython.digest(clone / source)
            for clone, source in zip(clones, sources, strict=True)
        ],
        "shear_binary_sha256": cpython.digest(binary),
        "dependency_lock_sha256": cpython.digest(lock),
        "python": sys.version,
        "rule_applications": len(preview.stderr.splitlines()),
        "named_outcomes_identical": True,
        "idempotent": True,
        "tracked_originals_unchanged": True,
        "note": "Two upstream projects assembled for this demo, not one existing mixed-language application.",
    }
    (out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(f"Checked mixed-language result and full evidence: {out}", flush=True)


if __name__ == "__main__":
    main()
