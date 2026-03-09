# Frame v0.56 Release Notes

**Release Date**: January 27, 2025  
**Test Success Rate**: 100% (341/341 tests passing)  
**Rust Version**: 1.89.0

## Major Features

Frame v0.56 introduces several Python 3.8+ features, enhancing Frame's alignment with modern Python syntax and maintaining our 100% test success rate.

### 1. Walrus Operator (Assignment Expressions) `:=`

The walrus operator allows inline variable assignment that returns the assigned value, enabling more concise code patterns.

```frame
fn process_data() {
    # Assign and use in condition
    if (n := get_next_value()) > 0 {
        print("Got positive: " + str(n))
    }
    
    # Use in while loops
    while (line := read_line()) != "" {
        process(line)
    }
    
    # In comprehensions
    var results = [y for x in data if (y := transform(x)) > 0]
}
```

**Key Features:**
- Creates variables in current scope (not new scope)
- Returns the assigned value for immediate use
- Works in conditions, loops, and comprehensions
- Full Python 3.8+ compatibility

### 2. Numeric Literal Underscores

Underscores can now be used as digit separators in numeric literals for improved readability.

```frame
fn numeric_examples() {
    # Decimal numbers
    var million = 1_000_000
    var precise = 3.141_592_653_589
    
    # Different bases
    var hex_color = 0xFF_FF_FF
    var binary = 0b1111_0000_1111_0000
    var octal = 0o755_644
    
    # Scientific notation
    var avogadro = 6.022_140_76e23
}
```

**Supported in:**
- Decimal integers and floats
- Binary literals (`0b`)
- Octal literals (`0o`)
- Hexadecimal literals (`0x`)
- Scientific notation

### 3. Complex Number Support

Native support for complex numbers with imaginary unit suffix `j` or `J`.

```frame
fn complex_math() {
    # Pure imaginary
    var i = 3j
    var i2 = 2.5J
    
    # Complex numbers
    var z1 = 3 + 4j
    var z2 = complex(2, -3)
    
    # Operations
    var sum = z1 + z2
    var product = z1 * z2
    var conjugate = z1.conjugate()
    var magnitude = abs(z1)
}
```

### 4. Type Aliases (Python 3.12+)

Type aliases provide a way to create named types for better code documentation and reusability.

```frame
# Simple type alias
type UserID = int
type Coordinate = tuple[float, float]

# Generic type alias (Python 3.12+ syntax)
type Result[T] = tuple[bool, T]
type Handler[T] = lambda[T, None]

fn process_user(id: UserID) : Result[str] {
    if id > 0 {
        return (true, "User found")
    }
    return (false, "")
}
```

**Features:**
- Module-level type alias declarations
- Generic type parameters with `[T]` syntax
- Proper Python 3.12+ code generation
- Context-sensitive `type` keyword (doesn't conflict with `type()` function)

### 5. Scientific Notation Enhancement

Full support for exponential notation in numeric literals.

```frame
fn scientific() {
    var avogadro = 6.022e23
    var planck = 6.626e-34
    var electron_mass = 9.109_383_56e-31
    
    # Both e and E work
    var light_speed = 3E8
    var tiny = 1.5E-10
}
```

## Technical Improvements

### Dependency Updates

All dependencies have been updated to their latest versions:

- **Rust Edition**: 2018 → 2021
- **clap**: 3.0.14 → 4.5.47
- **convert_case**: 0.4.0 → 0.6.0
- **indoc**: 1.0 → 2.0
- **wasm-bindgen**: 0.2.79 → 0.2.101
- **Removed**: deprecated structopt

### Build Quality

- **Zero Warnings**: All compiler warnings eliminated
- **Zero Deprecations**: No future incompatibility warnings
- **Clean Build**: Both debug and release modes build cleanly
- **Panic Removal**: User-facing panics converted to proper error handling

### Parser Enhancements

- **Context-Sensitive Keywords**: `type` keyword only recognized in type alias context
- **Improved Number Parsing**: Enhanced scanner for underscores and complex numbers
- **Scientific Notation**: Proper handling of `e`/`E` in numeric literals

## Bug Fixes

1. **Future Compatibility**: Resolved wasm-bindgen deprecation warnings
2. **Build Script**: Updated to exclude legacy test files with backticks
3. **Workspace Configuration**: Fixed framec_tests workspace exclusion

## Test Coverage

- **Total Tests**: 341 (up from 339)
- **New Tests**: 2 tests for v0.56 features
- **Success Rate**: 100%
- **Categories**: All feature categories passing

## Breaking Changes

None - v0.56 maintains full backward compatibility with v0.55.

## Migration Guide

No migration required. All existing Frame code continues to work unchanged.

### Optional Enhancements

You can optionally adopt new v0.56 features:

1. **Use walrus operator** for more concise conditionals
2. **Add underscores** to large numeric literals for readability
3. **Use type aliases** for complex type definitions
4. **Leverage complex numbers** for mathematical computations

## Known Issues

None - all features working as designed.

## Platform Support

- **macOS**: ✅ Fully supported
- **Linux**: ✅ Fully supported  
- **Windows**: ✅ Fully supported (via Rust toolchain)

## Next Release Preview (v0.57)

Planned features:
- Enhanced error messages with source location
- Performance optimizations
- Additional Python 3.12+ features
- Improved IDE integration

## Acknowledgments

Thanks to the Frame community for continued support and feedback!

## Resources

- **Documentation**: [Read the Docs](https://docs.frame-lang.org)
- **Discord**: [The Art of the State](https://discord.com/invite/CfbU4QCbSD)
- **Playground**: [Frame Playground](https://playground.frame-lang.org)
- **VSCode Extension**: [Frame Machine Maker](https://marketplace.visualstudio.com/items?itemName=frame-lang-org.frame-machine-maker)