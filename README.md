# Frame Language Transpiler

This project contains the code for building the Frame Language Transpiler - the **Framepiler**.  The Framepiler is written in Rust and transpiles Frame specification documents into Python, TypeScript, and GraphViz as well as UML Statechart diagrams.

**Current Version**: v0.86.25  
**Test Framework**: Unified multi-language testing (905 total tests, enforced in CI)  
**Python Tests**: 100% execution (462/462)  
**TypeScript Tests**: 100% execution (433/433)  
**LLVM Smoke Tests**: 100% execution (10/10)  
**Rust Version**: 1.89.0 (2025-08-04)  
**Last Updated**: 2025-10-28

## Runtime & Standard Library

Frame programs run atop two generated layers:

| Layer | Purpose |
| --- | --- |
| **FrameRuntime** | Implements language semantics (state machine scheduling, Frame collections, truthiness helpers). Emitted automatically per target; user code should never reference it directly. |
| **Frame Standard Library (FSL)** | Target-agnostic capability modules (networking, filesystem, process control, timers, etc.). Developers import FSL modules in Frame source; each backend provides its implementation. |

Keep capability work in the FSL so Frame specs stay portable while the runtime focuses purely on language behavior.

## Current Focus (v0.86.25) 🛠️ Native Backend Readiness
- LLVM runtime grows queue plumbing: compartments now expose `frame_runtime_compartment_set_forward_event`, and transitions reuse the active compartment pointer so forwarded events can route through the kernel.
- New LLVM smoke coverage (`test_action_locals.frm`) exercises action locals mutating typed (`int`) and inferred (`string`) domain fields to ensure native code handles mixed typing.
- MacOS packaging remains the priority for LLVM; new docs capture the updated helper APIs so future targets can reuse them.
- Python and TypeScript suites remain green (900 specs total) while the LLVM backend edges toward full parent-forward semantics.
- GitHub Actions continues to run LLVM smoke tests alongside Python/TypeScript regressions to guard the native backend.

## Recent Improvements (v0.86.18 – v0.86.25)
- **LLVM Queue Prep (v0.86.25):** Runtime compartments add a forward-event setter, the builder reuses active compartment pointers, and new smoke tests (`test_action_locals.frm`, `test_parent_forward_queue.frm`) lock in typed/untyped mutations plus queued parent forwarding ahead of full enter/exit wiring.
- **LLVM Domain Actions (v0.86.22):** The experimental LLVM backend now lowers actions, expands domain assignments, and supports string/bool initialisers so smoke tests cover real state mutations.
- **Async Runtime Parity (v0.86.21):** Automatic detection of async systems now upgrades generated TypeScript dispatchers, interface methods, and kernel loops to `async`/`await`, matching Python semantics for mixed sync/async event handlers.
- **External API Alignment (v0.86.21):** Python network/process specs now call the emitted action helpers instead of direct method names, eliminating runtime attribute errors while retaining the original behaviour.
- **Negative Suite Expansion (v0.86.21):** Added a dedicated nested-function regression test to guarantee the parser rejects unsupported inner function declarations.
- **All-Green Test Runs (v0.86.21):** Python (462) and TypeScript (433) suites now execute cleanly, including language-specific external API fixtures.
- **LLVM Backend (Phase 1 preview):** `framec -l llvm` emits LLVM IR with system structs, event dispatch, print lowering, and state transitions, providing a foundation for the native backend roadmap.
- **LLVM Runtime (Week 8 scaffold):** `runtime/llvm` now ships the minimal FrameRuntime ABI (`frame_runtime_llvm`) so the backend can link against shared event/compartment helpers while the kernel evolves.

## Previous Features (v0.85.4) ✅ BUG #50 PARSER ERROR HANDLING COMPLETE!

### Critical Parser Error Handling Fix
- **Bug #50 Resolution**: Fixed misleading error messages that masked real parsing problems.
- **Improved Error Reporting**: Parser now highlights precise syntax issues instead of generic module-level errors.
- **Complex File Support**: Large Frame specifications (900+ lines) surface actionable diagnostics without test regressions.
- **Cross-Language Fix**: Improvements apply to Python, TypeScript, and GraphViz generation.

