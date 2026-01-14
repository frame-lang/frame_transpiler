# Frame v4 Implementation Status

## Overview
Frame v4 is being built using a hybrid approach that leverages proven v3 state machine parsers while generating clean, runtime-free code. This document tracks the current implementation status.

## Implementation Approaches

### 1. V3 Direct Compiler (Default - 80% functional)
- **Status**: Working
- **Location**: `framec/src/frame_c/v4/v3_direct_compiler.rs`
- **Method**: Uses complete v3 compile_module pipeline with syntax conversion
- **Pros**: Most functional, leverages all v3 features
- **Cons**: Generates v3-style code with runtime dependencies

### 2. V4 State Machine Compiler (NEW - 60% functional)
- **Status**: Working for Python
- **Location**: `framec/src/frame_c/v4/v4_state_machine_compiler.rs`
- **Method**: Uses adapted v3 state machine parsers with v4 code generation
- **Pros**: Clean code without runtime, proper state machine parsing
- **Cons**: Needs more work on indentation and completeness
- **Usage**: `USE_V4_STATE_MACHINE=1 framec compile file.fpy -l python_3`

### 3. V3 Based Compiler (30% functional)
- **Status**: Partial
- **Location**: `framec/src/frame_c/v4/v3_based_compiler.rs`
- **Method**: Uses v3 parsers with custom code generation
- **Pros**: Leverages v3 parsing infrastructure
- **Cons**: Incomplete implementation

### 4. V4 Pure Implementation (10% functional)
- **Status**: Experimental
- **Location**: `framec/src/frame_c/v4/parser.rs`, `scanner.rs`, etc.
- **Method**: Complete rewrite with token-based parsing
- **Pros**: Clean slate design
- **Cons**: Reinvents wheel, very incomplete
- **Usage**: `USE_V4_PURE=1 framec compile file.fpy -l python_3`

## Key Components

### State Machine Parsers (Adapted from v3)
- ✅ `system_parser_v4.rs` - Parses @@system blocks with v4 syntax
- ✅ `module_partitioner_v4.rs` - Partitions modules into sections
- ✅ `machine_parser_v4.rs` - Parses state machine blocks
- ⏳ `body_closer_v4` - Not yet adapted, using v3 version

### V4 Syntax Support
- ✅ `@@target` pragma for target language specification
- ✅ `@@system` for system definition and instantiation
- ✅ `@@persist` annotation for persistence support
- ✅ Language-specific file extensions (.fpy, .frts, .frs)
- ✅ Native imports before @@system
- ✅ Native test code after @@system block

### Code Generation Features (Python)
- ✅ Class-based state machine without runtime
- ✅ State transitions via `_transition_to_State()` methods
- ✅ Enter/exit handlers
- ✅ Event dispatch via `_dispatch()` method
- ✅ Interface methods with system.return support
- ✅ Domain variable initialization
- ✅ Actions as methods
- ⏳ Operations support (not implemented)
- ⏳ Hierarchical state machines (not implemented)
- ⏳ State parameters (not implemented)
- ⏳ Stack operations (basic implementation)

### Known Issues
1. **Indentation**: Nested structures in actions lose proper indentation
2. **Operations**: Not yet implemented in v4 state machine compiler
3. **TypeScript/Rust**: Only Python generation implemented so far
4. **Validation**: V3 validator needs adaptation for v4
5. **State Parameters**: Not yet handled in state machine compiler

## Test Coverage
- ✅ Basic state machine with transitions
- ✅ Enter handlers ($>)
- ✅ Interface methods with dispatch
- ✅ Domain variables
- ✅ Actions with native code
- ✅ system.return handling
- ✅ Native imports and test code preservation
- ⏳ Exit handlers ($<)
- ⏳ Hierarchical states (=>)
- ⏳ Stack operations ($$[+], $$[-])
- ⏳ State parameters

## Next Steps
1. Fix indentation handling for nested structures in actions/operations
2. Implement TypeScript and Rust code generation
3. Add operations support
4. Implement hierarchical state machines
5. Add state parameter support
6. Create comprehensive test suite using frame-docker-runner
7. Adapt v3 validator for v4 validation rules
8. Complete v4 pure implementation as alternative

## Usage Examples

### Compile with v4 State Machine Compiler
```bash
# Use the new state machine-based compiler
USE_V4_STATE_MACHINE=1 ./target/release/framec compile test.fpy -l python_3 -o test.py

# Default (uses v3 direct compiler)
./target/release/framec compile test.fpy -l python_3 -o test.py
```

### Example .fpy File
```python
@@target python

import json
from datetime import datetime

@@persist @@system TrafficLight {
    interface:
        tick()
        getColor(): str
    
    machine:
        $Red {
            $>() {
                print("Entering Red")
            }
            tick() {
                -> $Green()
            }
            getColor() {
                system.return = "red"
            }
        }
        $Green {
            tick() {
                -> $Yellow()
            }
            getColor() {
                system.return = "green"
            }
        }
        $Yellow {
            tick() {
                -> $Red()
            }
            getColor() {
                system.return = "yellow"
            }
        }
    
    domain:
        tickCount = 0
}

# Native test code
def test():
    light = TrafficLight()
    assert light.getColor() == "red"
    light.tick()
    assert light.getColor() == "green"
```

## Architecture Decision
The v4 state machine compiler approach (using adapted v3 parsers) is proving to be the most pragmatic path forward. It provides:
- Proven parsing technology from v3
- Clean code generation without runtime dependencies
- Gradual migration path from v3 to v4
- Ability to reuse and adapt existing infrastructure

This hybrid approach balances the need for v4's "going native" philosophy while leveraging the battle-tested v3 parsing infrastructure.