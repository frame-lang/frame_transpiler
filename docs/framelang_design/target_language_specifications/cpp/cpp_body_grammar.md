# Frame Target Body Grammar — C++

Version: 0.1 (Draft)
Date: 2025-11-01

Subset of C++ accepted in Frame bodies. Core Frame constructs are handled by
the common grammar.

## Prolog
- `@target cpp` at file start.
- Core parser owns headers/selectors/transitions/forwards.
- C++ body parser owns `{ ... }` regions.

## Lexical Rules
- Comments: `//` and `/* … */`.
- Identifiers, literals: per C++ (string/char/number forms).

## Operators
- Full boolean/arithmetic/bitwise. Member access `->` allowed in bodies.
- Equality: `==`, `!=` as per C++.

## Statements
- if/else, for/while/do‑while, switch, return, break/continue.
- Lambdas allowed in expressions.

## Disambiguation with Core
- Transition `->` is core only at statement start with `$State` follow; member
  access `p->f` remains a body expression.
- Parent forward `=> $^` is a core statement only.

## Initially Unsupported
- Templates within bodies that require specialized parsing hooks.

## Source Mapping
- Preserve offsets for dual‑origin diagnostics.
