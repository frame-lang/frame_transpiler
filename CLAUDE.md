# Frame Transpiler - Claude Context

## Project Overview

Frame is a state machine language that transpiles to multiple target languages. This project has completed the v0.20 syntax migration and is now working on v0.30 enhancements, including multi-entity support and deprecated feature cleanup while preserving its unique event-driven state machine capabilities.

## File Locations

### Test Files
- **CORRECT location for Frame test files**: `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/`
- **DO NOT** create test files in the project root directory
- All `.frm` test files must go in the framec_tests/python/src/ directory

## Current State

**Branch**: `v0.30`  
**Status**: ✅ **89.7% TEST SUCCESS RATE** (131/146 tests passing)

📋 **For release notes and development status, see**: [`docs/framelang_design/dev_notes.md`](docs/framelang_design/dev_notes.md)
📊 **For v0.30 achievements summary, see**: [`docs/v0.30_achievements.md`](docs/v0.30_achievements.md)

## Architecture

```
Frame Source (.frm) 
    ↓
Scanner (Tokenizer) → framec/src/frame_c/scanner.rs
    ↓  
Parser → framec/src/frame_c/parser.rs
    ↓
AST (FrameModule) → framec/src/frame_c/ast.rs
    ↓
Visitors (Code Generation) → framec/src/frame_c/visitors/
    ↓
Target Code (Python, C#, etc.)
```

### v0.30 Modular AST Structure

```
FrameModule (Top-Level)
├── Module (metadata/attributes)
├── Functions[] (peer entities)
└── Systems[] (peer entities)
    └── SystemNode
        ├── Module (system-specific metadata)
        ├── Interface Block
        ├── Machine Block  
        ├── Actions Block
        ├── Operations Block
        └── Domain Block
```

## Frame Syntax Evolution (v0.11 → v0.20 → v0.30)

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

### v0.30 Enhancements

#### Multi-Entity Support (NEW in v0.30)
- **Multiple Functions**: Support for multiple functions with any names
- **Multiple Systems**: Support for multiple system definitions per file
- **Module Architecture**: Foundation for comprehensive module system

#### Deprecated Features (v0.30)
- **Conditional Testing**: `?`, `?!`, `?~`, `?#`, `?:` patterns deprecated
- **Migration Path**: Use modern if/elif/else statements instead
- **Error Messages**: Helpful deprecation warnings guide users to new syntax

## Build & Test

### Build
```bash
cargo build
```

### Test Transpiler

**IMPORTANT: GENERATION LOCATION**  
⚠️ **Generate Python files in the SAME directory as the source .frm file for easy location.**
- When transpiling `framec_tests/python/src/test.frm`, generate to `framec_tests/python/src/test.py`
- DO NOT use the `generated/` directory - generate right next to the source file

**CRITICAL: PROPER TEST VALIDATION PROTOCOL**

When claiming tests are "passing" or "working", you MUST follow this 4-step validation process:

1. **Generate**: Run framec to generate code IN THE SAME DIRECTORY as the source
2. **Execute**: Run the generated Python/target code 
3. **Validate**: Verify the output matches expected behavior
4. **Report**: State specifically what functionality was verified

**DO NOT claim "passing", "working", "100% success", or "complete validation" unless all 4 steps are completed.**

#### Example Proper Test Validation:
```bash
# Step 1: Generate (to same directory as source)
./target/debug/framec -l python_3 framec_tests/python/src/test.frm > framec_tests/python/src/test.py

# Step 2: Execute  
python3 framec_tests/python/src/test.py

# Step 3: Validate output
# Expected: "NoParameters started"
# Actual: "NoParameters started" ✅

# Step 4: Report
# VERIFIED: System initialization, enter event handling, print statement execution
```

#### Quick Generation Only (for syntax checking):
```bash
# Available languages: python_3, graphviz
./target/debug/framec -l python_3 file.frm
```

**Note**: Generation-only checks are useful for syntax validation but cannot be called "passing tests".

### Test Files Location
**ALWAYS PUT TEST FILES HERE:**
- `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/` - ALL Frame test files (.frm) go here
- **Generated Python files (.py)**: Generated next to source files in `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/`
- NEVER put test files in the main project directory
- NEVER use test5 directory - it's deprecated
- **Note**: The `/generated/` folder has been removed - all transpiled output goes directly to the `src/` directory

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

## Design Decisions Log

### `=> $^` Parent Dispatch (2025-01-20)
- **Decision**: Statement syntax (not terminator) replacing deprecated `@:>`
- **Rationale**: More flexible - can appear anywhere in event handler with statements after
- **Implementation**: Parser validates hierarchical context, AST tracks parent state, visitor generates parent call
- **Transition Safety**: Generated code checks for transitions after parent call and returns early if needed
- **Validation**: Parser error if used in non-hierarchical state

### Router-Based Parent Dispatch Architecture (2025-01-20)
- **Decision**: Use existing `__router` infrastructure for all parent dispatch instead of hardcoded method names
- **Rationale**: Maintains architectural consistency, eliminates code duplication, easier maintenance
- **Implementation**: 
  - Modified router signature: `__router(self, __e, compartment=None)`
  - Parent dispatch: `self.__router(__e, compartment.parent_compartment)`
  - Fallback dispatch also uses router for consistency
- **Benefits**: Dynamic state resolution, no hardcoded names, single point of routing logic
- **Compatibility**: Preserves all existing functionality while improving code quality

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

## Notes
- Always indent the code in the frame blocks (operations: interface: machine: etc) in the samples that are generated or updated.
- Do not add attribution to claude on the commit messages
- Full validation or comprehensive testing means running every test
- Put transient documents we work on like reports and findings in docs/tmp