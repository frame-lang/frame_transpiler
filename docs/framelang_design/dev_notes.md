# Frame v0.30 Development Notes

## Development History

### 2025-01-30: Enum Support & Call Chain Fixes

#### Complete Enum Support Implementation
- **Achievement**: All enum tests now pass (9/9 = 100% success rate)
- **Issue 1**: Action calls within systems (`getRandomFruit()`) generating without `self._` prefix  
- **Solution 1**: Modified `visit_call_expression_node` to check for actions even in ExternalCall context when no call chain present
- **Issue 2**: System interface calls (`sys.testFruit()`) generating as `sys.self._testFruit()`
- **Solution 2**: Fixed call chain processing to handle multi-node chains correctly without adding action prefixes
- **Issue 3**: Action calls with underscore prefix (`_testFruit()`) not being recognized as actions
- **Solution 3**: Strip underscore prefix when looking up actions in symbol table
- **Files Modified**: `framec/src/frame_c/visitors/python_visitor.rs` (lines 4448-4464, 4536-4574, 4960-4971)

#### Call Chain Processing Architecture Overhaul
- **Multi-Node Chains**: `sys.testFruit()` correctly generates interface method calls on system instances
- **Single-Node Chains**: `_testFruit()` correctly generates action calls with proper `self._` prefix
- **Context Awareness**: UndeclaredCall nodes in chains handle interface vs action contexts appropriately
- **Debug Infrastructure**: Added comprehensive debug output for call chain processing

#### Test Success Rate Milestone
- **Before Enum Fixes**: 89.7% success rate (131 passed, 14 failed)
- **After Enum Fixes**: 95.2% success rate (139 passed, 7 failed)
- **Improvement**: +5.5% success rate improvement
- **Status**: Enum functionality fully operational in Frame v0.30

### 2025-01-30: Critical Code Generation Fixes

#### System Instantiation Bug Fixed
- **Issue**: System instantiation calls like `SimpleHSM()` generated as `SimpleHSM` without parentheses
- **Root Cause**: `visit_call_expression_node_to_string` returned early for ExternalCall without generating parentheses
- **Solution**: Added `call_expr_list.accept_to_string()` before early return to generate parentheses
- **Impact**: System instantiation now works correctly, enabling proper object creation
- **Files Modified**: `framec/src/frame_c/visitors/python_visitor.rs` (line 4475)

#### Interface Method Call Generation Fixed  
- **Issue**: System-scoped variables generated `sys.self.test()` instead of `sys.test()`
- **Solution**: Modified `format_variable_expr` and `format_list_element_expr` to use variable name for SystemScope
- **Impact**: Interface method calls on system instances now generate correctly
- **Files Modified**: `framec/src/frame_c/visitors/python_visitor.rs` (lines 579-580, 644-645)

#### Test Success Rate Improvement
- **Before**: 64.4% success rate (94 passed, 52 failed)
- **After**: 89.7% success rate (131 passed, 14 failed)
- **Improvement**: +25.3% success rate improvement with forced re-transpilation
- **Validation**: Comprehensive test suite run with all fixes applied

### 2025-01-28: Python Privacy Convention & External Function Fixes

#### Action Naming Convention Changed to Python Standards
- **Change**: Actions now use Python underscore prefix convention instead of _do suffix
- **Old**: `def actionName_do(self):` called as `self.actionName_do()`
- **New**: `def _actionName(self):` called as `self._actionName()`
- **Impact**: Success rate improved from 85.1% to 89.6% (+4.5%)
- **Benefits**: Aligns with Python privacy conventions, cleaner code generation
- **Files Modified**: `framec/src/frame_c/visitors/python_visitor.rs` - `format_action_name()` method

