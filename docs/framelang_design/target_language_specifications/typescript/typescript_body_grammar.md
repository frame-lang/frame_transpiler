# Frame Target Body Grammar — TypeScript

Version: 0.1 (Draft)
Date: 2025-11-01

Body grammar for TypeScript target inside Frame action/handler bodies. Core
Frame constructs (`$State`, `$Child => $Parent`, `$>()`, `<$()`, `->`, `=> $^`,
`$$…`) are owned by the common grammar.

## Prolog
- File begins with `@target typescript`.
- Core parser owns headers/selectors/transitions/forwards.
- TS body parser handles statements/expressions inside `{ ... }`.

## Lexical Rules
- Comments: `//` and `/* … */` (non‑nesting).
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

## Disambiguation with Core
- Transition `->` is core only at statement start with `$State` follow; otherwise
  TypeScript treats `=>` for lambdas and never `->`.
- Parent forward `=> $^` is a core statement (not a lambda).
- `$` is a plain identifier char in TS bodies; `$State` headers never appear there.

## Initially Unsupported
- Optional chaining `?.`, nullish coalescing `??`, ternary `?:` (pending spec).

## Source Mapping
- Preserve offsets for dual‑origin diagnostics.
