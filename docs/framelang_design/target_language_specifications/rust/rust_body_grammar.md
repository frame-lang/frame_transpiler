# Frame Target Body Grammar — Rust

Version: 0.1 (Draft)
Date: 2025-11-01

Rust subset for Frame action/handler bodies. Core Frame constructs handled by the
common grammar.

## Prolog
- `@target rust` at file start.
- Core parser owns headers/selectors/transitions/forwards.
- Rust body parser owns `{ ... }` regions.

## Lexical Rules
- Comments: `//` and `/* … */`.
- Identifiers/literals per Rust (including raw strings if supported by visitor).

## Operators
- Boolean: `&&`, `||`, `!`.
- Equality: `==`, `!=` per Rust.
- Pattern matching via `match` may be supported if visitor implements it.

## Statements
- if/else, loop/while/for, match, return, break/continue.
- Closures: `|args| -> Ty { ... }` permitted; no conflict with Frame transitions.

## Disambiguation with Core
- Transition `->` is core only at statement start with `$State` follow; closures
  and function signatures use `->` in expression/type contexts.
- Parent forward `=> $^` is a core statement.

## Initially Unsupported
- Macros that change parsing significantly within bodies.

## Source Mapping
- Preserve offsets for dual‑origin diagnostics.
## Domain Declarations (Rust target)

Domain variables appear in `domain:` using Frame style (`var name [: type] = value`).
The Rust visitor emits them as struct fields with default initializers.

Example (Frame):
```
domain:
    var count: int = 0
    var label: string = ""
```
