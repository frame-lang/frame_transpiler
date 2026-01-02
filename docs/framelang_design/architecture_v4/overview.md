# Frame v4 Architecture Overview

## Core Philosophy: Native Code with Frame Structure

Frame v4 represents an evolution where native language code is used within Frame's structural framework. Frame provides the state machine structure and transitions, while the implementation uses native language syntax.

## Key Principles

1. **Native Code in Blocks**: Code blocks use target language's native syntax
2. **Frame Structure**: State machines, transitions, and system organization from Frame
3. **Native Imports**: Use target language's native import system
4. **Semantic Validation**: Compile-time enforcement of Frame architectural patterns
5. **Universal Debug Adapter**: Debugging happens through Frame-aware tooling

## What Frame v4 IS

- A state machine structural framework for any target language
- An architectural enforcement tool with semantic validation  
- A native-code system with Frame state machine constructs
- A foundation for universal state machine debugging

## What Frame v4 IS NOT

- A full general-purpose programming language
- A universal syntax language (uses native code in blocks)
- A complete type system (uses native types)
- A replacement for native language features

## Core Constructs

### 1. Frame Statements
- `-> $State(args)` - State transition
- `=> $^` - Event forwarding  
- `$$[+]` / `$$[-]` - Stack push/pop

### 2. Frame Annotations (all use @@ prefix)
- `@@target language` - Specifies target language for transpilation
- `@@system varname = SystemName(args)` - Declares and instantiates a Frame system
  - Enables semantic validation of all calls to `varname`
  - Enforces interface access control
  - Provides type safety for system interactions
- `@@persist` - Enables persistence support for a system
  - Generates save/restore methods
  - Supports system snapshots and restoration

### 3. Structural Keywords and Ordering
System blocks must appear in this canonical order when present:
- `system` - Defines a Frame system with state machine
- `operations:` - Internal helper methods (1st when present)
- `interface:` - Public methods (2nd when present)
- `machine:` - State definitions (3rd when present)
- `actions:` - Private implementation methods (4th when present)
- `domain:` - Private state variables (5th when present)

## File Extensions

**Important**: Frame v4 uses language-specific file extensions that clearly indicate the target language. Each extension maps to a specific target language, making the intent obvious for developers and tooling.

### Target-Specific Extensions
- `.fpy` - Python Frame files (typically with `@@target python` or `@@target python_3`)
- `.frts` - TypeScript Frame files (typically with `@@target typescript`)
- `.frs` - Rust Frame files (typically with `@@target rust`)
- `.fc` - C Frame files (typically with `@@target c`)
- `.fcpp` - C++ Frame files (typically with `@@target cpp`)
- `.fjava` - Java Frame files (typically with `@@target java`)
- `.frcs` - C# Frame files (typically with `@@target csharp`)

Each language-specific extension clearly indicates the intended target language. The `@@target` pragma remains required to maintain explicit declaration of compilation intent.

## Language Integration

### Python Example
```python
@@target python

# Native Python imports
import json
from datetime import datetime

system TrafficLight {
    operations:
        calculateDelay() {
            # Native Python code
            base_delay = 1000
            return base_delay * 2
        }
    
    interface:
        timer()
        getColor(): str
    
    machine:
        $Red {
            timer() {
                # Native Python within blocks
                message = "Transitioning to green"
                print(f"Status: {message}")  # Native Python f-string
                -> $Green()  # Frame transition statement
            }
            
            getColor() {
                return "red"
            }
        }
        
        $Green {
            timer() {
                -> $Yellow()
            }
            
            getColor() {
                return "green"
            }
        }
        
        $Yellow {
            timer() {
                -> $Red()
            }
            
            getColor() {
                return "yellow"
            }
        }
}

# Create a Frame-tracked system instance
@@system light = TrafficLight()

# Frame validates these calls
light.timer()        # Valid - in interface
color = light.getColor()  # Valid - in interface
# light.calculateDelay()  # ERROR - operations are private
```

### TypeScript Example
```typescript
@@target typescript

// Native TypeScript imports
import { Logger } from './logger';

system TrafficLight {
    operations:
        calculateDelay(): number {
            // Native TypeScript code
            const baseDelay = 1000;
            return baseDelay * 2;
        }
    
    interface:
        timer(): void
        getColor(): string
    
    machine:
        $Red {
            timer() {
                // Native TypeScript within blocks
                const message = "Transitioning to green";
                console.log(`Status: ${message}`);  // Native TS template literal
                -> $Green()  // Frame transition statement
            }
            
            getColor(): string {
                return "red";
            }
        }
        
        $Green {
            timer() {
                -> $Yellow()
            }
            
            getColor(): string {
                return "green";
            }
        }
        
        $Yellow {
            timer() {
                -> $Red()
            }
            
            getColor(): string {
                return "yellow";
            }
        }
}

// Create a Frame-tracked system instance
@@system light = TrafficLight()

// Frame validates these calls
light.timer();              // Valid - in interface
const color = light.getColor();  // Valid - in interface
// light.calculateDelay();  // ERROR - operations are private
```

## Key Features

### Native Language Integration
- **Imports**: Use native language import syntax
- **Code blocks**: Native language syntax for variables, functions, control flow
- **Implicit modules**: Files are implicitly modules without explicit declaration

### Frame Innovations
- `@@system` annotation for system instantiation
- Semantic validation of interface compliance
- Access control enforcement (public interface vs private implementation)
- Improved debugging through Frame Debug Adapter Protocol

## Complete Documentation

Frame v4 is fully documented with comprehensive guides:

### Core Concepts
- **[Quick Reference](quick_reference.md)** - Complete syntax cheat sheet
- [Native Features](native_features.md) - Abstract documentation of native language features
- [System Initialization](system_initialization.md) - System parameters and default initialization
- [Persistence](persistence.md) - System snapshots and state restoration
- [Return Semantics](return_semantics.md) - Contextual return behavior and system.return
- [State Machine Concepts](state_machine_concepts.md) - States, transitions, HSM, compartments
- [Language Features](language_features.md) - Types, control flow, collections, async/await
- [Grammar Specification](grammar.md) - Complete syntax reference
- [Semantic Validation](semantic_validation.md) - Compile-time validation rules
- [Architecture Clarifications](clarifications.md) - Design decisions and rationale

## Benefits

1. **Language Familiarity**: Developers write implementation code in their native language
2. **Native Integration**: Use native imports and external libraries seamlessly
3. **Architecture Enforcement**: Frame validates proper system usage at compile time
4. **Better Debugging**: Frame-aware debugging across all platforms
5. **Clear Boundaries**: Obvious separation between Frame systems and external code