# Frame Target Body Grammar — Java

Version: 0.1 (Draft)
Date: 2025-11-01

Java subset for Frame action/handler bodies. Core Frame constructs are handled
by the common grammar.

## Prolog
- `@target java` at file start.
- Core parser owns headers/selectors/transitions/forwards.
- Java body parser owns `{ ... }` regions.

## Lexical Rules
- Comments: `//` and `/* … */`.
- Identifiers/literals per Java.

## Operators
- Boolean: `&&`, `||`, `!`.
- Equality: `==`, `!=` with Java semantics (identity for references). Prefer
  `.equals()` for value comparison; visitors may normalize.

## Statements
- if/else, for/while/do‑while, switch, try/catch/finally, return, break/continue.

## Disambiguation with Core
- Transition `->` is core only when followed by a `$State` pattern at statement
  start. Java uses `->` in lambda bodies only in specific contexts; not confused.
- Parent forward `=> $^` is a core statement.

## Initially Unsupported
- Advanced generics/lambda shorthand where visitors lack support.

## Source Mapping
- Preserve offsets for dual‑origin diagnostics.
