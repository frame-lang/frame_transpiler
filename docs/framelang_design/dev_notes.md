# Frame v0.31 Development Notes

## Development History

### 2025-01-31: Module Variables with Automatic Global Declarations

#### Overview
Implemented comprehensive support for module-level variables with automatic `global` declaration generation for Python target. This eliminates UnboundLocalError runtime errors and provides a clean, natural syntax for module variable access.

#### Key Features
- **Automatic Global Generation**: Transpiler detects when module variables are modified and generates `global` declarations
- **Function Support**: Works for all standalone functions that modify module variables  
- **System Support**: Also generates globals for system state methods
- **Conditional Imports**: Only generates imports (like `from enum import Enum`) when actually used
- **Shadowing Protection**: Prevents local variables from shadowing module variables (Python limitation)

#### Technical Implementation
- **Two-Pass Analysis**: First identifies local declarations, then detects module variable modifications
- **CallChainExprT Support**: Handles v0.30's assignment syntax properly
- **HashSet Tracking**: Uses `global_vars_in_function` and `required_imports` HashSets for efficient tracking
- **Shadowing Check in Parser**: Added semantic analysis check in `var_declaration` method (parser.rs:3325-3356)
  - Uses `arcanum.lookup` with `UnknownScope` to search entire scope chain
  - Only triggers error for `ModuleVariable` type symbols
  - Provides clear error message at transpilation time

