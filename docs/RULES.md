# Rewrite contracts

Python and JavaScript use separate semantic contracts, not generic AST equivalence laws. Implementations: `src/python.rs` and `src/javascript.rs`; behavioral evidence: `tests/rewrites.rs` and `tests/javascript.rs`.

## Python applicability

Tree-sitter must parse the entire UTF-8 file without errors. Skip a proposed region containing tabs, carriage returns, form feeds, named expressions, multiline strings, or recognized tooling directives (`type:`, `fmt:`, `ruff:`, `noqa`, `nosec`, `pylint:`, `pyright:`, `mypy:`, `isort:`, `pragma:`, `doctest:`). Skip one-line suites where lifting would require reconstructing layout. Source positions are byte ranges, not character indices.

Ordinary leading, inline, and trailing suite comments travel with their suite. Tree-sitter places leading comments outside the block node, so suite slices begin after the header newline. Skip comments attached to a header that must be rewritten, under-indented comments with ambiguous ownership, and comments between a pair of conditions that would be merged. Guard rewrites also skip comments between suites that cannot be assigned safely.

Every reparse compares the multiset of exact comment texts with the original: dropping, duplicating, or changing a comment is an error with no file write. This enforces text preservation, not meaning; ownership is additionally checked by rule-specific fixtures.

Shear does not preserve source-location introspection, debugger line numbers, traceback locations, coverage positions, or bytecode identity: moving source inherently changes those observations. Behavior preservation here concerns ordinary program values, effects, exceptions, binding, and control flow. Do not apply to code whose contract depends on source layout.

## `redundant-else`

Trigger: an `if` has a plain `else`, and the final direct statement of its consequence is `return`, `raise`, `break`, or `continue`.

Action: remove the `else` header and dedent its suite into the containing block. Preserve the original condition and the terminating branch. Elif chains are not handled. Leading and trailing suite comments are retained; text outside the parsed region is never silently consumed. If blank padding already precedes the removed header, leading blank padding within the moved suite is dropped rather than duplicating it.

Reason: the taken consequence cannot reach the lifted suite. Python has no branch-local variable scope. Moving the suite one block outward does not cross a loop, function, exception handler, or context-manager boundary. Exit-looking calls are not treated as guaranteed termination.

Check: syntax, idempotence, nested contexts, early returns, raises, loop break/continue, and output/effect comparisons. Dynamic source introspection remains outside the contract above.

## `merge-nested-if`

Trigger: an `if` suite contains exactly one nested `if`, and neither conditional has an alternative. Both conditions occupy one physical line; the inner suite occupies later lines.

Action: join the conditions with `and` and dedent the inner suite one level. Preserve parentheses where precedence requires them (`or`, conditional expressions, and unknown operand shapes); known high-precedence operands and existing `and` chains do not gain redundant wrappers. Skip a merge when its header plus indentation would exceed 88 UTF-8 bytes. This conservative applicability budget avoids creating long lines without reformatting unrelated source.

Reason: Python's short-circuit condition evaluates and tests the outer expression first, and evaluates/tests the inner expression only when the outer succeeds. No declarations move across a scope boundary. This reasoning is specific to condition context and must not be generalized to replacing arbitrary Boolean-valued expressions.

Check: nested convergence, effectful truthiness (`__bool__`), evaluation order, negative alternatives, comments, multiline strings, Unicode, and malformed input.

## `guard-clause`

Trigger: an `if` has a plain `else` whose final direct statement is `return`, `raise`, `break`, or `continue`, while the consequence does not directly terminate. Both suites are multiline and the condition occupies one physical line. Shared conservative exclusions still apply.

Action: invert the condition with `not (condition)`, place the exiting suite first, and lift the normal suite after the guard. Do not invert comparison operators: NaN, overloaded comparisons, and non-Boolean comparison results make that a different operation. An exiting consequence is handled by `redundant-else` instead, so the rules do not alternate branch order. Guard reordering skips regions containing `global` or `nonlocal` declarations, conservatively including nested scopes: Python requires textual declaration-before-use ordering even across mutually exclusive branches. Removing nesting without reordering is not subject to this exclusion. Additionally, at least one moved suite must contain no identifiers: moving names across names can change CPython's first symbol-encounter order, local-slot order, `locals()` order, and finalizer effects even when neither branch declares a new scope. Bare exits and literal returns remain useful safe cases; effectful named exits may be skipped until adequate scope analysis exists.

Reason: the original condition is truth-tested once; exactly one original suite executes. The exit prevents the lifted normal suite from executing on the false path. Python branch suites do not create binding scopes, and neither suite crosses its enclosing loop, function, exception handler, or context manager.

Check: direct return/raise, loop break/continue, effectful truthiness and exceptions, NaN comparison, `finally` effects, rule composition, comment/inline-suite skips, and idempotence. Each application removes an alternative, so the existing decreasing structural measure still holds.

## JavaScript `redundant-else`

Trigger: an `if` directly inside a statement block or program has a braced `else`, and its consequence directly ends in `return`, `throw`, `break`, or `continue`. A consequence may be unbraced only with an explicit semicolon. Elif-style chains, labeled parents, and unbraced enclosing controls are skipped.

Action: preserve the condition and consequence, remove the alternative's braces/header, and dedent its contents. The last lifted statement must have an explicit semicolon: otherwise a following `(` or `[` could join a previously separate expression through automatic semicolon insertion. The same boundary check protects an unbraced consequence. Header/tail comments that would be reassigned cause a skip.

Scope: reject lexical, class, function, generator-function, and `using` declarations recursively, including `await using`. Removing a block must not widen a binding, change Annex B function behavior, or delay resource disposal. `var` retains its function/global scope and textual position. Nested declarations are conservatively rejected even where their own retained block might be sufficient. Ordinary comments are preserved; multiline literals/comments, JSX, tooling directives, and uncertain indentation are skipped.

Checks: Node execution before/after, CJS/ESM, variable hoisting, loop exits/labels, exceptions, async/generator cleanup, sync/async disposal, ASI counterexamples, comments, negative scope cases, and idempotence. The real `path-browserify` example additionally runs unchanged upstream tests. Source introspection such as `Function.prototype.toString()` is outside the preservation contract, just like source locations.

## Engine guarantees and limits

One deterministic preorder-selected edit is applied per pass, followed by a full reparse. Every current rule removes one conditional or alternative, giving a decreasing structural measure. After 1,024 edits the engine fails rather than writing a partial result. It also refuses files over 2 MiB. Two runs must produce identical source; tests enforce this.

The implementation does not provide type analysis, complexity scoring, diagnostic-only smells, automatic native formatting, Git-changed selection, or languages beyond Python and JavaScript. Add those only with a concrete purpose and corresponding checks.
