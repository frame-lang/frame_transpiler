# Frame Target Body Grammar — Python

Version: 0.1 (Draft)
Date: 2025-11-01

This document specifies the Python target body grammar used inside Frame action
and handler bodies. Core Frame constructs (system/blocks, `$State` headers,
`$>()`/`<$()`, `->`, `=> $^`, `$$…`) are defined in the common grammar and are
not redefined here.

## Prolog
- File begins with `@target python` (first non‑whitespace token). Comments are not permitted before the prolog.
- Core parser owns state headers, handler selectors, transitions, forwards.
- Python body parser handles statements/expressions within `{ ... }` bodies.

## Lexical Rules
- Comments: `#` to end of line. After the prolog, `#` applies throughout the file (shell + bodies). Triple‑quoted strings are strings/docstrings, not comments.
- Strings: single, double, triple-quoted; f‑strings allowed.
- Identifiers: Python identifier rules.
- Numbers: int, float; underscores permitted.

## Operators
- Boolean: `and`, `or`, `not`.
- Equality: `==`, `!=` (Python semantics); `is`/`is not` allowed.
- Arithmetic/bitwise: as per Python.
- Floor division: `//`.

## Statements
- if/elif/else; for/while; try/except/finally; with; return; break/continue; pass.
- Comprehensions, slices, generator expressions allowed if visitor supports.
- Native imports follow standard Python syntax:
  - `import module`
  - `import module as alias`
  - `from module import symbol [as alias]`
  - `from package.module import (symbol_1, symbol_2, …)`
- Imported symbols are scanned and forwarded to the `.fid` generator so Frame can validate subsequent usage.

## Body Policy (Native‑Only)

- Inside action/operation/event handler bodies and free `fn` function bodies, use native Python syntax exclusively:
  - No `var` declarations in bodies; use native assignments (`x = expr`).
  - No brace‑style control flow (`if { ... } else { ... }`); use colon + indentation (`if ...:\n    ...`).
- Frame structural declarations remain Frame‑style:
  - State and class fields are declared with `var` inside `$State { ... }` and `class { ... }` blocks (e.g., `var count:int = 0`).
  - The `domain:` block uses native Python assignments (`name[: type] = expr`).

## Disambiguation with Core
- Transition `->` is core only at statement start followed by a `$State` pattern;
  otherwise Python uses `->` only in type annotations of `def`, which do not
  appear inside Frame inline bodies.
- Parent forward is exactly `=> $^` at statement start (never a Python token).
- `$` outside core regions is not used.

## Initially Unsupported (subject to visitor capability)
- Metaclasses and advanced reflection.
- Dynamic `exec`/`eval`.

## Source Mapping
- Python body parser must preserve start/end offsets for dual‑origin diagnostics.
