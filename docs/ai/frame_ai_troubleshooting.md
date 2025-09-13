# Frame AI Troubleshooting Guide
*Common Issues and Solutions for AI-Generated Frame Code*

## 🚨 Most Common Errors

### 1. ❌ Using Old/Wrong Syntax

#### Problem: C-Style Logical Operators
```frame
# WRONG - Will fail
if x && y {  # Error: Expected identifier, found '&&'
    action()
}
```

#### Solution: Use Python Operators
```frame
# CORRECT
if x and y {
    action()
}
```

#### Problem: Old Return Syntax
```frame
# WRONG - Will fail
$State {
    event() {
        ^(42)  # Error: Unexpected token '^'
    }
}
```

#### Solution: Modern Return
```frame
# CORRECT
$State {
    event() {
        system.return = 42
        return
    }
}
```

### 2. ❌ State Name Errors

#### Problem: Missing $ Prefix
```frame
# WRONG - Will fail
machine:
    Active {  # Error: Expected state name starting with $
        event() -> Next
    }
```

#### Solution: Use $ Prefix
```frame
# CORRECT
machine:
    $Active {
        event() -> $Next
    }
```

### 3. ❌ Event Handler Syntax

#### Problem: Old Pipe Syntax
```frame
# WRONG - Will fail
$State {
    |click| -> $Next  # Error: Expected identifier
}
```

#### Solution: Parentheses Syntax
```frame
# CORRECT
$State {
    click() -> $Next
}
```

### 4. ❌ Block Order Violations

#### Problem: Wrong Block Order
```frame
# WRONG - Will fail
system Example {
    domain:      # Error: Blocks must be in order
        var x = 0
    
    interface:   # Too late!
        method()
}
```

#### Solution: Correct Order
```frame
# CORRECT
system Example {
    interface:   # 1st
        method()
        
    domain:      # Last
        var x = 0
}
```

### 5. ❌ Null/None Confusion

#### Problem: Using null or nil
```frame
# WRONG - Will fail
var x = null  # Error: Unknown identifier 'null'
var y = nil   # Error: Unknown identifier 'nil'
```

#### Solution: Use None
```frame
# CORRECT
var x = None
var y = None
```

## 🔍 Parse Errors

### Expected 'import' after module name
**Symptom**: `Expected 'import' after module name`

**Cause**: Confusing Python and Frame import syntax

```frame
# WRONG - Mixing syntaxes
from Utils import something  # Frame file imports don't use 'from X import Y'
```

**Fix**:
```frame
# For Python modules
from datetime import datetime

# For Frame files
import Utils from "./utils.frm"
```

### Expected string literal file path after 'from'
**Symptom**: `Expected string literal file path after 'from'`

**Cause**: Frame import expects quoted path

```frame
# WRONG
import Utils from utils.frm  # Missing quotes
```

**Fix**:
```frame
# CORRECT
import Utils from "./utils.frm"  # Quoted path
```

### Symbol table construction failed
**Symptom**: `Symbol table construction failed. Please check your Frame syntax.`

**Common Causes**:
1. Using undefined variables
2. Calling undefined methods
3. Syntax errors in expressions
4. Missing state definitions

**Debug Steps**:
1. Check all variable declarations
2. Verify all method calls exist
3. Ensure states referenced in transitions exist
4. Look for typos in identifiers

## 🔧 Runtime Errors

### AttributeError: 'NoneType' object has no attribute
**Cause**: Accessing attributes on None

```frame
# PROBLEMATIC
$State {
    process() {
        var result = self.data.value  # If self.data is None
    }
}
```

**Fix**:
```frame
# SAFE
$State {
    process() {
        if self.data != None {
            var result = self.data.value
        }
    }
}
```

### NameError: name 'X' is not defined
**Cause**: Using undefined identifier

```frame
# WRONG
fn calculate() {
    return unknown_var * 2  # unknown_var not defined
}
```

**Fix**:
```frame
# CORRECT
fn calculate() {
    var unknown_var = 10  # Define before use
    return unknown_var * 2
}
```

### RecursionError: maximum recursion depth exceeded
**Cause**: Infinite state transitions

```frame
# PROBLEMATIC
$State1 {
    $>() {
        -> $State2  # Immediate transition
    }
}

$State2 {
    $>() {
        -> $State1  # Creates infinite loop
    }
}
```

**Fix**:
```frame
# CORRECT
$State1 {
    event() -> $State2  # Transition on event
}

$State2 {
    event() -> $State1  # Not automatic
}
```

## 📋 Validation Checklist

### Before Transpilation
- [ ] All states start with `$`
- [ ] All events use `()` syntax
- [ ] Using `and`/`or`/`not` (not `&&`/`||`/`!`)
- [ ] Using `None` (not `null`/`nil`)
- [ ] Using `#` for comments (not `//`)
- [ ] Blocks in correct order
- [ ] All referenced states exist
- [ ] All called methods exist
- [ ] All variables declared before use

