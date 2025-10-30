# Frame Common Grammar

## Purpose
This document summarizes the syntax every target must support before introducing language-specific features. It mirrors the shared AST produced by the Frame parser so target implementers know which constructs are universal.

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

## Statement Forms

- **Expression statement**: `expression`
- **Assignment**: `target = expression`
- **Transition**: `-> $State` or `-> $$[+]` / `-> $$[-]`
- **Forward**: `=> $^`
- **Return**: `return [expression]`
- **If / Else**: `if condition { ... } [else { ... }]`
- **While**: `while condition { ... }`
- **For-in**: `for var in iterable { ... }`
- **Try/Catch**: `try { ... } catch (Identifier) { ... }`
- **Raise**: `raise expression`
- **Async/Await**: `await expression`
- **Call**: `Callable(arguments)`

## Expressions (Non-Exhaustive)

- Literals: numbers, strings, booleans
- Identifiers and attribute access (`identifier`, `object.property`)
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `and`, `or`, `not`
- Collections: lists `[a, b]`, dicts `{ key: value }`
- Function calls: `function(args)`
- Lambda (planned): `lambda params: expression`

## Notes
- Actual parser grammar lives in `docs/framelang_design/grammar.md`; this summary is a quick reference for target implementers.
- Any extension (pragmas, target-specific syntax) must build on top of these constructs and preserve their semantics across languages.
