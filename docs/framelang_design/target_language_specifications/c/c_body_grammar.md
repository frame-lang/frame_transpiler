# Frame Target Body Grammar — C

Version: 0.1 (Draft)
Date: 2025-11-01

This document defines the C subset accepted in Frame action/handler bodies.
Core Frame constructs are handled by the common grammar.

## Prolog
- `@target c` at file start.
- Core parser owns headers, selectors, transitions, forwards.
- C body parser parses statements/expressions within `{ ... }`.

## Lexical Rules
- Comments: `//` and `/* … */` (non‑nesting).
- Identifiers: C rules.
- Literals: integer/float/char/string (escape sequences per C standard).

## Operators
- Boolean/arithmetic/bitwise per C. Short‑circuit `&&`, `||`, unary `!`.
- Equality: `==`, `!=` as in C.

## Statements
- if/else, for/while/do‑while, switch, return, break/continue.
- Declarations within bodies if visitor permits; otherwise assume predeclared locals.

## Disambiguation with Core
- `p->field` inside bodies must not be parsed as a transition; transitions are
  recognized only at statement start and must lead to `$State`.
- `=> $^` is a core statement and never a C token.

## Initially Unsupported
- Preprocessor directives inside bodies.
- Variable length arrays and non‑portable extensions.

## Source Mapping
- Preserve offsets for dual‑origin diagnostics.
