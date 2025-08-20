# Frame Transpiler - Claude Context

## Project Overview

Frame is a state machine language that transpiles to multiple target languages. This project is currently migrating from v0.11 syntax to v0.20 syntax, which involves significant changes to make Frame more conventional while preserving its unique event-driven state machine capabilities.

## Current State

**Branch**: `v0.20`  
**Status**: Active v0.20 syntax migration - all updated syntax validated  
**Achievement**: 100% test coverage for implemented v0.20 features (57/57 files passing)  
**Recent**: Complete `=> $^` parent dispatch implementation with validation and auto-return statements

## Architecture

```
Frame Source (.frm) 
    ↓
Scanner (Tokenizer) → framec/src/frame_c/scanner.rs
    ↓  
Parser → framec/src/frame_c/parser.rs
    ↓
AST → framec/src/frame_c/ast.rs
    ↓
Visitors (Code Generation) → framec/src/frame_c/visitors/
    ↓
Target Code (Python, C#, etc.)
```

## Key v0.20 Syntax Changes

### System Declaration
- **Old**: `#SystemName ... ##`
- **New**: `system SystemName { ... }`

### Block Keywords
- **Old**: `-interface-`, `-machine-`, `-actions-`, `-domain-`
- **New**: `interface:`, `machine:`, `actions:`, `domain:`

### Parameters
- **Old**: `[param1, param2]`
- **New**: `(param1, param2)`

### Event Handlers
- **Old**: `|eventName|`
- **New**: `eventName()`

### Enter/Exit Events
- **Old**: `|>|` and `|<|`
- **New**: `$>()` and `<$()`

### Return Statements
- **Old**: `^` and `^(value)`
- **New**: `return` and `return value`

### Event Forwarding to Parent (NEW in v0.20)
- **Old**: `:>` (deprecated), `@:>` (terminator - deprecated)
- **New**: `=> $^` (statement - can appear anywhere in event handler)

### Attributes (NEW in v0.20)
- **Old**: `#[static]` (Rust-style)
- **New**: `@staticmethod` (Python-style)

### Current Event Reference (NEW in v0.20)
- **Old**: `@` for current event
- **New**: `$@` for current event
- **Note**: Single `@` now reserved for Python-style attributes

### System Parameters
- **Old**: `#System [$[start], >[enter], #[domain]]`
- **New**: `system System ($(start), $>(enter), domain)`

### System Instantiation
- **Old**: `System($("a"), >("b"), #("c"))`
- **New**: `System("a", "b", "c")` (flattened arguments)

## Build & Test

### Build
```bash
cargo build
```

### Test Transpiler
```bash
# Available languages: python_3, graphviz
./target/debug/framec -l python_3 file.frm
```

### Test Files Location
- `/Users/marktruluck/projects/test5/.vscode/v0.20_syntax/`
- Keep test files here, NOT in main transpiler project

## Code Conventions

### Scanner (scanner.rs)
- Token recognition in `scan_token()` method
- New tokens added to `TokenType` enum
- Use `peek()` and `peek_next()` for lookahead

### Parser (parser.rs)
- Event handler parsing in `event_handler()` method
- Terminator parsing handles `return`, `=>`, `@:>`
- Use `TerminatorType` enum for different terminators

### AST (ast.rs)
- All syntax tree node definitions
- `TerminatorType` enum defines terminator semantics

### Visitors
- Each target language has its own visitor
- All visitors must handle new `TerminatorType::DispatchToParentState`
- Python visitor is primary reference implementation

## Documentation Migration Status

Documentation is located in `/Users/marktruluck/projects/frame-docs/`

### Getting Started Documentation (✅ COMPLETED)
- `getting_started/basics.rst` - Basic Frame syntax and concepts
- `getting_started/system.rst` - System declarations and structure
- `getting_started/frame_events.rst` - Event handling fundamentals
- `getting_started/machine_block.rst` - State machine structure
- `getting_started/actions_block.rst` - Action method definitions
- `getting_started/domain_block.rst` - Domain variable declarations
- `getting_started/index.rst` - Getting started overview

