# Rewrite contracts

Python and JavaScript use separate semantic contracts, not generic AST equivalence laws. Implementations: `src/python.rs` and `src/javascript.rs`; behavioral evidence: `tests/rewrites.rs` and `tests/javascript.rs`.

## Python applicability

Tree-sitter must parse the entire UTF-8 file without errors. Skip a proposed region containing tabs, carriage returns, form feeds, named expressions, multiline strings, or recognized tooling directives (`type:`, `fmt:`, `ruff:`, `noqa`, `nosec`, `pylint:`, `pyright:`, `mypy:`, `isort:`, `pragma:`, `doctest:`). Skip one-line suites where lifting would require reconstructing layout. Source positions are byte ranges, not character indices.

Ordinary leading, inline, and trailing suite comments travel with their suite. Tree-sitter places leading comments outside the block node, so suite slices begin after the header newline. Skip comments attached to a header that must be rewritten, under-indented comments with ambiguous ownership, and comments between a pair of conditions that would be merged. Guard rewrites also skip comments between suites that cannot be assigned safely.

Every reparse compares the multiset of exact comment texts with the original: dropping, duplicating, or changing a comment is an error with no file write. This enforces text preservation, not meaning; ownership is additionally checked by rule-specific fixtures.

Shear does not preserve source-location introspection, debugger line numbers, traceback locations, coverage positions, or bytecode identity: moving source inherently changes those observations. Behavior preservation here concerns ordinary program values, effects, exceptions, binding, and control flow. Do not apply to code whose contract depends on source layout.

## `redundant-else`

Trigger: an `if` has an alternative, and its consequence always exits through explicit `return`, `raise`, `break`, or `continue`. Exhaustive nested `if`/`elif`/`else` branches propagate this fact; loops, calls and `try` statements do not.

Action: remove the `else` header and dedent its suite into the containing block. Preserve the original condition and the terminating branch. An `elif` after an exiting consequence becomes a separate `if` (`redundant-elif`), allowing the remaining chain to normalize on subsequent passes. Leading and trailing suite comments are retained; text outside the parsed region is never silently consumed. If blank padding already precedes the removed header, leading blank padding within the moved suite is dropped rather than duplicating it.

Reason: the taken consequence cannot reach the lifted suite. Python has no branch-local variable scope. Moving the suite one block outward does not cross a loop, function, exception handler, or context-manager boundary. Exit-looking calls are not treated as guaranteed termination.

Check: syntax, idempotence, nested contexts, early returns, raises, loop break/continue, and output/effect comparisons. Dynamic source introspection remains outside the contract above.

## `merge-nested-if`

Trigger: an `if` suite contains exactly one nested `if`, and neither conditional has an alternative. Both conditions occupy one physical line; the inner suite occupies later lines.

Action: join the conditions with `and` and dedent the inner suite one level. Preserve parentheses where precedence requires them (`or`, conditional expressions, and unknown operand shapes); known high-precedence operands and existing `and` chains do not gain redundant wrappers. Skip a merge when its header plus indentation would exceed 88 UTF-8 bytes. This conservative applicability budget avoids creating long lines without reformatting unrelated source.

Reason: Python's short-circuit condition evaluates and tests the outer expression first, and evaluates/tests the inner expression only when the outer succeeds. No declarations move across a scope boundary. This reasoning is specific to condition context and must not be generalized to replacing arbitrary Boolean-valued expressions.

Check: nested convergence, effectful truthiness (`__bool__`), evaluation order, negative alternatives, comments, multiline strings, Unicode, and malformed input.

## `guard-clause`

Trigger: an `if` has a plain `else` that always exits, while the consequence does not always exit. Both suites are multiline and the condition occupies one physical line. Shared conservative exclusions still apply.

Action: invert the condition with `not (condition)`, place the exiting suite first, and lift the normal suite after the guard. Do not invert comparison operators: NaN, overloaded comparisons, and non-Boolean comparison results make that a different operation. An exiting consequence is handled by `redundant-else` instead, so the rules do not alternate branch order. Guard reordering skips regions containing `global` or `nonlocal` declarations, conservatively including nested scopes: Python requires textual declaration-before-use ordering even across mutually exclusive branches. Removing nesting without reordering is not subject to this exclusion. Moving names across names can change CPython local-slot order, `locals()` order, and finalizer effects. Identifier-free suites remain eligible. Named exiting suites require function-level binding checks: their local names must already be established by parameters or earlier assignments with values. The analysis includes later bindings, skips nested/annotation scopes and declarations, and does not infer canonical identities for non-ASCII or name-mangled identifiers. Annotation-only statements do not establish a slot.

