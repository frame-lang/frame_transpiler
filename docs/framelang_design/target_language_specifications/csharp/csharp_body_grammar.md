# Frame Target Body Grammar — C#

Version: 0.1 (Draft)
Date: 2025-11-01

C# subset for Frame action/handler bodies. Core Frame constructs handled by the
common grammar.

## Prolog
- `@target csharp` at file start.
- Core parser owns headers/selectors/transitions/forwards.
- C# body parser owns `{ ... }` regions.

## Lexical Rules
- Comments: `//` and `/* … */`.
- Identifiers/literals per C#.

## Operators
- Boolean: `&&`, `||`, `!`.
- Equality: `==`, `!=` per C# (operator overloads may apply).
- Initially omit: `?.`, `??` until semantics are standardized.

## Statements
- if/else, for/while/do‑while, switch, try/catch/finally, return, break/continue.

## Disambiguation with Core
- Transition `->` recognized only at statement start with `$State` follow; not a
  C# token elsewhere.
- Parent forward `=> $^` is a core statement; `=>` in lambdas has different shape.

## Source Mapping
- Preserve offsets for dual‑origin diagnostics.
## Domain Declarations (C# target)

Domain variables declared in the `domain:` block use Frame style
(`var name [: type] = value`). The C# visitor maps them to class fields with
initializers.

Example (Frame):
```
domain:
    var count: int = 0
    var text: string = ""
```