### Perfect TypeScript Code Generation (v0.85.0)
- **TypeScript Actions**: Generate complete implementations instead of TODO placeholders
- **Frame Debugging Unblocked**: VS Code extension debugging fully functional
- **Try-Catch-Finally Support**: Complete TypeScript translation for Frame exception handling
- **100% TypeScript Test Success**: Perfect 426 of 426 TypeScript tests passing
- **Feature Parity**: Complete alignment between Python and TypeScript targets
- **Frame Module Support**: Frame modules generate proper TypeScript namespaces
- **Multifile Compilation**: Full support for multifile Frame programs in TypeScript
- **Operator Coverage**: All Frame operators (including In/NotIn) work correctly in TypeScript
- **Production Ready**: TypeScript code generation now fully production-ready

### Key Technical Improvements (v0.84.0)
- **Module Namespace Generation**: Frame modules become TypeScript `export namespace`
- **Runtime Class Management**: Conditional generation prevents duplication in multifile projects
- **Direct Function Returns**: Module functions generate clean TypeScript with proper return statements
- **Shared Runtime Optimization**: Efficient runtime class sharing across multifile projects

## Previous Features (v0.82.1) ✅

### CLI Improvements (NEW in v0.82.1)
- **Enhanced Help Output**: Target languages clearly listed in CLI help
- **Language Options**: Shows `python_3`, `typescript`, `graphviz`, `llvm` with descriptions
- **Better UX**: Improved documentation directly in command-line interface

## Previous Features (v0.82.0) ✅ NEW

### TypeScript Code Generation (NEW in v0.82.0)
- **New Target Language**: Full TypeScript support for Frame state machines
- **Complete Runtime**: Event-driven state machine with FrameEvent and FrameCompartment classes
- **Type-Safe Code**: Generated TypeScript compiles with strict type checking
- **All Core Features**: States, transitions, events, actions, domain variables, and expressions

## Previous Features (v0.81.6) ✅ COMPLETE

### Python Name Mangling Fix (v0.81.6)
- **Bug #47 Fixed**: Resolved triple-mangling issue in generated Python code
- **Simplified Naming Convention**: Changed from double-underscore to simple prefix patterns
- **Clean Internal Methods**: Actions use `_action_`, handlers use `_handle_`, runtime uses `_frame_`
- **100% Test Pass Rate**: All 397 tests passing with improved code generation

## Previous Features (v0.81.5) ✅ COMPLETE

### Source Mapping Improvements (v0.81.5)
- **Bug #40 Fixed**: Interface method source mappings now point to executable statements
- **Bug #35 Fixed**: Enhanced source mapping classification for statement types
- **Debugger Integration**: Improved VS Code debugging experience

## Previous Features (v0.81.3-4) ✅ COMPLETE

### Bug Resolution and Method Call Enhancements (v0.81.3)
- **Bug #38 Fixed**: Resolved string concatenation with escape sequences issue
- **Enhanced Method Resolution**: Improved conflict detection for method name ambiguity
- **Robust String Operations**: Better handling of escape sequences in generated Python code

## Previous Features (v0.81.2) ✅ COMPLETE

### System Interface Method Calls (NEW in v0.81.2)
- **New Syntax**: `system.interfaceMethod()` for calling interface methods within systems
- **Multi-Context Support**: Works in event handlers, actions, and non-static operations
- **Validation**: Prevents `self.interfaceMethod` usage with helpful error messages
- **2-Pass Parser Fix**: Improved parser architecture for proper semantic validation

## Previous Features (v0.81.1) ✅ COMPLETE

### Interface Method Default Values (Fixed in v0.81.1)
- **Return Value Semantics**: Complete implementation of Frame's return value precedence
- **Default Values**: Fixed interface method default values and handler precedence
- **Test Coverage**: Maintained 100% test compatibility with comprehensive validation

## Previous Features (v0.76) ✅ COMPLETE

