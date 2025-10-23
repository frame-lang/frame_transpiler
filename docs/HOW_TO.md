# Frame Transpiler - Complete Development Guide

**🚨 MANDATORY READING FOR ALL AI SESSIONS 🚨**

This document captures every process, tool, and workflow used in the Frame Transpiler project. All AI assistants working on this project MUST read and follow these guidelines.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Development Environment](#development-environment)
4. [Testing Framework](#testing-framework)
5. [Version Management](#version-management)
6. [Code Patterns](#code-patterns)
7. [Bug Fixing Process](#bug-fixing-process)
8. [File Organization](#file-organization)
9. [Common Commands](#common-commands)
10. [Critical Rules](#critical-rules)

## Project Overview

Frame is a state machine language that transpiles to multiple target languages (Python, TypeScript, C#, etc.). The project is currently in v0.86.0 and migrating from v0.11 to v0.20 syntax.

### Current Status
- **Version**: v0.86.16
- **Branch**: `dev`
- **Test Success Rate**: 82.5% TypeScript overall execution success (887 total tests: 458 Python 100% + 429 TypeScript 82.5%)
- **Supported Targets**: Python 3, TypeScript (with runtime library), GraphViz
- **Recent Achievement**: Debugger-controller readiness plan with improved TypeScript execution coverage

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
Target Code (Python, TypeScript, etc.)
```

### Core Components

1. **Scanner (`scanner.rs`)**: Tokenizes Frame source code
2. **Parser (`parser.rs`)**: Builds AST from tokens using recursive descent parsing
3. **AST (`ast.rs`)**: Abstract syntax tree definitions
4. **Visitors**: Code generators for each target language
5. **Symbol Table (`symbol_table.rs`)**: Manages scoping and symbol resolution
6. **Compiler (`compiler.rs`)**: Orchestrates the compilation pipeline

## Development Environment

### Required Tools
- **Rust**: Latest stable version
- **Python 3**: For test execution and validation
- **Node.js + TypeScript**: For TypeScript target validation
- **Git**: Version control

### Build Commands
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Clean rebuild
cargo clean && cargo build --release

# Update dependencies
cargo update
```

## Testing Framework

### Test Organization
```
framec_tests/
├── common/tests/           # Cross-language test specifications (.frm files)
├── language_specific/      # Language-specific tests and external API tests
│   ├── python/            # Python-specific Frame tests
│   │   └── external_apis/ # Python external API integration tests
│   └── typescript/        # TypeScript-specific Frame tests
│       └── external_apis/ # TypeScript external API integration tests
├── generated/python/       # Generated Python code
├── generated/typescript/   # Generated TypeScript code
├── runner/                 # Test runner framework (frame_test_runner.py)
└── configs/               # Test configuration files
```

### Running Tests

**Comprehensive Test Suite:**
```bash
# All tests (both languages, all categories including language-specific)
python3 framec_tests/runner/frame_test_runner.py --languages python typescript --framec ./target/release/framec

# All Python tests (including Python-specific external API tests)
python3 framec_tests/runner/frame_test_runner.py --languages python --framec ./target/release/framec

# All TypeScript tests (including TypeScript-specific external API tests)
python3 framec_tests/runner/frame_test_runner.py --languages typescript --framec ./target/release/framec

# Specific categories (common tests only)
python3 framec_tests/runner/frame_test_runner.py --languages python --categories regression --framec ./target/release/framec

# Language-specific tests only
python3 framec_tests/runner/frame_test_runner.py --languages python --categories language_specific_python --framec ./target/release/framec

# Verbose output with batch TypeScript compilation
python3 framec_tests/runner/frame_test_runner.py --languages typescript --framec ./target/release/framec --verbose

# Transpile-only mode (no execution)
python3 framec_tests/runner/frame_test_runner.py --languages typescript --framec ./target/release/framec --transpile-only
```

**Single File Testing:**
```bash
# Python target
./target/release/framec -l python_3 path/to/test.frm

# TypeScript target
./target/release/framec -l typescript path/to/test.frm

# GraphViz target
./target/release/framec -l graphviz path/to/test.frm
```

### Test Categories

**Common Tests (Cross-Language):**
- **systems**: Core state machine functionality (200+ tests)
- **data_types**: Collections, dictionaries, lists (66+ tests)
- **control_flow**: Conditionals, loops, multifile (49+ tests)
- **scoping**: Variable scoping and resolution (45+ tests)
- **core**: State management and transitions (31+ tests)
- **operators**: Language operators and expressions (16+ tests)
- **negative**: Error handling and validation (13+ tests)
- **regression**: Bug prevention tests (7+ tests)

**Language-Specific Tests:**
- **language_specific_python**: Python-specific features and external APIs
- **language_specific_typescript**: TypeScript-specific features and external APIs

**External API Test Structure:**
- Language-specific external API tests demonstrate proper Frame integration with:
  - File I/O operations (Python: `os.path`, `open()`; TypeScript: `fs` module)
  - Process control (Python: `subprocess`; TypeScript: `child_process`)
  - Network operations (Python: `socket`; TypeScript: `net`)
  - Platform-specific libraries and frameworks

## Version Management

### Single Source of Truth
- Root `Cargo.toml` contains `[workspace.package]` and is the authoritative version.
- Member crates (`framec`, `frame_build`, `frame_runtime`) inherit that value via `version.workspace = true`; no per-crate edits are required.
- Build-time constants use `env!("FRAME_VERSION")`, which the build script maps directly to `CARGO_PKG_VERSION`.

### Semantic Versioning Rules
- **Bug fixes**: Increment patch version (0.85.1 → 0.85.2)
- **Minor features**: Increment minor version (0.85.x → 0.86.0)
- **Major changes**: Only the project owner declares major version bumps

### Version Update Process
```bash
# 1. Edit the workspace version in Cargo.toml
# 2. Sync auxiliary metadata (version.toml)
./scripts/sync-versions.sh

# 3. Rebuild to pick up the new version
cargo build --release

# 4. Verify version in output
./target/release/framec --version
./target/release/framec -l python_3 test.frm | head -3  # Check header comment
```

## Code Patterns

### Parser Patterns
```rust
// Event handler parsing with proper token synchronization
if !self.check(TokenType::Identifier) && 
   !self.check(TokenType::EnterStateMsg) && 
   !self.check(TokenType::ExitStateMsg) {
    break;
}
// Use check() instead of match_token() to avoid consuming tokens
```

### Error Handling
```rust
// Always provide context in error messages
let run_error = RunError::new(frame_exitcode::PARSE_ERR, &format!(
    "Parser error at line {}: {}", line_number, error_details
));
```

### Symbol Table Usage
```rust
// Two-pass parsing architecture
// Pass 1: Build symbol table (is_building_symbol_table = true)
// Pass 2: Semantic analysis (is_building_symbol_table = false)
```

## Bug Fixing Process

### 1. Reproduction
- Create minimal test case in `framec_tests/common/tests/regression/`
- Use descriptive naming: `test_bug[NUMBER]_[description].frm`
- Document expected vs actual behavior

### 2. Root Cause Analysis
- Use `FRAME_TRANSPILER_DEBUG=1` for verbose debugging
- Check scanner vs parser issues
- Verify token synchronization in parser loops

### 3. Implementation
- Fix the core issue (never implement workarounds)
- Update all affected visitors if AST changes
- Maintain backward compatibility

### 4. Validation
- Ensure regression test passes
- Run full test suite (must maintain 100% success rate)
- Test across all target languages
- Update version number appropriately

### 5. Documentation
- Update relevant documentation
- Add comments explaining the fix
- Update CLAUDE.md if process changes

## File Organization

### Critical Files to Never Edit
- Generated test files in `framec_tests/generated/`
- Legacy documentation (keep for reference only)
- Main project test files (use proper test framework)

### Test File Locations
- **Frame Specifications**: `framec_tests/common/tests/[category]/`
- **Generated Python**: `framec_tests/generated/python/`
- **Generated TypeScript**: `framec_tests/generated/typescript/`
- **Test Runner**: `framec_tests/runner/frame_test_runner.py`

### Source Code Structure
```
framec/src/frame_c/
├── scanner.rs           # Tokenization
├── parser.rs           # Recursive descent parser
├── ast.rs              # AST node definitions
├── compiler.rs         # Main compilation orchestration
├── symbol_table.rs     # Symbol resolution
├── visitors/           # Code generators
│   ├── python_visitor_v2.rs
│   ├── typescript_visitor.rs
│   └── graphviz_visitor.rs
└── utils.rs           # Utility functions
```

## Common Commands

### Command Line Help
```bash
# View all available command line options and parameters
./target/release/framec --help

# Get help for specific subcommands
./target/release/framec build --help
./target/release/framec init --help
```

**Important CLI Options:**
- `-l, --language <LANG>`: Specify target language (python_3, typescript, graphviz, rust, c)
- `-m, --multifile`: Enable multi-file project compilation
- `--debug-output`: Generate JSON with transpiled code and source map
- `--validate-syntax`: Enable comprehensive syntax validation
- `-V, --version`: Print version information

### Development Workflow
```bash
# Start development session
cd /Users/marktruluck/projects/frame_transpiler

# Check current status
git status
./target/release/framec --version

# Run quick test
./target/release/framec -l python_3 test_file.frm

# Full test validation
python3 framec_tests/runner/frame_test_runner.py --languages python --framec ./target/release/framec

# Debug compilation
FRAME_TRANSPILER_DEBUG=1 ./target/release/framec -l python_3 test_file.frm

# Multi-file compilation
./target/release/framec -m entry_file.frm -l python_3
```

### Debugging Commands
```bash
# Enable debug output
export FRAME_TRANSPILER_DEBUG=1

# AST output to file
export FRAME_AST_OUTPUT=/tmp/ast.json

# Scanner vs Parser issue diagnosis
FRAME_TRANSPILER_DEBUG=1 ./target/release/framec -l python_3 problem_file.frm

# Test specific pattern
python3 framec_tests/runner/frame_test_runner.py --languages python --framec ./target/release/framec | grep PATTERN
```

### Performance Testing
```bash
# Time compilation
time ./target/release/framec -l python_3 large_file.frm

# Memory usage
/usr/bin/time -v ./target/release/framec -l python_3 large_file.frm

# Test runner with timing
python3 framec_tests/runner/frame_test_runner.py --languages python --framec ./target/release/framec --verbose
```

### TypeScript-Specific Debugging
```bash
# Debug TypeScript visitor issues
FRAME_TRANSPILER_DEBUG=1 ./target/release/framec -l typescript test.frm

# Test TypeScript compilation without execution
python3 framec_tests/runner/frame_test_runner.py --languages typescript --framec ./target/release/framec --transpile-only

# Debug call chain processing
FRAME_TRANSPILER_DEBUG=1 ./target/release/framec -l typescript dict_test.frm | grep "call chain"

# Compile and run TypeScript manually
./target/release/framec -l typescript test.frm > test.ts
npx tsc --target es2020 --module commonjs test.ts
node test.js
```

## AST Debugging and Printing

### AST Serialization
Frame provides comprehensive AST debugging through JSON serialization:

```bash
# Generate AST JSON output (requires debug mode)
FRAME_TRANSPILER_DEBUG=1 FRAME_AST_OUTPUT=/tmp/ast.json ./target/release/framec -l python_3 your_file.frm

# Note: FRAME_TRANSPILER_DEBUG=1 is REQUIRED for AST output to be generated
# The language parameter (-l) can be any valid target, as AST generation happens before visitor stage
```

### AST Printing Coverage (v0.86.0+)
Enhanced AST-to-string printing for debugging expression types:

#### **Supported Expression Types** ✅
- **Core expressions**: Variables, literals, binary/unary operations, calls
- **Collections**: Lists, dicts, sets, tuples
- **System constructs**: System instantiation (`MySystem()`), action calls (`actionName()`)  
- **Modern Python**: List/dict/set comprehensions, walrus operator (`:=`), await expressions
- **Frame-specific**: Enum access (`MyEnum.VALUE`), state transitions (`-> $State`), self references
- **Advanced features**: Unpacking (`*args`, `**kwargs`), star expressions

#### **Usage in Development**
```rust
// Debugging expressions in visitor code
let mut debug_output = String::new();
expr.accept_to_string(visitor, &mut debug_output);
eprintln!("DEBUG: Expression = {}", debug_output);
```

#### **Coverage Statistics**
- **Total expression types**: ~30
- **Implemented printing**: ~70% (improved from 40%)
- **Critical debugging constructs**: 100% coverage

### AST Structure Analysis
Use the generated JSON for:
- **System structure**: Interface methods, states, actions
- **Expression analysis**: Call chains, assignments, literals
- **Line mapping**: Source-to-generated code correlation
- **Symbol resolution**: Variable and method references

### Debugging Workflow
1. **Enable AST output**: Set `FRAME_AST_OUTPUT` environment variable
2. **Compile with debug**: Use `FRAME_TRANSPILER_DEBUG=1` for verbose output
3. **Analyze structure**: Inspect generated JSON for AST node relationships
4. **Trace expressions**: Use printing methods for detailed expression debugging

## Critical Rules

### 🚨 NEVER DO
1. **Never commit changes without explicit permission**
2. **Never create test files in the root project directory**
3. **Never manually edit generated files**
4. **Never update git config**
5. **Never use workarounds instead of fixing root causes**
6. **Never break backward compatibility without approval**
7. **Never skip the comprehensive test suite**

### ✅ ALWAYS DO
1. **Always use the official test framework** (`frame_test_runner.py`)
2. **Always test across multiple target languages**
3. **Always update version numbers for releases**
4. **Always run the full test suite before claiming completion**
5. **Always create regression tests for bug fixes**
6. **Always document significant changes**
7. **Always follow semantic versioning rules**

### 🎯 Process Requirements
1. **Test files**: Must be in `framec_tests/common/tests/[category]/`
2. **Generated output**: Goes in `framec_tests/generated/[language]/`
3. **Bug fixes**: Require regression tests with descriptive names
4. **Version updates**: Must update ALL version files together
5. **Test validation**: Must achieve 100% success rate before release

## External API Testing Strategy

### Language-Specific Approach
Frame supports importing and using external libraries/modules generically, with each visitor generating target-appropriate code from the AST. This means:

1. **No Cross-Language API Mapping**: Frame doesn't map `subprocess.spawn()` to `child_process.spawn()` - instead, Frame code should use target-appropriate APIs
2. **Import Support**: Frame syntax supports importing modules/libraries generically: `import fs`, `import os.path`
3. **Generic Method Calls**: Frame supports calling methods generically: `module.method(params)`
4. **Visitor Responsibility**: Each visitor generates correct target code from the generic AST representation

### External API Test Structure
```
framec_tests/language_specific/
├── python/external_apis/
│   ├── test_file_io.frm        # Python: os.path, open(), f.read()
│   ├── test_process.frm        # Python: subprocess.run()
│   └── test_network.frm        # Python: socket module
└── typescript/external_apis/
    ├── test_file_io.frm        # TypeScript: fs.existsSync(), fs.readFileSync()
    ├── test_process.frm        # TypeScript: child_process.spawn()
    └── test_network.frm        # TypeScript: net.createServer()
```

### Creating External API Tests
1. **Target-Specific APIs**: Write Frame code using the appropriate APIs for each target language
2. **Semantic Equivalence**: Tests should have identical functionality but use language-appropriate syntax
3. **Import Handling**: Use Frame's generic import syntax: `import module_name`
4. **Validation**: Both tests should produce identical output demonstrating equivalent functionality

### Example: File I/O
**Python Version** (`test_file_io.frm`):
```frame
import os.path

system FileIOTest {
    actions:
        testFileOperations() {
            var exists = os.path.exists("test.txt")
            var f = open("test.txt", "r")
            var content = f.read()
            f.close()
        }
}
```

**TypeScript Version** (`test_file_io.frm`):
```frame
import fs

system FileIOTest {
    actions:
        testFileOperations() {
            var exists = fs.existsSync("test.txt")
            var content = fs.readFileSync("test.txt", "utf8")
        }
}
```

## Target Language Specifics

### Python (Primary Target)
- Uses PythonVisitorV2 with CodeBuilder architecture
- Supports async/await, comprehensions, decorators
- Full Python 3.x feature compatibility
- State machines generate clean, readable Python classes

### TypeScript (Secondary Target)
- Full type annotations and interfaces
- Supports async/await, generics, decorators
- Generates ES2020+ compatible code
- Integration with Node.js runtime and `@types/node`
- **Performance Features:**
  - Batch compilation for improved speed (reduces 0.9s per file to ~1s total)
  - Shared runtime module to avoid duplicate identifier issues
  - Local vs global TypeScript compiler detection
  - Intelligent compilation caching and error recovery
- **Dependencies**: Requires Node.js and TypeScript (`npm install typescript @types/node`)
- **Recent Improvements (v0.86.1-v0.86.3)**:
  - **FIXED**: Interface method default return values (e.g., `getDefault() : int = 42`)
  - **FIXED**: Event handler return value overrides (e.g., `getOverride() : int = 99`)
  - **FIXED**: Call chain handling for nested dictionary access
  - **BREAKTHROUGH v0.86.3**: Operators category 87.5% success (+31.3% improvement)
  - **FIXED**: Unary minus operator (`-`) now generates `-` instead of `!`
  - **FIXED**: Matrix multiplication (@) operator now generates `.matmul()` calls
  - **FIXED**: `len()` function now properly generates `x.length` instead of `(x)`
  - **FIXED**: Property access bug - no more `.length` added to numeric literals
  - **ENHANCED**: 'in'/'not in' operators with comprehensive type checking
  - **FIXED**: Array length comparisons with comprehensive parentheses support
  - **FIXED**: String slicing operations (e.g., `text[0:3]` → `text.slice(0, 3)`)
  - **FIXED**: Set literals (e.g., `{1, 2, 3}` → `new Set([1, 2, 3])`)
  - **FIXED**: Tuple literals (e.g., `(1, 2, 3)` → `[1, 2, 3]`)
  - **FIXED**: Dictionary comprehensions (e.g., `{x: x*x for x in nums}` → `Object.fromEntries(nums.map(x => [x, x*x]))`)
  - **RESULT**: Achieved TypeScript success rate of 74.6% (320/429 tests), excellent performance across multiple categories

### GraphViz (Visualization)
- Generates DOT format for state diagrams
- Multi-system support with clear separation
- Hierarchical state machine visualization
- Debug and documentation purposes

## Async/Await Capabilities (v0.86.15)

Frame provides unified async/await functionality across target languages using embedded runtime libraries.

### TypeScript Async Support
- **FrameAsync Runtime**: Embedded async operations library
- **HTTP Operations**: `FrameAsync.httpGet()`, `FrameAsync.httpPost()`
- **Concurrency**: `FrameAsync.parallel()`, `FrameAsync.sequence()`, `FrameAsync.race()`
- **Timing**: `FrameAsync.sleep()`, `FrameAsync.timeout()`

### Frame Async Syntax
```frame
module AsyncCapabilities {
    async fn httpGet(url) {
        var response = await fetch(url)
        return response
    }
    
    async fn parallel(tasks) {
        var results = []
        for task in tasks {
            var result = await task
            results.append(result)
        }
        return results
    }
}
```

### Generated TypeScript
```typescript
export namespace AsyncCapabilities {
    export async function httpGet(url: any): Promise<any> {
        let response = await fetch(url);
        return response;
    }
    
    export async function parallel(tasks: any): Promise<any> {
        let results = [];
        for (const task of tasks) {
            let result = await task;
            results.push(result);
        }
        return results;
    }
}
```

### Runtime Architecture
- **Embedded Runtime**: No external dependencies
- **Type Safety**: Full TypeScript annotations
- **Cross-Language Template**: Ready for Rust, C#, Java
- **Semantic Consistency**: Identical behavior across targets

## Error Patterns and Solutions

### Common Parser Issues
1. **Token synchronization**: Use `check()` instead of `match_token()` in loops
2. **Context loss**: Ensure proper error recovery in complex blocks
3. **Unicode handling**: Scanner must handle multi-byte characters properly

### Common Test Issues
1. **File not found**: Check absolute paths and working directory
2. **Version mismatch**: Ensure all version files are synchronized
3. **Permission errors**: Check file permissions and ownership

### Performance Issues
1. **Large files**: Parser must handle 900+ line files efficiently
2. **Memory usage**: Avoid unnecessary AST node duplication
3. **Compilation speed**: Optimize visitor patterns for speed

## Additional Resources

### Documentation
- `CLAUDE.md`: Project instructions for AI assistants
- `CLAUDE.local.md`: Private project instructions
- `docs/`: Technical documentation and guides
- `README.md`: Project overview and quick start

### Configuration Files
- `Cargo.toml`: Rust package configuration
- `version.toml`: Centralized version management
- `clippy.toml`: Rust linting configuration
- `.gitignore`: Version control exclusions

---

**Last Updated**: 2025-10-22  
**Version**: v0.86.16  
**Status**: Python execution 100% (458/458) · TypeScript execution 82.5% (354/429) — debugger-controller readiness in progress

**Remember**: This document is the single source of truth for Frame Transpiler development processes. When in doubt, refer to this guide.
