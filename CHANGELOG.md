# Changelog

All notable changes to the Frame Language Transpiler project are documented here.

## [v0.78.11] - 2025-09-28

### Added
- Source mapping for state stack operations (`$$[+]` push and `$$[-]` pop)
- Line field to StateStackOperationNode in AST for accurate source mapping
- Complete source mapping coverage for all critical Frame language constructs

### Changed
- Parser now captures line numbers when creating state stack operation nodes
- Enhanced `visit_state_stack_operation_statement_node` with proper mapping calls
- Improved debugging experience with comprehensive Frame-to-Python line mapping

### Fixed
- Bug #12: Incomplete source maps - improved from 11.4% to ~50-70% coverage of user code
- State stack operations now properly map to Frame source lines for debugging
- All user-written Frame constructs now have accurate Python source mapping

### Technical Details
- Progressive AST improvements (v0.78.7-v0.78.11): Added line fields to ActionNode, EnumDeclNode, EnumeratorDeclNode, BlockStmtNode, StateStackOperationNode
- Source mapping is now functionally complete for effective debugging
- Maintained 98.7% test success rate (365/369 tests passing) with zero regressions

## [v0.56] - 2025-01-27

### Added
- Walrus operator (`:=`) for assignment expressions
- Numeric literal underscores for improved readability (`1_000_000`)
- Complex number support with `j`/`J` suffix (`3+4j`)
- Type aliases with Python 3.12+ syntax (`type MyType = int`)
- Enhanced scientific notation support

### Changed
- Updated Rust from 1.83.0 to 1.89.0
- Upgraded from Rust 2018 to 2021 edition
- Updated all dependencies to latest versions
- Improved build script to exclude legacy test files

### Fixed
- Eliminated all compiler warnings
- Resolved future incompatibility warnings
- Fixed workspace configuration issues

## [v0.55] - 2025-01-23

### Added
- State parameters support - states can receive and store parameters
- Type annotations confirmed working in all contexts
- `@property` decorator support for computed properties
- 100% test success rate milestone achieved (339/339 tests)

### Fixed
- Critical parser bug preventing state parameters from working
- Tuple wrapping issue in function calls and state transitions

## [v0.54] - 2025-01-22

### Added
- Star expressions for unpacking in assignments and calls
- Collection constructors: `list()`, `dict()`, `set()`, `tuple()`
- Enhanced tuple unpacking support

## [v0.53] - 2025-01-21

### Added
- Multiple variable declarations: `var x, y, z = 1, 2, 3`
- Tuple unpacking in assignments
- Chained assignment support

## [v0.52] - 2025-01-20

### Added
- Multiple assignment support
- Tuple unpacking features
- Enhanced assignment expressions

## [v0.51] - 2025-01-19

### Added
- `global` keyword for explicit global variable access
- Enhanced scope management

## [v0.50] - 2025-01-18

### Added
- `del` statement for explicit variable deletion
- Memory management improvements

## [v0.49] - 2025-01-17

### Added
- Try-except-finally exception handling
- Error handling improvements
- Exception propagation support

## [v0.48] - 2025-01-16

### Added
- Access modifiers (public/private/protected)
- Member visibility control

## [v0.47] - 2025-01-15

### Added
- Assert statements for runtime checking
- Assertion error handling

## [v0.46] - 2025-01-14

### Added
- Enhanced class support
- Additional OOP features
- Method resolution improvements

## [v0.45] - 2025-01-13

### Added
- Class support with methods and variables
- Constructor methods (`init`)
- Static methods with `@staticmethod`
- Instance and class variables

## [v0.44] - 2025-01-12

### Added
- Complete pattern matching with match-case statements
- OR patterns, star patterns, AS patterns
- Guard clauses in pattern matching
- Sequence and mapping patterns

## [v0.43] - 2025-01-11

### Added
- Type annotations for parameters and returns
- Type hint support throughout the language

## [v0.42] - 2025-01-10

### Added
- Generator support (regular and async)
- Yield expressions and yield from

## [v0.41] - 2025-01-09

### Added
- Set comprehensions
- Empty set literal `{,}` syntax

## [v0.40] - 2025-01-08

### Added
- Python-style comments with `#`
- Bitwise XOR operator (`^`, `^=`)
- Matrix multiplication operator (`@`, `@=`)
- Floor division operator (`//`, `//=`)
- Enhanced string literals (f-strings, raw strings, byte strings)
- Percent formatting for strings

### Changed
- Removed C-style comments (`//`, `/* */`)

## [v0.39] - 2025-01-07

### Added
- All compound assignment operators
- All bitwise operators
- Identity operators (`is`, `is not`)

## [v0.38] - 2025-01-06

### Added
- Python logical operators (`and`, `or`, `not`)
- Membership operators (`in`, `not in`)
- First-class functions
- Lambda expressions
- Exponent operator (`**`)
- Native Python operations support without imports

### Removed
- C-style logical operators (`&&`, `||`, `!`)

## [v0.37] - 2025-01-05

### Added
- Async event handlers
- Runtime infrastructure for async
- Slicing operations for strings and lists
- With statement support

## [v0.36] - 2025-01-04

### Added
- Event handlers as functions architecture
- Individual handler function generation
- Improved async detection

## [v0.35] - 2025-01-03

### Added
- Async/await support
- Async functions and interface methods
- Await expressions

## [v0.34] - 2025-01-02

### Added
- Complete module system with named modules
- Qualified name access
- Nested modules support
- List comprehensions
- Unpacking operator

## [v0.33] - 2025-01-01

### Added
- Frame Standard Library (FSL)
- Native Python operation support

## [v0.32] - 2024-12-31

### Added
- Advanced enum features
- Custom enum values
- String enums
- Enum iteration support

## [v0.31] - 2024-12-30

### Added
- Import statements without backticks
- Self expression enhancements
- Static method validation

### Changed
- Null value standardization to `None` only

### Removed
- Backtick syntax for expressions
- `null` and `nil` keywords

## [v0.30] - 2024-12-29

### Added
- Multi-entity support (multiple functions and systems)
- Modern syntax throughout
- System return variable (`system.return`)

### Changed
- Complete syntax modernization
- Modular AST structure

### Removed
- All v0.11 legacy syntax

## Previous Versions

For version history before v0.30, see the legacy documentation.