# Frame Transpiler Project Status

## Current State

**Branch**: `dev`  
**Version**: `v0.86.15`  
**Status**: ✅ Python 458/458 passing (100%) · ⚠️ TypeScript execution 74.6% (320/429)  
**Achievement**: Unified async/await capabilities with the embedded TypeScript runtime  
**Latest Snapshot (2025-10-21)**: Overall TypeScript execution success 80.5% across 887 total tests  
**Rust Version**: 1.89.0 (Latest Stable)

## Latest Updates (October 21, 2025)

### 🚀 Frame v0.86.15 - Async Runtime Consolidation
- **Async/Await Unification**: Embedded TypeScript runtime aligned with Python async behavior
- **Test Coverage**: Python suite remains 100% (458/458); TypeScript execution success holding at 74.6% (320/429)
- **Focus**: Continue closing the remaining TypeScript execution gaps while maintaining cross-target parity

### ✅ Previous Updates (October 19, 2025)

#### Frame v0.85.6 - Critical TypeScript Bug Fixes Complete
- **Bug #52 RESOLVED**: Interface method calls now generate `this.method()` correctly in TypeScript
- **Bug #53 RESOLVED**: Exception variables now generate `e` not `this.e` in TypeScript  
- **Regression Tests**: Added targeted fixtures for both bug classes
- **Result**: TypeScript generation unblocked VS Code extension development; execution rate subsequently tracked separately (see current snapshot)

### ✅ Toolchain & Dependency Updates (January 27, 2025)
- **Rust**: Updated from 1.83.0 → 1.89.0 (latest stable)
- **Edition**: Upgraded from Rust 2018 → 2021
- **All Dependencies**: Updated to latest versions
- **Build Status**: Zero warnings, zero deprecations
- **Test Suite**: 100% passing (341/341 tests)

### ✅ Frame v0.56 Python Enhancement Features
- **Walrus Operator (:=)**: Assignment expressions for inline variable creation
- **Numeric Literal Underscores**: `1_000_000`, `0xFF_FF` for readability
- **Complex Numbers**: `3+4j`, `2.5j` imaginary number support
- **Type Aliases**: Python 3.12+ style `type MyType = int`
- **Scientific Notation**: `1.23e10`, `6.022e23` exponential notation

### ✅ Frame v0.55 State Parameters & Type Annotations
- **State Parameters**: States can receive and store parameters during transitions
- **Type Annotations**: Full support for parameter and return type hints
- **Property Decorators**: `@property` for computed properties in classes
- **100% Test Success**: All 339 tests passing (milestone achievement)

### ✅ Frame v0.54 Collection Operations
- **Star Expressions**: Unpacking with `*` in assignments and calls
- **Collection Constructors**: `list()`, `dict()`, `set()`, `tuple()`
- **Tuple Unpacking**: Multiple assignment support

### ✅ Frame v0.53 Multiple Assignment
- **Multiple Variable Declaration**: `var x, y, z = 1, 2, 3`
- **Tuple Unpacking**: `var (a, b) = get_pair()`
- **Chained Assignment**: `x = y = z = 0`

## Complete Feature Set (v0.30 - v0.56)

### Core Language Features
- **State Machines**: Hierarchical state machines with enter/exit handlers
- **Multi-Entity Support**: Multiple functions and systems per module
- **Module System**: Named modules with qualified access
- **Classes**: Object-oriented programming with methods and variables
- **Async/Await**: Full async function and event handler support
- **Pattern Matching**: Complete match-case support with guards
- **Generators**: Regular and async generators with yield

### Python Operator Alignment (100% Complete)
- **All Logical Operators**: `and`, `or`, `not`
- **All Bitwise Operators**: `&`, `|`, `~`, `^`, `<<`, `>>`
- **All Compound Assignments**: `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, etc.
- **Identity/Membership**: `is`, `is not`, `in`, `not in`
- **Matrix Multiplication**: `@` and `@=` operators
- **Floor Division**: `//` and `//=`
- **Exponentiation**: `**` with right associativity

### Python String Support
- **F-strings**: Formatted string literals with expressions
- **Raw strings**: No escape sequence processing
- **Byte strings**: Binary data representation
- **Triple-quoted**: Multi-line strings
- **Percent formatting**: Classic Python string formatting

### Advanced Features
- **List/Dict/Set Comprehensions**: Full Python-style comprehensions
- **Slicing**: Complete Python-style slicing support
- **With Statements**: Context manager support
- **Try-Except**: Exception handling with finally
- **Assert Statements**: Runtime assertions
- **Del Statement**: Explicit deletion
- **Global Keyword**: Explicit global access
- **Enums**: Custom values, string enums, iteration
- **Import Statements**: Native Python imports

### Native Python Operations
- **Type Conversions**: `str()`, `int()`, `float()`, `bool()`
- **List Operations**: All Python list methods work natively
- **String Operations**: All Python string methods work natively
- **Dict Operations**: All Python dict methods work natively
- **No Import Required**: Python built-in functions work directly in Frame

## Test Infrastructure

### Test Runner
- **Location**: `framec_tests/runner/frame_test_runner.py`
- **Test Files**: `framec_tests/python/src/test_*.frm`
- **Total Tests**: 341
- **Success Rate**: 100%

### Reports
- **Test Log**: `framec_tests/reports/test_log.md`
- **Test Matrix**: `framec_tests/reports/test_matrix_v0.31.md`
- **JSON Results**: `framec_tests/reports/test_results_v0.31.json`

## Documentation Structure

### Frame Language Design
- **Location**: `docs/framelang_design/`
- **Grammar**: `grammar.md` - Complete BNF specification (v0.56)
- **Dev Notes**: `dev_notes.md` - Version history and features

### Achievement Documents
- **Location**: `docs/`
- **Files**: `v0.30_achievements.md` through `v0.56_achievements.md`
- **Coverage**: Complete documentation of all features per version

## Build Instructions

```bash
# Build transpiler
cargo build --release

# Run tests
cd framec_tests
python3 runner/frame_test_runner.py --all --framec ../target/release/framec

# Transpile example
./target/release/framec -l python_3 file.frm
```

## Language Support

### 1st Class (Full Implementation)
- **Python 3**: Complete visitor with all features (341/341 tests passing)
- **TypeScript**: Complete visitor with all features (423/423 tests passing)

### 2nd Class (Design Guides)
- C/C++, JavaScript, C#, Java, Go, Rust

### Target Options
- `python_3`: Python code generation
- `typescript`: TypeScript code generation
- `plantuml`: UML state diagram generation

## Project Metrics

- **Versions Released**: 55 (v0.30 through v0.84.0)
- **Test Files**: 764 (341 Python + 423 TypeScript)
- **Success Rate**: 100% (both languages)
- **Language Features**: Complete coverage of Python 3.8+ and TypeScript ES2020+ features
- **Code Quality**: Zero warnings, zero deprecations
- **Rust Edition**: 2021
- **Min Rust Version**: 1.89.0

## Next Steps

1. **Rust Target**: Implementation of Rust code generation
2. **Multi-File Support**: Import Frame modules from other .frm files
3. **Build System**: Package management and dependency resolution
4. **Performance**: Optimization of transpiler performance
5. **IDE Support**: Enhanced VSCode extension features

## Contact

- **Creator**: Mark Truluck
- **Discord**: [The Art of the State](https://discord.com/invite/CfbU4QCbSD)
- **Documentation**: [Read the Docs](https://docs.frame-lang.org)
- **Playground**: [Frame Playground](https://playground.frame-lang.org)
- **Issues**: bugs@frame-lang.org