### Complete Source Mapping for All Statements
- **100% Statement Coverage**: Every statement type now generates accurate source mappings
- **Event Handler Debugging**: Full debugging support inside event handlers restored
- **Zero Active Bugs**: All known source mapping issues resolved
- **Comprehensive Fix**: Added mappings to 20+ statement visitor methods
- **Perfect Debugging**: Breakpoints and step-through debugging work flawlessly

## Previous Features (v0.75) ✅ COMPLETE

### CodeBuilder Architecture with PythonVisitorV2
- **Line-Aware Code Generation**: Automatic tracking of every character and line
- **No Manual Offsets**: Eliminates all fragile +1/-1 adjustments
- **Fragment Composition**: Full support for non-linear code generation
- **Perfect Mappings**: Automatic source mapping maintenance
- **PythonVisitorV2 Default**: New visitor with CodeBuilder is now the default
- **Legacy Support**: Original visitor available via USE_PYTHON_V1 environment variable
- **100% Backward Compatible**: Identical output with robust architecture

## Previous Features (v0.74.1) ✅ COMPLETE

### Source Map Bug Fix #7 (NEW in v0.74.1)
- **Event Handler Mapping Fix**: Corrected off-by-one error in event handler source mappings
- **Removed Extra Blank Lines**: Eliminated unnecessary newlines in state node generation
- **Proper Offset Handling**: Added correct offset for visual spacing
- **Architecture Note**: Current fix uses manual offsets (fragile); v0.75 will implement robust CodeBuilder

## Previous Features (v0.74)

### Source Map Architecture & Validation Tools
- **Comprehensive Documentation**: Complete source map architecture guide
- **Marker File Linter**: Validates intermediate files during compilation
- **Bug Resolution**: All known source mapping issues resolved
- **Zero Active Bugs**: Debugging infrastructure fully functional
- **100% Test Success**: All 379 tests passing

## Previous Features (v0.65)

### Complete Code Simplification
- **Removed ALL Backward Compatibility**: No more feature flags or fallback logic
- **500+ Lines Deleted**: Removed complex helper methods and tracking code
- **Semantic Resolution Only**: Parser's resolved types are the single source of truth
- **60% Complexity Reduction**: Visitor methods dramatically simplified
- **No Environment Variables**: FRAME_SEMANTIC_RESOLUTION no longer needed
- **Cleaner Architecture**: Clear separation between parsing and code generation

## Previous Features (v0.64)

### Visitor Simplification Using Resolved Types
- **Simplified Code Generation**: Leveraged v0.63's resolved types for clean visitor code
- **New Handler Methods**: `handle_call_with_resolved_type()` replaced complex logic
- **Code Reduction**: ~350 lines of complex call chain analysis simplified
- **Performance Improvement**: No redundant analysis during code generation

## Previous Features (v0.63)

### Accurate Semantic Call Resolution
- **Accurate Type Detection**: Actions, Operations, and External calls correctly identified
- **Context-Aware Resolution**: Parser maintains system/class/function context throughout
- **Symbol Table Integration**: Direct lookups in actual symbol table data
- **Verified Accuracy**: Test output confirms correct resolution of all call types
- **100% Test Success**: All 379 tests passing with improved resolution

## Previous Features (v0.62)

### Semantic Call Resolution Infrastructure
- **ResolvedCallType Enum**: Comprehensive categorization of all call types (Action, Operation, SystemOperation, ClassMethod, ModuleFunction, External)
- **SemanticAnalyzer Module**: Dedicated semantic analysis during parser's second pass
- **Enhanced AST**: CallExprNode includes `resolved_type` field for semantic resolution
- **Feature Flag Control**: Enable via `FRAME_SEMANTIC_RESOLUTION=1` for gradual migration
- **Parser Integration**: Resolution happens at all 5 CallExprNode creation points
- **Architectural Improvement**: Moves 350+ lines of complex logic from visitor to parser
- **100% Test Success**: All existing tests pass with new architecture

## Previous Features (v0.61)

### Call Chain Analysis and Documentation (NEW in v0.61)
- **Comprehensive Analysis**: Analyzed 350+ lines of complex call chain handling logic
- **Refactoring Modules**: Created foundation for future incremental improvements
- **Documentation**: Detailed architectural insights and refactoring strategy
- **Lessons Learned**: Understanding complexity before attempting to simplify
- **100% Test Success**: Maintained stability throughout analysis and experimentation

