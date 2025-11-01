# Frame Common Grammar

## Purpose
This document summarizes the syntax every target must support before introducing language-specific features. It mirrors the shared AST produced by the Frame parser so target implementers know which constructs are universal.

Related specifications:
- Multi-target design: `../../../plans/design/multi_lang_design.md`
- Python body grammar: `../python/python_body_grammar.md`
- TypeScript body grammar: `../typescript/typescript_body_grammar.md`
- C body grammar: `../c/c_body_grammar.md`
- C++ body grammar: `../cpp/cpp_body_grammar.md`
- Java body grammar: `../java/java_body_grammar.md`
- C# body grammar: `../csharp/csharp_body_grammar.md`
- Rust body grammar: `../rust/rust_body_grammar.md`

## Target Prolog & Modes

- Every Frame file must begin with a target declaration prolog at the first non‑whitespace token:
  - `@target <language>` where `<language>` ∈ { python, typescript, c, cpp, csharp, java, rust }
- The compiler uses a multi‑lexer with two modes per file:
  - Core mode: recognizes Frame automata constructs (defined below)
  - Body mode: uses the selected target’s lexer for action/handler bodies and expression statements
- A two‑phase composite parser is used:
  - Core parser builds the automata skeleton and records body spans
  - Target body subparser (per language) parses each recorded span

## High-Level Structure

```
FrameModule
 ├─ imports*          # (planned) module imports
 ├─ functions*        # free functions (e.g., main)
 └─ systems+          # at least one system definition
```

### System Definition
```
system IDENTIFIER {
    [interface:    interface_block]
    [machine:      machine_block]
    [actions:      actions_block]
    [operations:   operations_block]
    [domain:       domain_block]
}
```

### Interface Block
```
interface:
    IDENTIFIER(parameters?) [-> return_type]
    ...
```
Parameters and return types follow Frame’s type annotation rules (`int`, `float`, `bool`, `string`, custom types). Optional `async` modifier is allowed per language design.

### Machine Block
```
machine:
    state_definition+
```
Each state definition has the form:
```
$StateName [=> $ParentState] {
    [event_handler+]
    [$>() handler]
    [$<() handler]
}
```

### Event Handler
```
EventName(parameters?) [async] {
    statement*
}
```

### Actions Block
```
actions:
    ActionName(parameters?) [async] {
        statement*
    }
    ...
```

### Operations Block
```
operations:
    OperationName(parameters?) [: return_type] {
        statement*
    }
    ...
```

### Domain Block
```
domain:
    var identifier [: type] = expression
    ...
```

## Statement Concepts (Language-Neutral)

The parser recognises the following concepts across targets. Each backend maps them to its native syntax while preserving semantics.

| Concept | Description | Notes |
| --- | --- | --- |
| Expression statement | Evaluate an expression for side effects | Target determines exact statement form |
| Assignment | Bind a value to a variable/domain field | Simple `=` semantics in AST |
| Transition | Change state (`->`) including stack push/pop (`$$[+]`, `$$[-]`) | Must invoke kernel hooks |
| Forward | Queue event for parent/next cycle (`=> $^`) | Uses kernel-forward APIs |
| Return | Exit handler/action optionally with value | AST carries optional expression |
| Conditional | If/else branching | Target chooses native conditional syntax |
| Loop | While/for constructs over iterables | AST captures loop condition/body |
| Exception handling | Try/catch/finally semantics | AST records handlers and error identifiers |
| Error signal | Raise/throw semantics | Propagates through kernel loop |
| Async/await | Awaiting asynchronous operations | AST marks awaited expression |
| Call | Invoke callable with arguments | Target emits native call syntax |

## Expressions (Non-Exhaustive)

- Literals: numbers, strings, booleans
- Identifiers and attribute access (`identifier`, `object.property`)
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `and`, `or`, `not`
- Collections: lists `[a, b]`, dicts `{ key: value }`
- Function calls: `function(args)`
- Lambda (planned): `lambda params: expression`

## Core Tokenization Rules (Disambiguation)

- Core constructs are recognized only at statement start (ignoring leading whitespace/comments) and never inside strings/comments.
- Transition `->` is core only when it begins a statement and the follow matches: optional exit args, optional enter args, optional string label, then `$State` with optional state params. Otherwise, `->` belongs to the target body.
- Parent forward is the exact statement `=> $^`.
- State headers with `$Child => $Parent` appear only in `machine:` headers; `$` occurrences inside bodies belong to the target.
- Enter/exit selectors `$>()` and `<$()` are parsed only in state bodies as handler selectors.

## Notes
- Actual parser grammar lives in `docs/framelang_design/grammar.md`; this summary is a quick reference for target implementers.
- Any extension (pragmas, target-specific syntax) must build on top of these constructs and preserve their semantics across languages.
