# Frame Transpiler - Claude Context

⚠️ **IMPORTANT: When starting a new session, ALWAYS read these documents first:**
1. This file (CLAUDE.md) - Project structure and conventions
2. `docs/framelang_design/dev_notes.md` - Latest development status
3. `docs/v0.32_achievements.md` - Current release features
4. `framec_tests/reports/test_matrix_v0.31.md` - Current test results

## Project Overview

Frame is a state machine language that transpiles to multiple target languages. The project has evolved through v0.20 (syntax modernization), v0.30 (multi-entity support), v0.31 (import statements and self expression enhancements), and v0.32 (advanced enum features).

## File Locations

### Test Files
- **CORRECT location for Frame test files**: `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/`
- **DO NOT** create test files in the project root directory
- All `.frm` test files must go in the framec_tests/python/src/ directory

## Test Validation

⚠️ **CRITICAL: ALWAYS USE THE OFFICIAL TEST RUNNER - NEVER CREATE ONE-OFF VALIDATION SCRIPTS** ⚠️

**Use the existing test runner at:** `framec_tests/runner/frame_test_runner.py`
- DO NOT create custom validation scripts like `validate_all_tests.sh`
- DO NOT write one-off test scripts
- ALWAYS use the official runner for ALL test validation needs

**Standard test validation command:**
```bash
cd framec_tests
python3 runner/frame_test_runner.py --all --matrix --json --verbose
```

## Current State

**Branch**: `v0.30`  
**Version**: `v0.32`  
**Status**: ✅ **100% TEST SUCCESS RATE** (170/170 tests passing)

📋 **For release notes and development status, see**: [`docs/framelang_design/dev_notes.md`](docs/framelang_design/dev_notes.md)
📊 **For v0.30 achievements, see**: [`docs/v0.30_achievements.md`](docs/v0.30_achievements.md)
📊 **For v0.31 achievements, see**: [`docs/v0.31_achievements.md`](docs/v0.31_achievements.md)
📊 **For v0.32 achievements, see**: [`docs/v0.32_achievements.md`](docs/v0.32_achievements.md)
📊 **For latest test results, see**: [`framec_tests/reports/test_matrix_v0.31.md`](framec_tests/reports/test_matrix_v0.31.md)

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

### v0.32 Latest Features (NEW)

#### Advanced Enum Support (NEW in v0.32)
- **Custom Integer Values**: `enum Status { Ok = 200, NotFound = 404 }`
- **String Enums**: `enum Color : string { Red = "red", Blue = "blue" }`
- **Auto String Values**: `enum LogLevel : string { Debug, Info }` → Debug="Debug", Info="Info"
- **Mixed Values**: Explicit values with auto-increment continuation
- **Negative Values**: `enum Priority { Low = -1, High = 10 }`
- **Module-Scope Enums**: Enums can be declared at module level (outside systems)
- **Enum Iteration**: `for status in StatusEnum { ... }`
- **Property Access**: `.name` and `.value` properties on enum members

### v0.31 Features

#### Import Statements (NEW in v0.31)
- **Simple imports**: `import math`
- **Aliased imports**: `import numpy as np`
- **From imports**: `from collections import defaultdict`
- **Wildcard imports**: `from typing import *`

#### Self Expression (NEW in v0.31)
- **Standalone self**: Can use `self` as complete expression
- **Example**: `jsonpickle.encode(self)` works without backticks

#### Static Methods (FIXED in v0.31)
- **Operations default**: Instance methods by default
- **Static declaration**: Use `@staticmethod` for static operations
- **Validation**: Parser validates no `self` in static operations

#### Null Value Standardization (v0.31)
- **Standard**: `None` is the only keyword for null values
- **Removed**: `null` and `nil` are no longer supported (breaking change)

