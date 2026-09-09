# Run from the repository root; preparation stays outside the visible take.
Require bash
Require python3
Require node
Require npm
Require bat
Require diff
Output demo/output/shear-terminal-raw.mp4
Set Shell bash
Set Width 1920
Set Height 900
Set FontSize 28
Set FontFamily "Menlo"
Set Padding 30
Set Theme "Dracula"
Set Framerate 30
Set TypingSpeed 50ms
Set CursorBlink false

# Allow ttyd to consume Enter before checking the prompt.
Hide
Type@1ms `export SHEAR_RECORDING_ROOT="$PWD"; source demo/terminal_demo.sh /private/tmp/shear-desktop-locale --prepare-only; export PS1='$ '; unset PROMPT_COMMAND; printf '%s\n' "$PWD" > "$SHEAR_RECORDING_ROOT/demo/output/workspace.txt"; clear`
Enter
Sleep 300ms
Wait+Line@120s /^\$ ?$/
Show
Type "bat -n -r 208:226 cpython/Lib/locale.py"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
Screenshot demo/output/01-before.png
Sleep 10s
Ctrl+L
Type "shear --explain cpython/Lib/locale.py path-browserify/index.js"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
Screenshot demo/output/02-apply.png
Sleep 5s
Ctrl+L
Type "bat -n -r 208:224 cpython/Lib/locale.py"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
Screenshot demo/output/02-after.png
Sleep 10s
Hide
Type@1ms "diff -U4 --label before/index.js --label after/index.js before.js path-browserify/index.js > javascript.patch || test $? -eq 1"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
Ctrl+L
Show
Type "bat -l diff -p -r 1:17 javascript.patch"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
Screenshot demo/output/03-javascript.png
Sleep 7s
Ctrl+L
Set TypingSpeed 15ms
Type "python3 -S -m unittest test.test_locale > locale.log 2>&1 && tail -4 locale.log"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
Type "(cd path-browserify && npm test) > javascript.log 2>&1 && tail -5 javascript.log"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
Type "(cd path-browserify && node test/test-path-parse-format.js) > javascript-extra.log 2>&1 && tail -5 javascript-extra.log"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
Screenshot demo/output/04-tests.png
Sleep 4s
Ctrl+L
Type "bat -n -r 218:224 cpython/Lib/locale.py"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
Screenshot demo/output/05-closeup.png
Sleep 6s
Hide
Type@1ms "verify_demo"
Enter
Sleep 300ms
Wait+Line /^\$ ?$/
