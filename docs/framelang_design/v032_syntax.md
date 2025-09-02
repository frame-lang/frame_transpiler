# Frame Language v0.32 Syntax Reference

## Overview

Frame is a state machine language that transpiles to multiple target languages. This document covers the current v0.32 syntax with advanced enum features, after the complete removal of legacy v0.11 features.

## Current Syntax (v0.32)

### System Declaration
- **Syntax**: `system SystemName { ... }`

### Block Keywords
- `interface:` - Interface methods
- `machine:` - State machine definition
- `actions:` - Action implementations
- `operations:` - Helper operations
- `domain:` - Domain variables

### Parameters
- **Syntax**: `(param1, param2)`

### Event Handlers
- **Method Events**: `eventName()`
- **Enter Event**: `$>()`
- **Exit Event**: `<$()`

### Return Statements
- **Simple Return**: `return`
- **Return with Value**: `return value`
- **Interface Return Assignment**: `return = value`

### Event Forwarding
- **To Parent State**: `=> $^` (statement - can appear anywhere in event handler)
- **Current Event Reference**: `$@`

### Attributes
- **Syntax**: `@staticmethod` (Python-style attributes only)

### System Parameters
- **Declaration**: `system System ($(start), $>(enter), domain)`
- **Instantiation**: `System("a", "b", "c")` (flattened arguments)

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

## v0.31 Breaking Changes - Complete Legacy Syntax Removal

### All v0.11 Syntax Removed
The following legacy syntax has been **completely removed** from the language and will cause compilation errors:

#### Removed Operators and Tokens
- **Return Operators**: `^` and `^(value)` → Use `return` or `return value`
- **Return Assignment**: `^=` → Use `return = value`
- **Ternary Operators**: `?`, `?!`, `?~`, `?#`, `?:` → Use if/elif/else statements
- **Test Terminators**: `:|` and `::` → No longer needed
- **Pattern Matching**: `~/` (string), `#/` (number), `:/` (enum) → Use if/elif/else with comparisons

#### Removed Syntax Constructs
- **Old System Declaration**: `#SystemName ... ##` → Use `system Name { }`
- **Old Block Markers**: `-interface-`, `-machine-`, etc. → Use `interface:`, `machine:`, etc.
- **Old Parameters**: `[param1, param2]` → Use `(param1, param2)`
- **Old Event Handlers**: `|eventName|` → Use `eventName()`
- **Old Attributes**: `#[attr]` → Use `@attr`

#### Migration Requirements
- All code using old syntax must be migrated before compilation
- Compiler provides clear error messages for deprecated syntax
- No backward compatibility mode available

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

## Hierarchical State Machines

- Use `$Child => $Parent` syntax for hierarchy
- `=> $^` operator forwards events from child to parent
- Child processes event first, then forwards to parent
- Parent state handles forwarded event

## v0.32 Enum Enhancements

### Enum Features (NEW in v0.32)
- **Custom Integer Values**: Explicit values with `= number` syntax
- **String Enums**: Type annotation with `: string` for string-valued enums  
- **Negative Values**: Support for negative numbers in enum values
- **Auto-Increment**: Automatic value assignment for unspecified values
- **Module-Level Enums**: Declare enums at module scope (outside systems)
- **Enum Iteration**: Use `for...in` loops to iterate over enum values

### Enum Declaration Syntax

#### Basic Integer Enum
```frame
enum Status {
    Pending      // 0
    Active       // 1
    Complete     // 2
}
```

#### Custom Integer Values
```frame
enum HttpStatus {
    Ok = 200
    Created = 201
    BadRequest = 400
    NotFound = 404
    ServerError = 500
}
```

#### String Enum
```frame
enum Environment : string {
    Development = "dev"
    Staging = "staging"  
    Production = "prod"
}
```

#### Mixed Values with Auto-Increment
```frame
enum Priority {
    Low = 1      // 1
    Medium = 5   // 5
    High         // 6 (auto-incremented from previous)
    Critical     // 7
}
```

### Enum Usage

#### Access Enum Values
```frame
var status = Status.Active
var env = Environment.Production
```

#### Enum Properties
```frame
print(status.name)   // "Active"
print(status.value)  // 1 (for int) or "prod" (for string)
```

#### Enum Iteration
```frame
for env in Environment {
    print(env.name + ": " + env.value)
}
```

#### Module-Level vs System-Level Enums
```frame
// Module-level enum (accessible everywhere)
enum GlobalStatus {
    Inactive
    Active
}

system Monitor {
    domain:
        // System-level enum (only accessible within system)
        enum LocalState {
            Init
            Ready
        }
}
```

## Example v0.32 Multi-Entity File with Enums

```frame
// Module-level enum
enum LogLevel : int {
    Debug = 0
    Info = 1
    Warning = 2
    Error = 3
}

fn main() {
    var logger = Logger()
    logger.log("Starting", LogLevel.Info)
    
    // Iterate enum
    for level in LogLevel {
        print("Level: " + level.name)
    }
}

system Logger {
    interface:
        log(msg: string, level: LogLevel)
        
    machine:
        $Ready {
            log(msg: string, level: LogLevel) {
                if level.value >= LogLevel.Warning.value {
                    print("[IMPORTANT] " + msg)
                } else {
                    print("[" + level.name + "] " + msg)
                }
                return
            }
        }
}