### Common Patterns to Avoid

#### Don't Put Code After Transitions
```frame
# WRONG
$State {
    event() {
        -> $Next
        cleanup()  # Unreachable!
    }
}

# CORRECT
$State {
    event() {
        cleanup()
        -> $Next
    }
}
```

#### Don't Use self in Static Methods
```frame
# WRONG
@staticmethod
fn static_func() {
    self.value = 42  # No self in static!
}

# CORRECT
@staticmethod  
fn static_func() {
    return 42  # No self reference
}
```

#### Don't Forget Enter/Exit Handlers
```frame
# INCOMPLETE
$State {
    event() -> $Next
    # What about cleanup?
}

# COMPLETE
$State {
    event() -> $Next
    
    <$() {
        self.cleanup()  # Exit handler for cleanup
    }
}
```

## 🎯 Quick Fixes

### Convert Old to New Syntax

| Old Syntax | New Syntax | Notes |
|------------|------------|-------|
| `^(value)` | `return value` | Return with value |
| `^` | `return` | Return without value |
| `^= value` | `system.return = value` | Interface return |
| `\|event\|` | `event()` | Event handler |
| `&&` | `and` | Logical AND |
| `\|\|` | `or` | Logical OR |
| `!` | `not` | Logical NOT |
| `null`/`nil` | `None` | Null value |
| `//` comment | `#` comment | Single-line comment |
| `#System ##` | `system Name { }` | System declaration |

### State Machine Fixes

#### Missing Initial State
```frame
# PROBLEM: No entry point
system Example {
    machine:
        $State1 { }
        $State2 { }
}

# SOLUTION: Add initial transition
system Example {
    machine:
        $Initial {
            $>() {
                -> $State1  # Define entry
            }
        }
        $State1 { }
        $State2 { }
}
```

#### Unreachable States
```frame
# PROBLEM: $Orphan never reached
machine:
    $Start {
        go() -> $End
    }
    $End { }
    $Orphan { }  # Never reached!

# SOLUTION: Add transition
machine:
    $Start {
        go() -> $End
        special() -> $Orphan
    }
    $End { }
    $Orphan { }
```

## 🔄 Migration Guide

### From v0.11 to Current

```frame
# OLD v0.11
#StateMachine
    -interface-
        method [x]
    -machine-
        $State
            |event| [params] {
                x = params["x"]
                ^(x)
            }
##

# NEW v0.57
system StateMachine {
    interface:
        method(x)
    
    machine:
        $State {
            event(params) {
                var x = params["x"]
                system.return = x
                return
            }
        }
}
```

### Python Import to Frame Import

```frame
# If importing Python module
import math  # No change needed

# If importing Frame file
# OLD: Would require backticks or workarounds
# NEW: Direct Frame import
import MyModule from "./my_module.frm"
```

## 🛠️ Debugging Tips

### Enable Debug Output
```bash
# Set environment variable
export FRAME_TRANSPILER_DEBUG=1
./framec -l python_3 file.frm
```

### Check Generated Python
Look at the generated Python to understand:
- How states become methods
- How events become function calls
- How domain variables become instance variables

### Common Debug Patterns

#### Trace State Transitions
```frame
$State {
    $>() {
        print(f"Entered {self.__state.__name__}")
    }
    
    <$() {
        print(f"Exiting {self.__state.__name__}")
    }
}
```

#### Log Event Handling
```frame
$State {
    event(param) {
        print(f"Handling event with param: {param}")
        -> $Next
    }
}
```

## 📊 Error Message Decoder

| Error Message | Likely Cause | Solution |
|--------------|--------------|----------|
| "Expected identifier" | Wrong operator or syntax | Check for &&, \|\|, ! |
| "Unknown identifier" | Undefined variable/function | Declare before use |
| "Expected state name" | Missing $ prefix | Add $ to state names |
| "Blocks must be in order" | Wrong block sequence | Reorder blocks |
| "Expected 'import'" | Malformed import | Check import syntax |
| "Symbol table construction failed" | Various syntax errors | Check all identifiers |
| "Parse error" | Syntax violation | Review nearby syntax |

## 🎓 Best Practices for AI Code Generation

1. **Start Simple**: Generate minimal working code first
2. **Validate Syntax**: Check against grammar rules
3. **Use Templates**: Base on working patterns
4. **Test Incrementally**: Add features one at a time
5. **Check Versions**: Ensure features exist in target version
6. **Follow Patterns**: Use established Frame patterns
7. **Verify Transitions**: Ensure all states are reachable
8. **Handle Edge Cases**: Add enter/exit handlers
9. **Document Intent**: Use comments to explain logic
10. **Test Generated Code**: Always transpile and run

---

*This troubleshooting guide helps AI systems identify and fix common issues in Frame code generation. Regular updates ensure compatibility with the latest Frame version.*