## Previous Features (v0.60) ✅ COMPLETE

### Critical Bug Fix & Complete AST Dump Feature (NEW in v0.60)
- **Double-Call Bug Fixed**: Resolved critical issue where action calls in assignments generated incorrect double parameters
- **Complete AST Serialization**: Full JSON serialization of Frame AST for debugging and validation
- **AST Summary Output**: Concise overview with counts: `Systems (1): TestSystem (2 states), Functions (1): hello`
- **Line Mapping**: Hierarchical listing of all AST elements with accurate line numbers
- **File Export**: Save complete AST to JSON file for external analysis and validation
- **Environment Variables**: `FRAME_TRANSPILER_DEBUG=1`, `FRAME_AST_OUTPUT=filename.json`
- **Enhanced Debugging**: Foundation for advanced transpiler development and validation
- **100% Test Success**: All 378 tests now passing with improved reliability

## Previous Features (v0.59)

### 100% Source Map Generation for Debugging
- **Complete AST Coverage**: All 122 AST nodes now have line tracking (100% coverage)
- **Debug Output Mode**: Generate JSON with transpiled code and source mappings via `--debug-output` flag
- **Full Line Mapping**: Maps ALL Frame source lines to generated Python lines
- **VSCode DAP Ready**: Complete Debug Adapter Protocol support for IDE integration
- **Enhanced JSON Output**: Includes metadata with version, timestamp, and checksums
- **Zero Performance Impact**: Line tracking adds negligible overhead
- **Bug Fix**: Fixed dictionary comprehension key-value ordering issue

## Previous Features (v0.58)

### Class Decorators (NEW in v0.58)
- **Python Decorator Pass-Through**: Support for Python decorators on classes
- **Decorator Arguments**: Decorators with parenthesized arguments supported
- **Multiple Decorators**: Stack multiple decorators on a single class
- **Method Decorators Preserved**: `@staticmethod` and `@property` continue working
- **Common Use Cases**: `@dataclass`, `@total_ordering`, custom decorators
- **Smart Parser**: Context-aware parsing distinguishes class from method decorators

### GraphViz Multi-System Support (NEW in v0.58)
- **Multi-System Generation**: All systems in a Frame file are now generated
- **Clean Output**: Debug statements properly controlled by environment variable
- **System Separation**: Each system clearly labeled in concatenated output
- **VSCode Ready**: Clean DOT output for extension visualization
- **Backward Compatible**: Single system files work unchanged

### Previous Features (v0.57)

### Multi-File Module System (NEW in v0.57)
- **Frame File Imports**: Import modules from other `.frm` files
- **Three Import Syntaxes**: Standard, aliased, and selective imports
- **Dependency Resolution**: Automatic discovery and compilation ordering
- **Circular Detection**: Identifies and reports circular dependencies
- **Security Validation**: Path traversal protection and validation
- **Incremental Compilation**: SHA-256 based caching for efficiency
- **Module Access**: Use `::` for static access in Frame (transpiles to `.` in Python)
- **Separate File Generation**: Generate individual Python files with `-o` flag

### Core Language
- **State Machines**: Hierarchical state machines with enter/exit handlers
- **Multi-Entity Support**: Multiple functions and systems per module
- **Module System**: Named modules with qualified access (`module::function()`)
- **Event Handlers**: Named events, enter (`$>`), and exit (`<$`) handlers
- **Transitions**: State transitions with parameters and event forwarding

### Python Operator Alignment (v0.38-v0.40)
- **Python Comments**: `#` for single-line comments (v0.40 - C-style removed)
- **Matrix Multiplication**: `@` and `@=` operators for NumPy arrays (v0.40)
- **Bitwise XOR**: `^` and `^=` operators (v0.40)
- **Floor Division**: `//` and `//=` operators (v0.40)
- **Python Numeric Literals**: Binary (`0b`), octal (`0o`), hex (`0x`) notation (v0.40)
- **Python Logical Operators**: `and`, `or`, `not` keywords (v0.38)
- **All Compound Assignments**: `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `&=`, `|=`, `^=`, `<<=`, `>>=`, `//=`, `@=`
- **Bitwise Operators**: `&`, `|`, `~`, `<<`, `>>`, `^`
- **Identity/Membership**: `is`, `is not`, `in`, `not in` operators