Reason: the original condition is truth-tested once; exactly one original suite executes. The exit prevents the lifted normal suite from executing on the false path. Python branch suites do not create binding scopes, and neither suite crosses its enclosing loop, function, exception handler, or context manager.

Check: direct return/raise, loop break/continue, effectful truthiness and exceptions, NaN comparison, `finally` effects, rule composition, comment/inline-suite skips, and idempotence. Each application removes an alternative, so the existing decreasing structural measure still holds.

## Python `shared-branch-tail`

Trigger: both branches of a plain `if`/`else` end with textually identical statements and retain distinct nonempty prefixes.

Action: keep the distinct work in its branches and move the shared continuation after the conditional. Retain condition evaluation and the selected prefix before the continuation. Comments in the conditional, semicolon-separated boundaries, ambiguous indentation, and scope-uncertain cases are skipped. The same function-level binding checks require the moved local names to have been established before the first original copy; later assignments, annotation-only declarations, Unicode normalization and private-name mangling must not change slot order.

Checks: shared calls/returns, exceptions, loop destinations, new-binding exclusions, comments, local-slot order and idempotence. CPython `locale._localize` supplies a checked real example: its duplicated padding cleanup becomes one shared continuation.

## JavaScript `redundant-else`

Trigger: an `if` directly inside a statement block or program has a braced `else`, and its consequence always exits through `return`, `throw`, `break`, or `continue`. Exhaustive conditionals propagate exits; loops, `try`, switches and labels do not. A consequence may be unbraced only with an explicit semicolon. Elif-style chains, labeled parents, and unbraced enclosing controls are skipped.

Action: preserve the condition and consequence, remove the alternative's braces/header, and dedent its contents. The last lifted statement must have an explicit semicolon: otherwise a following `(` or `[` could join a previously separate expression through automatic semicolon insertion. The same boundary check protects an unbraced consequence. Header/tail comments that would be reassigned cause a skip.

Scope: reject lexical, class, function, generator-function, and `using` declarations recursively, including `await using`. Removing a block must not widen a binding, change Annex B function behavior, or delay resource disposal. `var` retains its function/global scope and textual position. Nested declarations are conservatively rejected even where their own retained block might be sufficient. Ordinary comments are preserved; multiline literals/comments, JSX, tooling directives, and uncertain indentation are skipped.

Checks: Node execution before/after, CJS/ESM, variable hoisting, loop exits/labels, exceptions, async/generator cleanup, sync/async disposal, ASI counterexamples, comments, negative scope cases, and idempotence. The real `path-browserify` example additionally runs unchanged upstream tests. Source introspection such as `Function.prototype.toString()` is outside the preservation contract, just like source locations.

## JavaScript `merge-nested-if` and `guard-clause`

Nested braced conditionals without alternatives combine using grouped short-circuit `&&`, preserving evaluation order and limiting the resulting header to 88 bytes. The removed block must satisfy the lexical/resource-scope exclusions above. Ambiguous headers, comments, padding and multiline conditions are skipped.

A braced exiting alternative can become a negated guard, with the non-exiting branch lifted after it. Both scopes retain their control-flow destinations. Declaration reordering is excluded, including `var` because global property instantiation order is observable. ASI boundaries, disposal, effects, exceptions, async/generator suspension, comments and convergence have native Node regressions. Exit analysis is iterative so nesting does not consume the Rust call stack.

## Engine guarantees and limits

One deterministic preorder-selected edit is applied per pass, followed by a full reparse. Every current rule removes a statement or alternative, giving a decreasing structural measure. After 1,024 edits the engine fails rather than writing a partial result. It also refuses files over 2 MiB. Two runs must produce identical source; tests enforce this.

The implementation does not provide type analysis, complexity scoring, diagnostic-only smells, automatic native formatting, Git-changed selection, or languages beyond Python and JavaScript. Add those only with a concrete purpose and corresponding checks.
