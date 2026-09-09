# Shared continuation after existing autofixes

CPython `Lib/locale.py`, revision `823f0323ee6ec1402088b73bce1a38473cac36dc` (v3.14.7), contains duplicated padding cleanup at the end of the decimal and integer branches in `_localize`. Shear retains each branch's distinct work and moves that cleanup into a shared continuation.

The shape is:

```python
if decimal:
    formatted = _format_decimal(value)
    if seps:
        formatted = _strip_padding(formatted, seps)
else:
    formatted = _format_integer(value)
    if seps:
        formatted = _strip_padding(formatted, seps)
return formatted
```

After `shared-branch-tail`:

```python
if decimal:
    formatted = _format_decimal(value)
else:
    formatted = _format_integer(value)
if seps:
    formatted = _strip_padding(formatted, seps)
return formatted
```

## Checked comparison

Ruff **0.16.6** was run on a disposable copy with:

```sh
ruff check --isolated --no-cache \
  --select RET,SIM,FURB,PIE,PLR,RUF,C4 --fix --unsafe-fixes locale.py
shear --diff --explain locale.py
shear locale.py
shear --check locale.py
```

Ruff fixed 23 diagnostics and left six non-fixed diagnostics; its exit status was 1. The duplicated cleanup remained. Shear then applied one `shared-branch-tail` transformation; its second check returned zero. This is an observed difference with this version and configuration, not a claim about every Ruff rule or other tools.

## Verification

The unchanged `test.test_locale` suite passed on the original, Ruff-only, and Ruff-then-Shear versions: **66 tests, one unchanged skip** at each stage. Named outcomes matched after normalizing only elapsed time. Tests used Python 3.14.7, the disposable checkout's `Lib` on `PYTHONPATH`, and `python3 -S -m unittest -v test.test_locale`.

| Input | SHA-256 |
|---|---|
| Original | `f2e390ebde2bb52eabcf6d444a6a2e758749f3aa82c61427cbb053332fac56b2` |
| After Ruff | `3eac9026954ceb91b774c02666ebdb05d41f18acc7e482310be3be5d44fd53a2` |
| After Ruff then Shear | `cffecafb527dd862e386d0d4c5034d779f087f7e905a4b217a277943006a929a` |

The recorded patch is retrievable from Git history at commit `a4a02ad`. Original sources and upstream tests were not edited. These finite checks do not prove equivalence for arbitrary programs.
