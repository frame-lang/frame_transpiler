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

Frame is a state machine language that transpiles to multiple target languages (Python, TypeScript, C#, etc.). The project is currently in v0.85.2 and migrating from v0.11 to v0.20 syntax.

### Current Status
- **Version**: v0.85.2
- **Branch**: `main`
- **Test Success Rate**: 100% (883 total tests: 456 Python + 427 TypeScript)
- **Supported Targets**: Python 3, TypeScript, GraphViz

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
├── generated/python/       # Generated Python code
├── generated/typescript/   # Generated TypeScript code
├── runner/                 # Test runner framework
└── configs/               # Test configuration files
```

### Running Tests

**Comprehensive Test Suite:**
```bash
# All Python tests
python3 framec_tests/runner/frame_test_runner.py --languages python --framec ./target/release/framec

# All TypeScript tests  
python3 framec_tests/runner/frame_test_runner.py --languages typescript --framec ./target/release/framec

# Specific categories
python3 framec_tests/runner/frame_test_runner.py --languages python --categories regression --framec ./target/release/framec

# Verbose output
python3 framec_tests/runner/frame_test_runner.py --languages python --framec ./target/release/framec --verbose
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
- **systems**: Core state machine functionality (200 tests)
- **data_types**: Collections, dictionaries, lists (66 tests)
- **control_flow**: Conditionals, loops, multifile (49 tests)
- **scoping**: Variable scoping and resolution (45 tests)
- **core**: State management and transitions (31 tests)
- **language_specific_python**: Python-specific features (29 tests)
- **operators**: Python operators and expressions (16 tests)
- **negative**: Error handling and validation (13 tests)
- **regression**: Bug prevention tests (7 tests)

## Version Management

### Version Files (ALL must be updated together)
1. `framec/Cargo.toml` - Package version
2. `frame_build/Cargo.toml` - Build tool version
3. `version.toml` - Central version configuration
4. `framec/src/frame_c/compiler.rs` - FRAMEC_VERSION constant

### Semantic Versioning Rules
- **Bug fixes**: Increment patch version (0.85.1 → 0.85.2)
- **Minor features**: Increment minor version (0.85.x → 0.86.0)
- **Major changes**: Only the project owner declares major version bumps

### Version Update Process
```bash
# 1. Update all version files
# 2. Update Cargo.lock
cargo update

# 3. Rebuild to pick up new version
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
- Integration with Node.js runtime

### GraphViz (Visualization)
- Generates DOT format for state diagrams
- Multi-system support with clear separation
- Hierarchical state machine visualization
- Debug and documentation purposes

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

**Last Updated**: 2025-10-18  
**Version**: v0.85.2  
**Status**: Production Ready - 100% Test Success Rate

**Remember**: This document is the single source of truth for Frame Transpiler development processes. When in doubt, refer to this guide.