### Modern Syntax Features
- **First-Class Functions**: Functions as values - pass, return, and store functions
- **Lambda Expressions**: Anonymous functions with closure support `lambda x: x * 2`
- **Async/Await**: Full async function and event handler support
- **Slicing**: Python-style slicing for strings and lists (`text[:5]`, `list[::2]`)
- **With Statements**: Context manager support (`with`/`async with`)
- **Import Statements**: Native Python imports without backticks
- **List Comprehensions**: Python-style comprehensions `[x*2 for x in list]`
- **Dictionary Comprehensions**: `{k: v for k, v in items}`
- **Exponent Operator**: Right-associative `**` for power operations
- **Collection Literals**: Lists `[]`, dicts `{}`, sets `{1,2}`, tuples `()`, empty set `{,}`

### String Literals (v0.40)
- **F-strings**: `f"Hello {name}"` - Formatted string literals with embedded expressions
- **Raw strings**: `r"C:\path"` - No escape sequence processing
- **Byte strings**: `b"binary"` - Binary data representation
- **Triple-quoted**: `"""multi-line"""` - Multi-line strings with preserved formatting
- **Percent formatting**: `"Hello %s" % name` - Classic Python string formatting


### Advanced Features
- **Pattern Matching**: Full match-case support with guards, OR patterns, star patterns
- **Classes**: Object-oriented programming with methods and variables
- **Generators**: Regular and async generators with yield expressions
- **Type Annotations**: Parameter and return type hints
- **Property Decorators**: `@property` for computed properties
- **Access Modifiers**: Public/private/protected member visibility
- **Assert Statements**: Runtime assertion checking
- **Try-Except**: Exception handling with finally blocks
- **Del Statement**: Explicit deletion of variables
- **Global Keyword**: Explicit global variable access
- **Multiple Assignment**: Tuple unpacking and multiple variable declarations
- **Star Expressions**: Unpacking operators in assignments and calls
- **State Parameters**: States can receive and store parameters
- **Enums**: Custom values, string enums, iteration support
- **Scope Resolution**: LEGB scope rules with proper isolation
- **Static Methods**: `@staticmethod` decorator support
- **Interface Methods**: Public system interfaces with async support

### Python 3.8+ Features (v0.56)
- **Walrus Operator**: `:=` assignment expressions for inline variable creation
- **Numeric Literal Underscores**: `1_000_000`, `0xFF_FF` for improved readability
- **Complex Numbers**: `3+4j`, `2.5j` imaginary number support
- **Type Aliases**: Python 3.12+ style `type MyType = int`
- **Scientific Notation**: `1.23e10`, `6.022e23` exponential notation

### Test Coverage
- **100% Success Rate**: 374/374 tests passing 🎉
- **UTF-8 Support**: Full Unicode character support in source files
- **Complete Feature Coverage**: All Python operators and core language features fully tested
- **Virtual Environment**: Test suite includes NumPy support for matrix multiplication
- **Clean Build**: Zero warnings, zero deprecations with latest Rust toolchain

## Explore Frame

To learn more about the Frame language, please find Frame's official documentation on [Read the Docs](https://docs.frame-lang.org). 

## Tools and Resources