#### Module Variables with Auto-Global Generation (v0.31)
- **Declaration**: `var counter = 0` at module level
- **Auto-Global**: Transpiler automatically generates `global` declarations for modified module variables
- **Functions**: Global declarations added when module variables are modified in functions
- **Systems**: Global declarations also generated for system state methods
- **Shadowing Protection**: Local variables cannot shadow module variables (Python target)
- **Conditional Imports**: Only generates imports (e.g., `from enum import Enum`) when actually used

### v0.30 Modular AST Structure

```
FrameModule (Top-Level)
├── Module (metadata/attributes)
├── Functions[] (peer entities)
├── Systems[] (peer entities)
├── Enums[] (module-level enums) - v0.32
└── Variables[] (module variables)
    └── SystemNode
        ├── Module (system-specific metadata)
        ├── Interface Block
        ├── Machine Block  
        ├── Actions Block
        ├── Operations Block
        └── Domain Block (can contain system-scoped enums)
```

## Frame Syntax (Current v0.32)

### Core Language Features

#### System Declaration
- **Syntax**: `system SystemName { ... }`

#### Block Keywords
- `interface:` - Interface methods
- `machine:` - State machine definition  
- `actions:` - Action implementations
- `operations:` - Helper operations
- `domain:` - Domain variables

#### Parameters
- **Syntax**: `(param1, param2)`

#### Event Handlers
- **Syntax**: `eventName()`
- **Enter Event**: `$>()`
- **Exit Event**: `<$()`

#### Return Statements
- **Simple**: `return`
- **With Value**: `return value`
- **Interface Return**: `return = value`

#### Event Forwarding
- **To Parent State**: `=> $^` (statement - can appear anywhere in event handler)
- **Current Event**: `$@`

#### Attributes
- **Syntax**: `@staticmethod` (Python-style)

#### System Parameters
- **Declaration**: `system System ($(start), $>(enter), domain)`
- **Instantiation**: `System("a", "b", "c")` (flattened arguments)

### v0.30 Enhancements

#### Multi-Entity Support
- **Multiple Functions**: Support for multiple functions with any names
- **Multiple Systems**: Support for multiple system definitions per file
- **Module Architecture**: Foundation for comprehensive module system

### v0.31 Breaking Changes

#### Completely Removed Legacy Syntax
The following v0.11 syntax has been **completely removed** and will cause compilation errors:

##### Removed Tokens
- `^` and `^(value)` - Old return syntax → Use `return` or `return value`
- `^=` - Old return assignment → Use `return = value`
- `#SystemName ... ##` - Old system declaration → Use `system Name { }`
- `?`, `?!`, `?~`, `?#`, `?:` - Ternary operators → Use if/elif/else
- `:|` and `::` - Test terminators → No longer needed
- `~/`, `#/`, `:/` - Pattern matching → Use if/elif/else with comparisons
- `#[attr]` - Rust-style attributes → Use `@attr`
- `[params]` - Bracket parameters → Use `(params)`
- `|event|` - Pipe event handlers → Use `event()`
- `-block-` - Dash block markers → Use `block:`

##### Migration Required
All code using old syntax must be migrated to modern syntax before compilation.

### v0.31 Enhancements

#### Module Variables (NEW in v0.31)
- **Module-level Variables**: Declare variables at module scope accessible from all functions/systems
- **Automatic Global Generation**: Transpiler detects modifications and generates `global` declarations for Python
- **Shadowing Protection**: Local variables cannot shadow module variables (enforced at transpilation)
- **Conditional Imports**: Only generates imports when actually used (e.g., `from enum import Enum`)
- **Two-Pass Analysis**: First identifies locals, then detects module variable modifications
- **System Support**: Works in both functions and system state methods

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

## Test Infrastructure

📚 **READ THE COMPLETE TESTING GUIDE**: [`framec_tests/docs/test_runner_guide.md`](framec_tests/docs/test_runner_guide.md)