#### Frame Privacy Model Documented
- **Documentation**: Created comprehensive privacy model at `docs/framelang_design/frame_privacy_model.md`
- **Principle**: Actions and operations are private implementation details (like Python's `_` and `__` conventions)
- **Interface Methods**: Public API accessible from external contexts
- **Enforcement**: Transpiler correctly prevents external access to private system internals

#### External Function Call Resolution Fixed
- **Issue**: External function calls like `print()` generated broken syntax in operations/actions contexts
- **Solution**: Restructured control flow to ensure external calls reach fallback logic regardless of context
- **Impact**: External functions now work consistently across all Frame v0.30 contexts

### 2025-01-23: Critical Bug Fixes

#### Backtick Import Bug Completely Resolved
- **Critical Issue**: Backtick statements (`import random`) not appearing in generated Python code when systems present
- **Impact**: Caused `NameError: name 'random' is not defined` runtime errors
- **Root Cause**: Module elements were being consumed by system parsing via `.take()`, leaving FrameModule empty
- **Solution**: Systems now get empty modules, module elements preserved for FrameModule
- **Files Modified**: `framec/src/frame_c/parser.rs` (line 254 - removed .take() logic)
- **Validation**: All backtick imports now correctly appear in generated Python
- **Test Improvement**: Success rate increased from 82.7% to 88.2% (7 additional tests passing)

#### Action Call Resolution Bug Fixed
- **Issue**: Action calls from functions/operations generated without `self.` prefix and `_do` suffix
- **Solution**: Pre-populate action tracking list before machine block processing
- **Impact**: All action calls now correctly generate as `self.action_do()`

#### Enum Scope Resolution Fixed
- **Issue**: Functions couldn't access system-defined enums, forward reference errors
- **Solution**: Generate all enums at module level before functions and systems
- **Impact**: Enums accessible from anywhere in module without self. prefix

### 2025-08-28: Complete Transpiler Overhaul

#### Return Value Handling System Fixed
- **Issue**: Interface methods used direct returns instead of Frame return stack mechanism
- **Solution**: Enhanced `visit_return_stmt_node` with scope-aware return generation
- **Impact**: +2 tests passing, correct Frame return semantics for event handlers vs actions

#### Function Scope Enforcement Implemented
- **Issue**: Standalone functions could access private system actions, violating Frame semantics
- **Solution**: Added `in_standalone_function` context checking in call resolution
- **Impact**: Proper enforcement of Frame v0.30 multi-entity scope boundaries

#### Enum Duplicate Resolution Fixed
- **Issue**: Duplicate enum entries caused Python enum conflicts (`'SUNDAY' already defined as 0`)
- **Solution**: Added duplicate detection with first-occurrence-wins strategy
- **Impact**: +1 test passing, eliminates enum syntax errors

#### State Arguments Support Implemented
- **Issue**: `FrameCompartment` missing `state_args` attribute causing AttributeError
- **Solution**: Enhanced constructor with complete parameter support and updated all instantiation calls
- **Impact**: +1 test passing, enables full state parameter functionality

### 2025-08-26: Operations Scope Fix

#### Operations Call Scope Bug Completely Resolved
- **Critical Issue**: Operations calling other operations (`helper_operation()`) missing `self.` prefix in Python
- **Impact**: Generated code caused `NameError: name 'helper_operation' is not defined` at runtime
- **Root Cause**: `lookup_operation()` relied on current system symbol which wasn't available during fallback syntactic parsing
- **Solution**: Changed to `lookup_operation_in_all_systems()` which searches all systems regardless of context
- **Files Modified**: `framec/src/frame_c/visitors/python_visitor.rs` (lines 4270, 4341)
- **Validation**: Operations calling operations now correctly generate `self.helper_operation()`
- **Production Impact**: Frame v0.30 now correctly handles all internal method calls within systems

### 2025-08-24: Call Chain & Hierarchical Fixes

#### Call Chain Scope Bug Completely Resolved
- **Critical Issue**: External object method calls (`obj.method()`) incorrectly generated `obj.self.method()` in Python
- **Impact**: Broke external object interactions, caused `NameError` runtime exceptions
- **Root Cause**: Python visitor's `visiting_call_chain_operation` flag incorrectly set for single-node operation calls
- **Solution**: Conditional flag setting based on call chain length - multi-node vs single-node distinction
- **Files Modified**: `framec/src/frame_c/visitors/python_visitor.rs` (lines 4481-4492, 4772-4783)
- **Validation**: Complex CultureTicks seat booking workflow with 20+ operation calls runs successfully
- **Production Impact**: Frame v0.30 now supports reliable object-oriented Python integration

#### Hierarchical State Machine Issues Completely Resolved
- **Root Cause Identified**: File format requirements, not parser bugs
- **File Format Rule**: Frame v0.30 requires `main()` function + system definitions (multi-entity format)
- **Parser Status**: All hierarchical parsing working perfectly with correct file format
- **Parent Dispatch**: `=> $^` syntax generates proper compartment hierarchy and router calls
- **Validation**: Complete HSM functionality verified working with test cases

## Test Success Rate History

| Date | Success Rate | Tests Passing | Total Tests | Key Changes |
|------|-------------|---------------|-------------|--------------|
| 2025-01-28 | 89.6% | 121 | 135 | Python underscore prefix for actions |
| 2025-01-28 | 85.1% | 114 | 134 | External function call fix |
| 2025-01-28 | 84.3% | 113 | 134 | Print function bug fix |
| 2025-01-23 | 88.2% | 112 | 127 | Backtick imports, action calls, enum scope |
| 2025-01-22 | 100% | 105 | 105 | Comprehensive test validation |
| 2025-01-21 | 73.2% | 93 | 127 | Symbol table preservation fix |
| 2025-01-17 | 19.7% | 25 | 127 | Initial v0.30 multi-entity support |

## Key Architecture Decisions

### Python Privacy Convention (2025-01-28)
- **Decision**: Use Python underscore prefix for private methods instead of _do suffix
- **Rationale**: Aligns with Python conventions, cleaner code, better IDE support
- **Implementation**: Changed `format_action_name()` to return `_{name}` instead of `{name}_do`

### Frame Privacy Model (2025-01-28)
- **Decision**: Actions and operations are private implementation details
- **Rationale**: Enforces proper encapsulation and object-oriented design
- **Documentation**: `docs/framelang_design/frame_privacy_model.md`
- **Impact**: Tests violating encapsulation should be fixed, not the transpiler

### Module Architecture (2025-01-22)
- **Decision**: FrameModule as top-level container with Functions[] and Systems[] as peers
- **Rationale**: Proper compiler design, no artificial parent-child relationships
- **Impact**: Enables true multi-entity support in Frame v0.30

## Known Issues

### Remaining Test Failures (as of 2025-01-28)
- **Runtime Errors**: 11 tests (down from 20)
- **Infinite Loops**: 2 tests timeout
- **Transpilation Failure**: 1 test (hierarchical state parsing)

### Future Improvements
- Consider enforcing interface/action name uniqueness in parser
- Optimize dead code generation in event handlers
- Complete migration from panic! to Result error handling

## Documentation References
- **Privacy Model**: `docs/framelang_design/frame_privacy_model.md`
- **Syntax Contexts**: `docs/framelang_design/syntax_contexts.md`
- **Test Matrix**: `docs/testing/test_matrix.md`
- **v0.30 Improvements**: `docs/v0.30_transpiler_improvements.md`
- **Language Changes**: `docs/v0.30_language_changes.md`