The Frame project is still early days but there are some resources and communities to help. You can download the [VSCode](https://marketplace.visualstudio.com/items?itemName=frame-lang-org.frame-machine-maker) extension to develop 
Frame programs on your desktop or experiment with Frame online at the [Frame Playground](https://playground.frame-lang.org). 

You can also learn more about programming with automata at Reddit ![re](https://www.google.com/s2/favicons?domain_url=https://reddit.com) on the [r/statemachines](https://www.reddit.com/r/statemachines/) subreddit (I'm the mod).

## Frame Community

Connect with me and other Frame enthusists on the Frame **Discord channel** -  [The Art of the State](https://discord.com/invite/CfbU4QCbSD). You can also connect with me directly on [LinkedIn](https://www.linkedin.com/in/marktruluck/).

## Testing

Frame uses a unified testing framework that validates transpilation across all target languages.

### Quick Test
```bash
./run_tests.sh --quick  # Run core tests quickly
```

### Full Test Suite
```bash
./run_tests.sh  # Run all tests for Python and TypeScript
```

### Language-Specific
```bash
./run_tests.sh --python      # Python tests only
./run_tests.sh --typescript  # TypeScript tests only
```

### Test Categories
- **Core**: State machines, events, transitions (31 tests)
- **Control Flow**: Conditionals, loops (48 tests)
- **Data Types**: Collections, literals (66 tests)
- **Systems**: System definitions (197 tests)
- **Total**: 461 tests (432 common, 29 language-specific)

See [framec_tests/README.md](framec_tests/README.md) for detailed testing documentation.

## Quick Start - Multi-File Module Example

Here's a simple multi-file Frame project example:

**utils.frm:**
```frame
module MathUtils {
    fn add(a, b) {
        return a + b
    }
    
    fn multiply(a, b) {
        return a * b
    }
}
```

**main.frm:**
```frame
import MathUtils from "./utils.frm"

fn main() {
    var sum = MathUtils::add(5, 3)      # Use :: for module access
    var product = MathUtils::multiply(4, 7)
    print("Sum: " + str(sum))           # Output: Sum: 8
    print("Product: " + str(product))   # Output: Product: 28
}

main()
```

**Compile:**
```bash
# Option 1: Generate single concatenated file (default)
framec -m main.frm -l python_3 > output.py
python3 output.py

# Option 2: Generate separate Python files (NEW!)
framec -m main.frm -l python_3 -o ./output
python3 output/main.py
```

## Frame Examples

The [Frame Solution Depot](https://github.com/frame-lang/frame_solution_depot) is a Github repo and contains a growing body of examples and test specifications. 

## Reporting Bugs and Problems 

For now send issues to <bugs@frame-lang.org> while we get a better system in place. If you have a recommendation for a free bug tracker for open source communities please let me know!


## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Installing


#### MacOS

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Navigate to the framepiler/framec directory.
3. Type `cargo build`.
	3.a You will see a bunch of warnings. Apologies but this is pre-alpha code.
4. Type `./target/debug/framec ../examples/HelloWorld.frm python_3`.
	4.a You should see a base class for a Frame controller generated to stdout.
5. If you want to generate a release build:
	5.a Type `cargo build --release`
	5.b Type `./target/release/framec ../examples/HelloWorld.frm python_3`
6. You now have working debug and release Framepilers. Congratulations!
7. You can try 6 other languages + Plant UML. Replace the `python_3` above with any of these:
	7.a `python_3`
	7.b `plantuml` (try output at [PlantUml site](http://www.plantuml.com/))

#### Linux

1. Install  [Rust](https://www.rust-lang.org/tools/install).
2. Probably the same as MacOS but guessing you can figure it out if you know Linux and Rust. Still - please send me instructions on [Discord](https://discord.com/invite/CfbU4QCbSD)  and I will add to next release notes. Thanks!

#### Windows

1. Install  [Rust](https://www.rust-lang.org/tools/install).
2. Help needed. Please send me instructions on [Discord](https://discord.com/invite/CfbU4QCbSD)  and I will add to next release notes. Thanks!

## Built With

* [Rust](https://www.rust-lang.org/) - Rust language

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/frame-lang/064097505d77b7ecb7f49a30f75622c4) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/frame-lang/frame_transpiler/tags).

## Author

**Mark Truluck** - *Creator of Frame* - [LinkedIn](https://www.linkedin.com/in/marktruluck/)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* [Alan Turing](https://en.wikipedia.org/wiki/Alan_Turing) - For inventing automata theory and helping end WWII. See [The Imitation Game](https://www.imdb.com/title/tt2084970/)
* [Dr. David Harel](http://www.wisdom.weizmann.ac.il/~harel/papers.html) - Who invented [Statecharts](https://www.sciencedirect.com/science/article/pii/0167642387900359) from which came Frame.
