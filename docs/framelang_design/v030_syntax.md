# Frame Language v0.30 Syntax Reference

## Overview

Frame is a state machine language that transpiles to multiple target languages. This document covers the v0.30 syntax evolution from earlier versions.

## Syntax Evolution (v0.11 → v0.20 → v0.30)

### System Declaration
- **Old (v0.11)**: `#SystemName ... ##`
- **New (v0.20+)**: `system SystemName { ... }`

### Block Keywords
- **Old (v0.11)**: `-interface-`, `-machine-`, `-actions-`, `-domain-`
- **New (v0.20+)**: `interface:`, `machine:`, `actions:`, `domain:`

### Parameters
- **Old (v0.11)**: `[param1, param2]`
- **New (v0.20+)**: `(param1, param2)`

### Event Handlers
- **Old (v0.11)**: `|eventName|`
- **New (v0.20+)**: `eventName()`

### Enter/Exit Events
- **Old (v0.11)**: `|>|` and `|<|`
- **New (v0.20+)**: `$>()` and `<$()`

### Return Statements
- **Old (v0.11)**: `^` and `^(value)`
- **New (v0.20+)**: `return` and `return value`

### Event Forwarding to Parent
- **Old (v0.11)**: `:>` (deprecated), `@:>` (terminator - deprecated)
- **New (v0.20+)**: `=> $^` (statement - can appear anywhere in event handler)

### Attributes
- **Old (v0.11)**: `#[static]` (Rust-style)
- **New (v0.20+)**: `@staticmethod` (Python-style)

### Current Event Reference
- **Old (v0.20)**: `@` for current event
- **New (v0.20+)**: `$@` for current event
- **Note**: Single `@` now reserved for Python-style attributes

### System Parameters
- **Old (v0.11)**: `#System [$[start], >[enter], #[domain]]`
- **New (v0.20+)**: `system System ($(start), $>(enter), domain)`

### System Instantiation
- **Old (v0.11)**: `System($("a"), >("b"), #("c"))`
- **New (v0.20+)**: `System("a", "b", "c")` (flattened arguments)

## v0.30 Enhancements

### Multi-Entity Support (NEW in v0.30)
- **Multiple Functions**: Support for multiple functions with any names
- **Multiple Systems**: Support for multiple system definitions per file  
- **Module Architecture**: Foundation for comprehensive module system
- **Smart Parsing Fallback**: Automatic fallback to syntactic parsing when semantic analysis fails
- **System-Scoped State Resolution**: Proper isolation between multiple systems in same file

### File Format Requirements (v0.30)
- **Correct Format**: `fn main() { ... }` followed by `system SystemName { ... }`
- **Multi-Entity Required**: Functions and systems must coexist in proper multi-entity format
- **Legacy Support**: System-only files may cause parsing issues

### Deprecated Features (v0.30)
- **Conditional Testing**: `?`, `?!`, `?~`, `?#`, `?:` patterns deprecated
- **Migration Path**: Use modern if/elif/else statements instead
- **Error Messages**: Helpful deprecation warnings guide users to new syntax

## System Block Structure

System blocks are optional but must appear in specified order:
1. `operations:`
2. `interface:`
3. `machine:`
4. `actions:`
5. `domain:`

Blocks can be omitted if not needed. Order is enforced by parser.

## Event Handler Terminators

All event handlers MUST end with a terminator:
- `return` - Return from event handler
- `=> $^` - Forward event to parent state (hierarchical systems)
- `@:>` - Legacy parent forward (deprecated, use `=> $^`)

## Hierarchical State Machines

- Use `$Child => $Parent` syntax for hierarchy
- `=> $^` operator forwards events from child to parent
- Child processes event first, then forwards to parent
- Parent state handles forwarded event

## Example v0.30 Multi-Entity File

```frame
// Multi-entity format required in v0.30
fn main() {
    var first = FirstSystem()
    var second = SecondSystem()
    first.start()
    second.activate()
}

system FirstSystem {
    interface:
        start()
        
    machine:
        $Begin {
            start() {
                -> $Running
            }
        }
        
        $Running {
            $>() {
                print("First system running")
            }
        }
}

system SecondSystem {
    interface:
        activate()
        
    machine:
        $Idle {
            activate() {
                -> $Active  
            }
        }
        
        $Active {
            $>() {
                print("Second system active")
            }
        }
}
```