#### Test Results
- **Success Rate**: Improved to 98.2% (162/165 tests passing)
- **New Test**: `test_module_scope_comprehensive.frm` validates all features
- **Fixed Tests**: Module variable tests now pass with proper global declarations

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor.rs`: Added global declaration generation logic
- `docs/source/language/grammar.md`: Documented module variable syntax and features
- `CLAUDE.md`: Updated with implementation details

### 2025-08-31: None Keyword Standardization

#### Overview
Standardized on `None` as the single null value keyword in Frame, completely removing support for `null` and `nil`. This aligns Frame with Python conventions and simplifies the language.

#### Changes Made
- **Scanner**: Removed `null` and `nil` from keywords map
- **TokenType**: Removed `Null` and `Nil` enum variants
- **Parser**: Removed deprecated keyword handling
- **All Visitors**: Updated to only recognize `None_` token
- **Documentation**: Updated all references to reflect None-only syntax

#### Migration Impact
- **Breaking Change**: Code using `null` or `nil` will no longer compile
- **Migration Path**: Replace all instances of `null` and `nil` with `None`
- **Error Behavior**: `null` and `nil` are now treated as undefined identifiers

### 2025-08-31: Scope Handling Implementation Complete

#### Overview
Completed all 7 phases of the comprehensive scope handling implementation plan, achieving proper LEGB (Local, Enclosing, Global, Built-in) scope resolution and full isolation between functions and systems.

#### Key Achievements
- **Parser Fix**: Added scope context checking before ActionCallExprNode creation
- **Function Isolation**: Functions cannot call system actions/operations
- **System Isolation**: Systems cannot access other systems' internals  
- **LEGB Resolution**: Proper symbol lookup order with shadowing support
- **Test Coverage**: 158/158 tests passing (100% success rate)

#### Technical Changes
1. **Parser (parser.rs:7818-7826)**:
   - Check `ScopeContext::Function` before creating ActionCallExprNode
   - Functions treat action calls as undeclared external calls
   
2. **Symbol Table**:
   - `legb_lookup()` method fully implemented
   - `is_symbol_accessible()` enforces scope boundaries
   - Proper ScopeContext tracking (Global/Function/System)

3. **Test Files Added**:
   - `test_scope_isolation.frm`: Validates function/system isolation
   - `test_legb_resolution.frm`: Tests LEGB lookup order

### 2025-08-31: System Return Semantics Design

#### Overview
Introduced `system.return` as a distinct concept from regular function returns, clarifying Frame's dual return semantic model. The system return is the value returned to the original interface caller, persisting through any depth of calls and state transitions.

#### Core Design Principles

1. **System Return** (`system.return`)
   - The value returned to the external caller of a system interface method
   - Can be set from anywhere during interface call execution
   - Persists through state transitions and nested calls
   - Last write wins - final value when interface call completes is what caller receives

2. **Regular Return** (`return value`)
   - Returns value to immediate caller
   - Used in actions and operations for internal call chains
   - Does not affect system.return

3. **Default Return Values** (`: type = default`)
   - Interface declarations: `validate() : bool = false` sets initial system.return
   - Event handlers: `validate() : bool = true {` overrides system.return default on entry
   - Actions/Operations: `helper() : int = -1 {` sets default return to caller (not system.return)

#### Context-Specific Rules

| Context | `: type = default` sets | `system.return = value` | `return value` | `return` |
|---------|-------------------------|-------------------------|----------------|----------|
| Interface | Initial system.return | N/A | N/A | N/A |
| Event Handler | system.return on entry | ✅ Can override | ❌ N/A | ✅ Exit handler |
| Action | Default return to caller | ✅ Must set explicitly | ✅ Return to caller | ✅ Exit action |
| Operation | Default return to caller | ❌ Compile error | ✅ Return to caller | ✅ Exit operation |

#### Why Operations Cannot Use system.return
Operations are static methods that can be called:
- Directly from outside: `Calculator.calculate(5)` - no interface context
- From functions: `fn main() { Calculator.calculate(5) }` - no system instance  
- From other operations: Pure functional composition

Since there's no guarantee they're called through an interface, `system.return` is meaningless and should be a compile-time error.

#### Example Usage
```frame
system Validator {
    interface:
        check() : bool = true      // Default: system.return = true
        
    machine:
        $Start {
            check() : bool = false {  // Override: system.return = false on entry
                processData()         // Call action
                if critical {
                    system.return = true  // Explicit override
                }
                return               // Exit with current system.return value
            }
        }
        
    actions:
        processData() : int = -1 {   // Default for action's return to caller
            if error {
                system.return = false // Explicitly set interface return
                return 0             // Return to event handler
            }
            return 1                // Return to event handler (system.return unchanged)
        }
        
    operations:
        @staticmethod
        validate(x: int) : int = 0 { // Default for operation's return
            // system.return = x     // ERROR: operations cannot use system.return
            if x > 0 {
                return x * 2
            }
            // Implicit return 0
        }
}
```

#### Grammar Changes Required
1. Add grammar rule for `system.return` as special compound identifier
2. Parse `: type = value` syntax in interface, event handler, action, and operation declarations
3. Add validation that operations cannot use `system.return`
4. Create SystemReturnNode AST node type

#### Implementation Status
- Design: ✅ Complete (2025-08-31)
- Parser: 🔄 In Progress
- AST: 🔄 In Progress
- Code Generation: 🔄 In Progress
- Tests: 📝 Planned

### 2025-01-31: Self Expression Support & Static Operation Validation

#### Self as Standalone Expression
- **Achievement**: The `self` keyword can now be used as a standalone expression, not just with dotted access
- **Issue**: Parser required `self.something` syntax, preventing use of bare `self` as function argument
- **Solution**: 
  - Modified `parse_self_context()` to allow standalone `self` when not followed by a dot
  - Creates special variable node representing the system instance
  - Updated Python visitor to handle standalone `self` correctly
- **Use Case**: Enables `jsonpickle.encode(self)` without backticks in persistence operations
- **Files Modified**: `framec/src/frame_c/parser.rs` (lines 9605-9619), `framec/src/frame_c/visitors/python_visitor.rs` (lines 562-570)

#### Static Operation Improvements
- **Achievement**: Operations are only static when explicitly declared with `@staticmethod`
- **Previous Behavior**: All operations were generated as static methods
- **New Behavior**: 
  - Operations without `@staticmethod` are instance methods with implicit `self` parameter
  - Operations with `@staticmethod` are static methods without `self` parameter
  - Static operations that use `self` trigger a parse error
- **Validation**: Parser checks `is_static_operation` flag and errors if `self` is used in static context
- **Error Message**: "Cannot use 'self' in a static operation (marked with @staticmethod)"
- **Files Modified**: `framec/src/frame_c/parser.rs` (lines 2668-2672, 9607-9611), `framec/src/frame_c/visitors/python_visitor.rs` (lines 3980-4000)

### 2025-01-30: Function-System Scope Interaction & Complete Multi-Entity Support

#### Function-Operation Integration Complete
- **Achievement**: Functions can now properly call system operations using correct static method syntax
- **Issue**: Functions calling operations generated as bare calls (`add(5, 3)`) instead of static method calls (`Utils.add(5, 3)`)
- **Solution**: 
  - Modified operations to generate as `@staticmethod` by default for external accessibility
  - Updated call generation logic to use `SystemName.operationName()` syntax when called from standalone functions
  - Fixed call chain handling to avoid double system name prefixes (`Utils.Utils.add` → `Utils.add`)
- **Frame Source Syntax**: Functions must use `Utils.add(5, 3)` syntax to call system operations
- **Generated Python**: Correctly produces static method calls with proper `@staticmethod` decorators
- **Files Modified**: `framec/src/frame_c/visitors/python_visitor.rs` (lines 3973-3978, 4471-4484, 4571-4578)

#### Complete Multi-Entity Architecture
- **Functions**: Multiple functions per module with any names, full system integration
- **Systems**: Multiple systems per module with proper isolation and cross-system calls
- **Operations**: Always public (static methods) - callable from functions and other systems
- **Actions**: Always private (instance methods with `_` prefix) - only callable within system
- **Interface Methods**: Always public (instance methods) - callable from external code

#### Test Success Rate Achievement
- **Before Function Fixes**: 95.2% success rate (139 passed, 7 failed)
- **After Function Fixes**: 97.3% success rate (142 passed, 4 failed)
- **After All v0.31 Fixes**: 100% success rate (153 passed, 0 failed)

### 2025-01-29: Native Import Statement Support

#### Import Statement Implementation
- **Achievement**: Frame v0.31 now supports native Python import statements without backticks
- **Syntax Types Supported**:
  - Simple imports: `import math`
  - Aliased imports: `import numpy as np`
  - From imports: `from typing import List, Dict`
  - Wildcard imports: `from collections import *`
- **Implementation**: Added ImportNode to AST with four import types
- **Parser**: New `parse_import_statement()` and `parse_from_import_statement()` methods
- **Scanner**: Added `Import`, `From`, `As` token types
- **Code Generation**: Direct pass-through to Python output
- **Files Modified**: `parser.rs`, `scanner.rs`, `ast.rs`, `python_visitor.rs`

### 2025-01-28: Scope Resolution & LEGB Implementation

#### Python LEGB Scope Resolution
- **Issue**: Frame was not properly implementing Python's LEGB (Local, Enclosing, Global, Built-in) scope resolution
- **Solution**: Modified symbol table and code generation to respect Python's scope rules
- **Key Changes**:
  - Local variables properly shadow outer scope variables
  - Built-in functions accessible without declaration
  - Module-level variables accessible with proper scoping
- **Impact**: Fixed multiple test failures related to variable shadowing and built-in access

### 2025-01-27: Hierarchical State Machine (HSM) Improvements

#### Parent Dispatch Router Integration
- **Achievement**: Parent dispatch now uses unified `__router` infrastructure
- **Previous Issue**: Hardcoded parent state method names caused maintenance issues
- **Solution**: Modified router signature to accept optional compartment parameter
- **Router Signature**: `__router(self, __e, compartment=None)`
- **Parent Dispatch**: `self.__router(__e, compartment.parent_compartment)`
- **Benefits**: Dynamic state resolution, no hardcoded names, single routing logic point

#### HSM Infinite Recursion Fix
- **Issue**: Parent dispatch caused infinite recursion due to improper compartment initialization
- **Root Cause**: Child compartments had `parent_compartment=None`
- **Solution**: Proper parent compartment references in hierarchical states
- **Generated Code**: `FrameCompartment('Child', ..., FrameCompartment('Parent', ...))`

### 2025-01-26: Multi-Entity Module Support

#### Module Architecture Redesign
- **Achievement**: Proper FrameModule container with peer Functions[] and Systems[]
- **Previous**: SystemNode-centric design with artificial parent-child relationships
- **New**: Functions and systems are peer entities within modules
- **Parser**: Sequential entity parsing supporting any combination
- **Symbol Table**: System-scoped state resolution with proper isolation

#### Call Chain Scope Processing Fix
- **Critical Bug**: External object method calls generated incorrect `obj.self.method()` syntax
- **Solution**: Conditional flag setting in call chain processing
- **Impact**: Properly distinguishes between external and internal call contexts

### 2025-01-25: State Stack Operations

#### State Stack Implementation
- **Operators**: `$$[+]` (push state), `$$[-]` (pop state)
- **Use Case**: History mechanisms and modal state preservation
- **Validation**: All state stack tests passing including complex nested sequences
- **Variable Preservation**: State variables maintain values through push/pop cycles

### 2025-01-24: Operations Block & Scope Handling

#### Operations vs Actions Clarification
- **Operations**: Public methods (can be static with `@staticmethod`)
- **Actions**: Private implementation methods (always instance methods with `_` prefix)
- **Scope Resolution**: Operations accessible externally, actions only within system
- **Static Validation**: Parse-time checking prevents `self` usage in static operations

## Release Notes Format

### Version Numbering
- v0.30: Multi-entity support, HSM improvements, state stack operations
- v0.31: Import statements, self expression, static validation, 100% test success
- v0.32: (Planned) System return semantics, default return values

### Test Success Tracking
- Track both transpilation success and execution success
- Document specific test fixes and their solutions
- Maintain test matrix in `framec_tests/reports/test_log.md`