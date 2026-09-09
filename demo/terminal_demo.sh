#!/usr/bin/env bash
# Run in a real terminal, or source with --prepare-only for the VHS tape.
# The optional second argument otherwise controls the count-in.
# Requires the verified locale preparation, Python 3.14.7, Node, npm and bat.
set -euo pipefail
export PATH="/opt/homebrew/bin:$PATH"
prepared=${1:?Usage: bash demo/terminal_demo.sh PREPARED_DIRECTORY [COUNT_IN_SECONDS]}
count_in=${2:-5}
demo_dir=$(mktemp -d /private/tmp/shear-terminal.XXXXXX)
export DEMO_PREPARED="$prepared" DEMO_DIR="$demo_dir"
python3 - <<'PY'
import hashlib, json, os, shutil, subprocess
from pathlib import Path
p = Path(os.environ['DEMO_PREPARED']).resolve()
d = Path(os.environ['DEMO_DIR'])
m = json.loads((p / 'prepared.json').read_text())
digest = lambda f: hashlib.sha256(f.read_bytes()).hexdigest()
assert digest(Path(m['shear'])) == m['shear_sha256']
for name, expected in zip(m['files'], m['before_sha256'], strict=True):
    assert digest(p / name) == expected
for repo in ['cpython', 'path-browserify']:
    shutil.copytree(p / repo, d / repo, ignore=shutil.ignore_patterns('.git'))
(d / 'bin').mkdir()
(d / 'bin/shear').symlink_to(m['shear'])
shutil.copyfile(d / 'cpython/Lib/locale.py', d / 'before.py')
shutil.copyfile(d / 'path-browserify/index.js', d / 'before.js')
env = dict(os.environ, PYTHONPATH=str(d / 'cpython/Lib'), PYTHONDONTWRITEBYTECODE='1')
origin = subprocess.check_output(['python3', '-S', '-c', 'import locale; print(locale.__file__)'], env=env, text=True)
assert origin.strip() == str(d / 'cpython/Lib/locale.py')
checks = [('python', ['python3', '-S', '-m', 'unittest', '-v', 'test.test_locale'], d / 'cpython'),
          ('javascript', ['npm', 'test'], d / 'path-browserify'),
          ('javascript-extra', ['node', 'test/test-path-parse-format.js'], d / 'path-browserify')]
for name, command, cwd in checks:
    with (d / ('baseline-' + name + '.log')).open('w') as log:
        subprocess.run(command, cwd=cwd, env=env, stdout=log, stderr=subprocess.STDOUT, check=True, timeout=60)
print('Fresh inputs and baselines ready: ' + str(d))
PY
cd "$demo_dir"
export PATH="$demo_dir/bin:$PATH" PYTHONPATH="$demo_dir/cpython/Lib" PYTHONDONTWRITEBYTECODE=1
export BAT_THEME=Dracula BAT_PAGER=cat
bat() { command bat --color=always --paging=never "$@"; }
verify_demo() {
    shear --check cpython/Lib/locale.py path-browserify/index.js
    python3 -S -m unittest -v test.test_locale > python.log 2>&1
    python3 - <<'PY'
import hashlib, re
from pathlib import Path
normalize = lambda s: re.sub(r'^Ran (\d+) tests in [0-9.]+s$', r'Ran \1 tests', s, flags=re.M)
for name in ['python', 'javascript', 'javascript-extra']:
    assert normalize(Path('baseline-' + name + '.log').read_text()) == normalize(Path(name + '.log').read_text()), name
for name, expected in [
    ('cpython/Lib/locale.py', 'cffecafb527dd862e386d0d4c5034d779f087f7e905a4b217a277943006a929a'),
    ('path-browserify/index.js', '248abc366fd1326edf762130c21698c08a9b7f284789e4c394e1fad6dd233712'),
]:
    assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == expected, name
Path('verified.txt').write_text('Named upstream outcomes and checked candidate hashes match.\n')
PY
}
if [[ "$count_in" == --prepare-only ]]; then
    return 0
fi
# These fixed commands are displayed and then executed in the same shell.
show() { printf '\033[32m$ %s\033[0m\n' "$1"; sleep 0.6; eval "$1"; }
page() { printf '\033[2J\033[H'; }
printf 'Recording starts in %s seconds. Use a terminal at least 105 columns by 27 rows.\n' "$count_in"
sleep "$count_in"
page
show 'bat -n -r 208:226 cpython/Lib/locale.py'
sleep 10
page
show 'shear --explain cpython/Lib/locale.py path-browserify/index.js'
sleep 4
show 'bat -n -r 208:224 cpython/Lib/locale.py'
sleep 11
python3 - <<'PY'
import difflib
from pathlib import Path
before = Path('before.js').read_text().splitlines(True)
after = Path('path-browserify/index.js').read_text().splitlines(True)
Path('javascript.patch').write_text(''.join(difflib.unified_diff(before, after, fromfile='before/index.js', tofile='after/index.js')))
PY
page
show 'bat -l diff -p -r 1:20 javascript.patch'
sleep 9
page
show 'python3 -S -m unittest test.test_locale'
show '(cd path-browserify && npm test) > javascript.log 2>&1 && tail -5 javascript.log'
show '(cd path-browserify && node test/test-path-parse-format.js) > javascript-extra.log 2>&1 && tail -5 javascript-extra.log'
sleep 8
page
show 'shear --check cpython/Lib/locale.py path-browserify/index.js && echo "No further changes"'
sleep 6
# Keep full verbose outcomes for comparison without adding noise to the take.
verify_demo
