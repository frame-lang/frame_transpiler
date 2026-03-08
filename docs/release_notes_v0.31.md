# Frame v0.31 Release Notes

**Release Date**: September 1, 2025  
**Status**: Production Ready - 100% Test Coverage

## Executive Summary

Frame v0.31 represents a major milestone with **100% test success rate** (166/166 tests passing). This release completes the language modernization effort with full support for module variables, native import statements, enhanced self expression, and critical bug fixes for domain variable access and static method calls.

## Major Features

### 1. Module Variables with Automatic Global Generation
- Declare variables at module scope accessible from all functions and systems
- Automatic `global` declarations generated for Python target
- Shadowing protection prevents local variables from shadowing module variables
- Two-pass analysis ensures correct scope resolution

```frame
var counter = 0
var data = []

fn increment() {
    counter = counter + 1  // Automatic 'global counter' in Python
    data.append(counter)
    return counter
}
```

### 2. Native Import Statement Support
- Simple imports: `import math`
- Aliased imports: `import numpy as np`
- From imports: `from collections import defaultdict`
- Wildcard imports: `from typing import *`
- No more backticks required for Python imports

```frame
import math
from typing import List, Dict

fn calculate() {
    var pi = math.pi
    var items: List[int] = [1, 2, 3]
    return pi * len(items)
}
```

### 3. Enhanced Self Expression
- Standalone `self` usage (e.g., `jsonpickle.encode(self)`)
- Full `self.variable` syntax for domain variable access
- Works in all contexts: lvalue, rvalue, nested expressions
- Static method validation prevents `self` in `@staticmethod` operations

```frame
system DataManager {
    domain:
        var count: int = 0
        var data = []
    
    operations:
        process() {
            self.count = self.count + 1  // Clean self.variable syntax
            self.data.append(self.count)
            return jsonpickle.encode(self)  // Standalone self
        }
}
```

### 4. Static Method Support
- Operations default to instance methods
- `@staticmethod` decorator for static operations
- Parse-time validation ensures no `self` in static methods
- Cross-system static method calls work correctly

```frame
system Utils {
    operations:
        @staticmethod
        calculate(x: int): int {
            return x * 2
        }
}

system Main {
    machine:
        $Start {
            test() {
                var result = Utils.calculate(42)  // Static call
                return
            }
        }
}
```

## Bug Fixes

### Critical Fixes
1. **Self.Variable Double Reference Bug**: Fixed `self.x` generating `self.self.x` in Python output
2. **Static Method Calls**: Fixed `SystemName.method()` generating `SystemName.self.method()`
3. **Domain Variable Syntax**: Corrected test file syntax requiring `var` keyword

### Affected Tests (Now Passing)
- test_self_domain_vars.frm
- test_self_variable_exhaustive.frm
- test_static_calls.frm
- test_v031_comprehensive.frm
- test_domain_assignment.frm
- test_domain_type_debug.frm
- test_explicit_self_syntax.frm
- test_simple_validation.frm
- test_validation_with_main.frm

## Breaking Changes

### None Keyword Standardization
- **Removed**: `null` and `nil` keywords no longer supported
- **Standard**: Use `None` exclusively for null values
- **Migration**: Replace all instances of `null` and `nil` with `None`

```frame
// Old (no longer works)
var x = null
var y = nil

// New (required)
var x = None
var y = None
```

## Test Coverage

### Test Statistics
- **Total Tests**: 166
- **Passing**: 166
- **Failing**: 0
- **Success Rate**: 100%

### Test Categories (All Passing)
- Import statements
- Module variables
- Self expression
- Static methods
- Domain variables
- Scope resolution
- Multi-entity support
- Hierarchical state machines
- System parameters
- Interface methods

## Technical Implementation

### Parser Enhancements
- Two-pass analysis for module variable detection
- Shadowing protection in semantic analysis
- Static method validation
- Import statement parsing

### Code Generation Improvements
- Automatic global declaration insertion
- Fixed call chain processing for self.variable
- Static method call detection
- Conditional import generation

### Files Modified
- `framec/src/frame_c/scanner.rs`: Removed deprecated keywords
- `framec/src/frame_c/parser.rs`: Enhanced semantic analysis
- `framec/src/frame_c/visitors/python_visitor.rs`: Fixed code generation bugs
- `docs/framelang_design/grammar.md`: Updated language specification
- `docs/framelang_design/dev_notes.md`: Documented implementation details

## Migration Guide

### From v0.30 to v0.31

1. **Replace null/nil with None**:
   ```frame
   // Before
   var x = null
   
   // After
   var x = None
   ```

2. **Update domain variable declarations**:
   ```frame
   // Ensure all domain variables use 'var' keyword
   domain:
       var counter: int = 0  // Correct
       // counter: int = 0   // Incorrect (missing 'var')
   ```

3. **Use native imports**:
   ```frame
   // Before
   `import math`
   
   // After
   import math
   ```

4. **Leverage module variables**:
   ```frame
   // Global state accessible everywhere
   var app_state = {}
   
   fn update_state(key, value) {
       app_state[key] = value  // Automatic global handling
   }
   ```

## Performance

- **Transpilation Speed**: No measurable performance impact
- **Generated Code**: Cleaner Python output with proper global declarations
- **Runtime Performance**: Identical to v0.30

## Known Limitations

- Module variables only support Python target currently
- Static method calls require explicit system name prefix
- No support for module-level bare instantiations

## Future Roadmap

### v0.32 Plans
- Module variable support for other target languages
- Enhanced type checking
- Improved error messages
- Performance optimizations

## Contributors

This release represents significant collaborative effort in achieving 100% test coverage and fixing critical bugs in the Frame transpiler.

## Support

For issues or questions:
- GitHub Issues: [Frame Transpiler Issues](https://github.com/frame-lang/frame_transpiler/issues)
- Documentation: [Frame Language Docs](https://frame-lang.org)
- Discord: [Frame Community](https://discord.gg/frame)

---

**Frame v0.31** - Production ready with complete test coverage and enhanced language features.