# Frame Target Body Grammar — TypeScript

Version: 0.1 (Draft)
Date: 2025-11-01

Body grammar for TypeScript target inside Frame action/handler bodies. Core
Frame constructs (`$State`, `$Child => $Parent`, `$>()`, `<$()`, `->`, `=> $^`,
`$$…`) are owned by the common grammar.

## Prolog
- File begins with `@target typescript` (first non‑whitespace token). Comments are not permitted before the prolog.
- Core parser owns headers/selectors/transitions/forwards.
- TS body parser handles statements/expressions inside `{ ... }`.

## Lexical Rules
- Comments: `//` and `/* … */` (non‑nesting). After the prolog, these comment forms apply throughout the file (shell + bodies).
- Strings: single, double, template strings.
- Identifiers: ES rules.
- Numbers: int/float; bigint literal forms allowed if supported by visitor.

## Operators (policy)
- Boolean: `&&`, `||`, `!` map to Frame boolean ops.
- Equality: require strict `===`, `!==` for value equality/inequality.
  - Disallow `==`, `!=` (emit diagnostic).
- Disallowed initially: `??`, `?.`, `?:` until semantics are standardized.

## Statements
- if/else, for/while, try/catch/finally, return, break/continue, async/await.
- Function/arrow declarations inside bodies are allowed if visitor supports.
- Native imports follow standard ECMAScript module syntax:
  - `import { symbol } from "module";`
  - `import symbol from "./module";`
  - `import * as alias from "./module";`
  - Dynamic `await import("./module")` allowed but may be ignored by the `.fid` generator.
- Imported bindings are surfaced to the `.fid` generator so the compiler can validate subsequent usage.

## Disambiguation with Core
- Transition `->` is core only at statement start with `$State` follow; otherwise
  TypeScript treats `=>` for lambdas and never `->`.
- Parent forward `=> $^` is a core statement (not a lambda).
- `$` is a plain identifier char in TS bodies; `$State` headers never appear there.

## Initially Unsupported
- Optional chaining `?.`, nullish coalescing `??`, ternary `?:` (pending spec).

## Source Mapping
- Preserve offsets for dual‑origin diagnostics.

## Body Boundary Detection

Some TypeScript bodies include template literals with nested `${ … }` that can contain braces. To avoid miscounting braces when locating the end of a TS body, the compiler uses a textual, template‑aware scan that:

- Tracks strings (`'…'`, `"…"` with escapes), template literals (`` `…` ``) and nested `${ … }`, and comments (`//`, `/* … */`).
- Only counts `{` and `}` as body delimiters when outside those constructs.
- Handles CRLF newlines and NBSP/tab indentation without special cases.

“Template‑aware” means the scanner understands backticks and `${ … }` and does not treat braces inside those regions as body delimiters.

See also: `docs/framepiler_design/stages/ts_textual_body_closer.md` for algorithm details and unit‑tested behaviors. Current rollout: active for operations; staged for actions/handlers (guarded by backtick detection) with full suite validation after each change.