### Test Organization
```
framec_tests/
├── runner/                    # Test execution infrastructure
│   ├── frame_test_runner.py   # Main test runner script
│   └── configs/               # Test configuration files
│       ├── all_tests.json    # Complete test suite
│       ├── hsm_tests.json    # Hierarchical state machines
│       ├── multi_entity_tests.json  # Multi-system/function tests
│       └── scope_tests.json  # Scope resolution tests
├── reports/                   # Test results and matrices
│   ├── test_matrix_v031.md   # Latest detailed test matrix
│   ├── test_results_v031.json # Machine-readable results
│   └── test_log.md           # Standard test status report
├── docs/                      # Test documentation
│   └── test_runner_guide.md  # Complete testing guide
└── python/
    ├── src/                   # Frame test files (.frm)
    ├── models/                # Expected output models
    └── scripts/               # Legacy helper scripts
```

**Key Directories:**
- **`runner/`**: Contains the official test runner and all configuration files for different test suites
- **`reports/`**: Stores all test results, matrices, and status reports - critical for tracking project health  
- **`docs/`**: Complete documentation including the comprehensive test runner guide

### Running Tests

⚠️ **ALWAYS READ THE TESTING GUIDE FIRST**: See [`framec_tests/docs/test_runner_guide.md`](framec_tests/docs/test_runner_guide.md) for complete details on usage, configuration options, and output formats.

#### Standard Test Validation Process
```bash
cd framec_tests
# Run all tests with matrix generation and JSON output
python3 runner/frame_test_runner.py --all --matrix --json --verbose

# After running, always check:
# 1. Test matrix saved to: reports/test_matrix_v0.31.md
# 2. JSON results saved to: reports/test_results_v0.31.json
```

#### Test Reporting Requirements
After EVERY test run, you MUST:
1. **Run the test suite** with `--matrix --json` flags
2. **Update the standard test log** at `reports/test_log.md` with:
   - Last run date/time
   - Total tests, passed, failed, success rate
   - Summary of passing categories
   - Table of failed tests with issue type
   - Any recent fixes applied
3. **Keep these files updated**:
   - `reports/test_log.md` - Main test status report (always overwrite)
   - `reports/test_matrix_v0.31.md` - Detailed test matrix (auto-generated)
   - `reports/test_results_v0.31.json` - JSON results (auto-generated)
4. **Categorize failures** as:
   - Environment issues (missing dependencies)
   - Test design issues (infinite loops, etc.)
   - Actual transpiler bugs
   - Expected failures (error validation tests)

### Test Files Location
**ALWAYS PUT TEST FILES HERE:**
- `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/` - ALL Frame test files (.frm) go here
- **Generated Python files (.py)**: Generated next to source files in `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/`
- NEVER put test files in the main project directory

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

## Known Limitations & Issues

### Module-Level Statements Not Supported
- **No module-level variables**: Cannot declare variables at module scope
- **No bare instantiations**: `SystemName()` at module level won't generate code
- **Workaround**: Always use a `main()` function for program entry point

### Current Test Failures (v0.31)
- `test_controlled_hsm_loop.frm` - Transpilation error
- `test_functions_simple.frm` - Runtime error
- `test_import_statements.frm` - Syntax error in generated code
- `test_legb_scope_resolution.frm` - Runtime error
- `test_single_system_transitions.frm` - Timeout during execution
- `test_static_self_error.frm` - Expected error test

## Important Notes for Development

### Code Style
- Always indent the code in the frame blocks (operations: interface: machine: etc)
- Do not add attribution to claude on the commit messages
- DO NOT add comments to generated code unless explicitly requested

### Testing Requirements
- **Generation != Validation**: Generating code is not the same as validating it works
- **Full validation** means: 
  1. Generate the Python code from .frm file
  2. RUN the generated Python code 
  3. Verify output matches expected behavior
  4. Report specific functionality verified
- Use the test runner for comprehensive testing
- Put transient documents in `docs/tmp/`

### Debug Output
- Debug output goes to stderr (eprintln! in Rust)
- Use `FRAME_TRANSPILER_DEBUG=1` environment variable to enable debug output
- Never send debug output to stdout (it contaminates generated code)