### Intermediate Frame Documentation (🔄 IN PROGRESS)
**✅ Completed:**
- `intermediate_frame/hsm.rst` - Hierarchical state machines with `@:>` operator
- `intermediate_frame/interface.rst` - v0.20 interface syntax and parameter matching
- `intermediate_frame/intermediate_events.rst` - Enter/exit events with v0.20 syntax
- `intermediate_frame/systems.rst` - System parameter syntax migration
- `intermediate_frame/transitions.rst` - Transition syntax updates
- `intermediate_frame/conditionals.rst` - **NEW** v0.20 if/elif/else conventional syntax
- `intermediate_frame/loops_new.rst` - **NEW** v0.20 for/while loop conventional syntax

**⏳ Pending Updates:**
- `intermediate_frame/control_flow.rst` - Contains deprecated pattern matching syntax (?, ?!, :>, ?~, ?#, ?:)
- `intermediate_frame/loops.rst` - Contains legacy loop syntax, may be replaced by loops_new.rst
- `intermediate_frame/history.rst` - State history mechanisms
- `intermediate_frame/enums.rst` - Enumeration support
- `intermediate_frame/functions.rst` - Function definitions and calls
- `intermediate_frame/lists.rst` - List/array operations
- `intermediate_frame/return.rst` - Return statement migration (^ → return)
- `intermediate_frame/states.rst` - State definitions and behavior

### Advanced Frame Documentation (⏳ PENDING)
- `advanced_frame/state_variables.rst` - ✅ COMPLETED
- `advanced_frame/transitions.rst` - ✅ COMPLETED  
- `advanced_frame/transition_parameters.rst` - Transition parameter passing
- `advanced_frame/control_flow.rst` - Advanced control flow patterns
- `advanced_frame/compartments.rst` - State compartmentalization

### Transpiler Grammar Documentation (✅ COMPLETED)
- `docs/source/language/grammar.md` - Complete BNF grammar with v0.20 syntax, design decisions, and examples

### Documentation Notes
- **conditionals.rst** and **loops_new.rst** focus exclusively on v0.20 conventional syntax
- Pattern matching syntax (?, ?!, :>, ?~, ?#, ?:) will be deprecated and should not be included in new docs
- All system examples use correct block order: operations, interface, machine, actions, domain
- Legacy files like control_flow.rst and loops.rst contain extensive deprecated syntax

## Important Notes

### System Block Structure
- System blocks are optional but must appear in specified order:
  1. `operations:`
  2. `interface:`
  3. `machine:`
  4. `actions:`
  5. `domain:`
- Blocks can be omitted if not needed
- Order is enforced by parser

### Event Handler Terminators
- All event handlers MUST end with a terminator (`return`, `@:>`, `=>`)
- `@:>` forwards events to parent states in hierarchical state machines
- `@:>` is a block terminator - no statements can follow it
- Code generators must emit implicit return after `@:>` dispatch

### Hierarchical State Machines
- Use `$Child => $Parent` syntax for hierarchy
- `@:>` operator forwards events from child to parent
- Child processes event first, then forwards to parent
- Parent state handles forwarded event

### Parameter Validation
- Interface method parameters must exactly match event handler parameters
- Names and types must be identical
- System parameter order: start state, enter event, domain (flattened)

## Git Workflow

### Branches
- `main` - stable v0.11 syntax
- `v0.20` - active development branch

### Commit Style
- Use conventional commits
- Reference specific syntax changes
- Include rationale for design decisions

## Common Tasks

### Adding New Syntax
1. Update scanner.rs with new token recognition
2. Add token to TokenType enum
3. Update parser.rs to handle new syntax
4. Update AST if needed (new node types)
5. Update all visitors in visitors/ directory
6. Update grammar.md documentation
7. Create test cases in test5 project

### Testing Changes
1. Build with `cargo build`
2. Test with sample .frm files
3. Verify generated code compiles/runs
4. Check all visitors handle new syntax

## Recent Accomplishments (2025-01-20)

### Complete `=> $^` Parent Dispatch Implementation ✅
- **Achievement**: Full implementation of new parent dispatch syntax replacing deprecated `@:>`
- **Parser Enhancement**: Added validation to prevent `=> $^` in non-hierarchical states  
- **AST Updates**: Enhanced `ParentDispatchStmtNode` with parent state tracking
- **Code Generation**: Generates actual parent state calls with transition detection
- **Auto-Return**: Parser automatically adds return terminators to event handlers without explicit returns
- **Double Return Fix**: Resolved issue where both explicit and auto-generated returns were being created
- **Test Coverage**: Comprehensive test suite with 57/57 files passing validation
- **Documentation**: Updated all syntax documentation and examples

### Key Features Implemented ✅
- **Statement Syntax**: `=> $^` can appear anywhere in event handler (not just as terminator)
- **Parent Call**: Generates `self.parent_state(__e, compartment.parent_compartment)`
- **Transition Safety**: Code after `=> $^` doesn't execute if parent triggers transition
- **Validation**: Parser prevents invalid usage in non-hierarchical states
- **Compatibility**: Works in all event handler types including enter/exit handlers
- **Smart Returns**: Parser intelligently avoids double return generation

## Recent Accomplishments (2025-01-18)

### @ Symbol Refactoring for v0.20 ✅
- **Achievement**: Successfully refactored @ symbol usage for clearer semantics
- **Python Attributes**: Adopted `@staticmethod` and other Python decorators as standard
- **Current Event**: Changed from `@` to `$@` to align with Frame's $ prefix pattern
- **FrameEvents**: Reserved `@@` for FrameEvent markers
- **Implementation**: Updated scanner, parser, AST, and Python visitor
- **Documentation**: Updated all Frame documentation to reflect new syntax

### Static Operations Support ✅
- **Parser**: Now correctly recognizes `@staticmethod` attribute
- **Code Generation**: Python visitor generates proper `@staticmethod` decorator
- **Method Signature**: Static methods correctly omit `self` parameter
- **Validation**: All operations.rst examples now generate working Python code

## Recent Accomplishments (2025-01-17)

### Comprehensive v0.20 Syntax Validation ✅
- **Achievement**: 100% test coverage for implemented v0.20 features (56/56 files)
- **Parser Fixes**: Transition + return parsing, system parameters, legacy syntax updates
- **Quality Assurance**: All generated Python code passes syntax validation
- **Test Suite**: Now serves as comprehensive v0.20 syntax documentation
- **Regression Testing**: All existing functionality preserved

### Major Parser Improvements ✅
- **Return Statements**: Now work as regular statements in all contexts (if/elif/else, loops, etc.)
- **Return Assignment**: `return = expr` syntax for interface return values  
- **Transition Parsing**: Fixed `-> $State` followed by `return` in conditional blocks
- **System Parameters**: Correct v0.20 syntax with flattened instantiation arguments
- **Legacy Cleanup**: Updated all test files from v0.11 to v0.20 syntax

### Test File Modernization ✅  
- **Legacy Syntax**: ^ → return, :> → @:>, old system parameters → v0.20
- **Function Restrictions**: Enforced single main function, converted multiple functions to system actions
- **Syntax Patterns**: Updated for loops, parameter lists, block structures
- **Documentation Value**: Test files now serve as syntax examples

## Design Decisions Log

### `=> $^` Parent Dispatch (2025-01-20)
- **Decision**: Statement syntax (not terminator) replacing deprecated `@:>`
- **Rationale**: More flexible - can appear anywhere in event handler with statements after
- **Implementation**: Parser validates hierarchical context, AST tracks parent state, visitor generates parent call
- **Transition Safety**: Generated code checks for transitions after parent call and returns early if needed
- **Validation**: Parser error if used in non-hierarchical state

### v0.20 System Parameters
- **Decision**: Flattened argument lists for instantiation
- **Rationale**: Simpler, more conventional syntax
- **Migration**: `System($(a), $>(b), c)` → `System(a, b, c)`

### Interface Return Assignment (2025-01-17)
- **Decision**: Replace `^=` with `return = value` syntax
- **Rationale**: More conventional and readable syntax
- **Implementation**: Parser recognizes `return =` as interface return assignment
- **Migration**: `^= expr` → `return = expr`
- **Codegen**: Generates assignment to return stack/field in target language

## Files to Never Edit

- Test files in main transpiler project (use test5 instead)
- Legacy v0.11 documentation (keep for reference)
- Generated code files

## Current Status & Issues Found

### ✅ RESOLVED: if/elif/else Parsing in Event Handlers (2025-01-16)

**Issue**: Event handlers failed to parse if/elif/else chains with return statements, causing "Expected '}'" errors.

**Root Cause**: Frame's parser only supported `return` as event handler terminators, not as regular statements within blocks.

**Solution Implemented**:
1. Added `ReturnStmt` variant to `StatementType` enum (ast.rs:1689)
2. Created `ReturnStmtNode` AST node (ast.rs:3780-3794)
3. Added return statement parsing to `statement()` method (parser.rs:4652-4667)
4. Implemented visitor support in Python and GraphViz visitors
5. Added `visit_return_stmt_node` to AstVisitor trait

**Files Modified**:
- `framec/src/frame_c/ast.rs` - Added ReturnStmt AST node
- `framec/src/frame_c/parser.rs` - Added return statement parsing
- `framec/src/frame_c/visitors/mod.rs` - Added visitor method
- `framec/src/frame_c/visitors/python_visitor.rs` - Python code generation
- `framec/src/frame_c/visitors/graphviz_visitor.rs` - Pattern matching

**Test Results**:
- ✅ Event handlers now support if/elif/else chains with return statements
- ✅ Action methods continue to work as before
- ✅ Generated Python code is clean and conventional
- ✅ All test cases pass successfully

**Working Test Files**:
- `/Users/marktruluck/projects/test5/.vscode/v0.20_syntax/test_enum_basic.frm` - ✅ Works
- `/Users/marktruluck/projects/test5/.vscode/v0.20_syntax/test_enums_terminator.frm` - ✅ Works
- `/Users/marktruluck/projects/test5/.vscode/v0.20_syntax/test_enums.frm` - ✅ **NOW WORKS**
- `/Users/marktruluck/projects/test5/.vscode/v0.20_syntax/test_elif_with_return.frm` - ✅ Works
- `/Users/marktruluck/projects/test5/.vscode/v0.20_syntax/test_simple_elif.frm` - ✅ Works

### Known Issues & Future Optimizations

**1. Dead Code Generation (Low Priority)**
- **Issue**: Event handlers generate unreachable return statements after complete if/elif chains
- **Example**: Final `return` after exhaustive if/elif/else is unreachable
- **Status**: Functional correctness is fine, this is a code generation optimization
- **Solution**: Requires control flow analysis to detect exhaustive return coverage

### Documentation Status After Fix
- `intermediate_frame/enums.rst` - ✅ Now fully validated with working examples
- `intermediate_frame/conditionals.rst` - ✅ Examples now work in event handlers
- All if/elif/else documentation examples can now be validated

## Current Priorities

1. ✅ **COMPLETED**: if/elif/else parsing in event handlers - fixed with transition + return parsing
2. ✅ **COMPLETED**: Validate all implemented syntax with transpiler - 57/57 test files passing
3. ✅ **COMPLETED**: Update legacy syntax (^, :>, @:>, system parameters, multiple functions)
4. ✅ **COMPLETED**: Complete `=> $^` parent dispatch implementation with validation and double return fix
5. ✅ **COMPLETED**: Auto-return statements for event handlers without explicit returns
6. Continue intermediate Frame documentation migration for remaining features
7. Update remaining advanced Frame topics
8. Remove deprecated `^` and `@:>` token support (parser updated, need to clean up scanner)
9. Complete v0.20 syntax implementation for remaining features
10. (Future) Optimize dead code generation in event handlers

## Helpful Commands

```bash
# Check for old syntax in docs
grep -r ":>" docs/
grep -r "\^" docs/
grep -r "\|.*\|" docs/

# Find Frame files for testing
find . -name "*.frm"

# Build and test in one command
cargo build && ./target/debug/framec -l python_3 test_file.frm
```
- Always indent the code in the frame blocks (operations: interface: machine: etc) in the samples that are generated or updated.
- do not add attribution to claude on the commit messages