# Frame Language Transpiler

This project contains the code for building the Frame Language Transpiler - the **Framepiler**.  The Framepiler is written in Rust and transpiles Frame specification documents into Python (with more languages to come) as well as UML Statechart diagrams.

**Current Version**: v0.58  
**Test Success Rate**: 100% (374/374 tests passing) 🎉  
**Rust Version**: 1.89.0 (2025-08-04)  
**Last Updated**: 2025-09-16

## Current Features (v0.58)

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

## Previous Features (v0.57)

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

### Frame Standard Library (FSL)
- **Type Conversions**: `str()`, `int()`, `float()`, `bool()`
- **List Operations**: `.append()`, `.pop()`, `.sort()`, `.reverse()`, etc.
- **String Operations**: `.upper()`, `.lower()`, `.split()`, `.replace()`

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

