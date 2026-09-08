"""Compare fixed observations on disposable copies of the active Python stdlib.

Not a replacement for CPython's test suite or a real-project showcase qualification.
Requires Python 3.11+ and a built Shear binary; uses no third-party Python packages.
"""

import argparse
import hashlib
import importlib.util
import io
import json
import shutil
import subprocess
import sys
import sysconfig
import time
from pathlib import Path


def observe(call, *args, **kwargs):
    try:
        return {"value": call(*args, **kwargs)}
    except Exception as error:  # noqa: BLE001 — exceptions are compared observations
        return {"error": type(error).__name__, "message": str(error)}


def probe(kind, path):
    spec = importlib.util.spec_from_file_location("shear_probe", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    results = []
    if kind == "configparser":
        parser = module.ConfigParser(allow_no_value=True)
        parser.read_string("[DEFAULT]\nbase=42\n[s]\nx=%(base)s\nflag\n")
        for section, option in [
            ("s", "x"),
            ("s", "flag"),
            ("s", "missing"),
            ("absent", "x"),
        ]:
            results.append(observe(parser.get, section, option))
            results.append(observe(parser.get, section, option, fallback="fallback"))
            results.append(observe(parser.get, section, option, raw=True))
        results.append(list(parser["DEFAULT"]))
        results.append(list(parser["s"]))
        results.append(observe(lambda: parser.set("s", "x", 42)))
        results.append(observe(lambda: parser.add_section("s")))
        if hasattr(module, "UNNAMED_SECTION"):
            results.append(observe(lambda: parser.add_section(module.UNNAMED_SECTION)))
        output = io.StringIO()
        parser.write(output)
        results.append(output.getvalue())
    elif kind == "urllib":
        for authority in [
            "example.com",
            "example.com:0",
            "example.com:65535",
            "example.com:65536",
            "example.com:bad",
            "example.com:１２",
            "[::1]:443",
        ]:
            for url in [f"https://{authority}/a?q=1#frag", f"https://{authority}/a"]:
                result = module.urlsplit(url)
                results.append(observe(getattr, result, "port"))
                results.append(result.geturl())
        for url in ["https://example.com/a", "https://example.com/a#x"]:
            results.append(module.urldefrag(url).geturl())
            results.append(module.urldefrag(url.encode()).geturl().decode())
        results.append(module.parse_qsl("a=1&a=2&blank=", keep_blank_values=True))
    else:
        parser = module.ArgumentParser(prog="probe", exit_on_error=False)
        parser.add_argument("--count", type=int, default=1)
        parser.add_argument("--mode", choices=["a", "b"], default="a")
        parser.add_argument("items", nargs="*")
        for args in [
            [],
            ["--count", "3", "x", "y"],
            ["--count", "bad"],
            ["--mode", "c"],
            ["--mode", "b", "--", "-x"],
        ]:
            results.append(observe(lambda args=args: vars(parser.parse_args(args))))
        results.append(parser.format_help())
    print(json.dumps(results, sort_keys=True))


def run(args):
    result = subprocess.run(
        args, capture_output=True, text=True, timeout=30, check=False
    )
    if result.returncode:
        raise RuntimeError(f"{args!r}: exit {result.returncode}\n{result.stderr}")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--shear", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=False)
    binary = str(args.shear.resolve())
    script = str(Path(__file__).resolve())
    stdlib = Path(sysconfig.get_path("stdlib"))
    report = {
        "python": sys.version,
        "binary_sha256": hashlib.sha256(Path(binary).read_bytes()).hexdigest(),
        "modules": [],
    }
    for kind, relative in [
        ("configparser", "configparser.py"),
        ("argparse", "argparse.py"),
        ("urllib", "urllib/parse.py"),
    ]:
        started = time.monotonic()
        source = stdlib / relative
        original = source.read_bytes()
        directory = args.out / kind
        directory.mkdir()
        copy = directory / "source.py"
        shutil.copyfile(source, copy)
        probe_args = [
            sys.executable,
            "-I",
            script,
            "--probe",
            kind,
            str(copy.resolve()),
        ]
        before = run(probe_args).stdout
        (directory / "before.json").write_text(before)
        preview = run([binary, "--diff", "--explain", str(copy)])
        (directory / "patch.diff").write_text(preview.stdout)
        (directory / "rules.log").write_text(preview.stderr)
        run([binary, str(copy)])
        run([sys.executable, "-I", "-m", "py_compile", str(copy)])
        after = run(probe_args).stdout
        (directory / "after.json").write_text(after)
        if before != after:
            raise RuntimeError(f"observations changed: {kind}")
        run([binary, "--check", str(copy)])
        if source.read_bytes() != original:
            raise RuntimeError(f"installed source changed: {source}")
        report["modules"].append(
            {
                "file": relative,
                "original_sha256": hashlib.sha256(original).hexdigest(),
                "candidate_sha256": hashlib.sha256(copy.read_bytes()).hexdigest(),
                "rule_applications": len(preview.stderr.splitlines()),
                "observations_equal": True,
                "idempotent": True,
                "original_unchanged": True,
                "seconds": time.monotonic() - started,
            }
        )
    (args.out / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    if len(sys.argv) == 4 and sys.argv[1] == "--probe":
        probe(sys.argv[2], sys.argv[3])
    else:
        main()
