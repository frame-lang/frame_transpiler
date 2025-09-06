# Frame Syntax Removal Plan - Aligning with Python

**Version**: v0.38 Planning Document  
**Date**: January 2025  
**Goal**: Remove Frame-specific syntax that duplicates Python functionality

## Executive Summary

Frame should not maintain its own syntax for features that Python already provides well. This document identifies Frame-specific syntax that should be deprecated and removed in favor of Python's standard approach.

## Part 1: Syntax to Remove

### 1. C-Style Logical Operators ❌ DEPRECATE

**Current Frame Syntax:**
```frame
if x && y { }       // C-style AND
if x || y { }       // C-style OR  
if !x { }           // C-style NOT
```

**Replace With Python:**
```frame
if x and y { }      // Python AND
if x or y { }       // Python OR
if not x { }        // Python NOT
```

**Migration Strategy:**
- v0.38: Add Python keywords, deprecate C-style with warnings
- v0.39: C-style becomes error
- Scanner: Keep tokens for error messages during transition

### 2. Backtick Expressions ❌ REMOVE

**Current Frame Syntax:**
```frame
var result = `complex_python_expr`
self.dict`[key]` = value
```

**Replace With Native Support:**
```frame
var result = complex_python_expr  // Parse it properly
self.dict[key] = value            // Full index support
```

**Required Implementations:**
- Dictionary literals: `{"key": value}`
- Full indexing: `dict[key] = value`
- Method chaining: `obj.method()[0]`
- Complex expressions in parser

### 3. Frame-Specific Null ❌ STANDARDIZED (Already Done)

**Old Frame (Removed in v0.31):**
```frame
var x = null  // REMOVED
var y = nil   // REMOVED
```

**Current (Python Standard):**
```frame
var x = None  // ✅ Python standard
```

### 4. Frame Variable Declaration ⚠️ CONSIDER

**Current Frame:**
```frame
var x = 5
const PI = 3.14
```

**Pure Python Would Be:**
```frame
x = 5
PI = 3.14  # Convention: UPPERCASE for constants
```

**Recommendation:** KEEP `var`/`const` for now
- Provides useful declaration intent
- Helps Frame's semantic analysis
- Clear scope introduction
- Not in conflict with Python (additive)

### 5. Frame Loop Syntax ⚠️ ANALYZE

**Current Frame:**
```frame
loop {           // Infinite loop
    break
}
```

**Python Equivalent:**
```frame
while True {     // More standard
    break
}
```

**Recommendation:** DEPRECATE `loop`
- `while True` is clearer and standard
- One less keyword to maintain
- Python developers expect `while`

## Part 2: Syntax Conflicts to Resolve

### 1. Block Delimiters: `{}` vs Python Indentation

**Current Frame:**
```frame
if x > 0 {
    do_something()
}
```

**Python Uses:**
```python
if x > 0:
    do_something()
```

**Resolution:** KEEP `{}`
- Core Frame design decision
- Enables features Python doesn't have (state machines)
- Not really "competing" - different paradigm
- Generated Python uses proper indentation

### 2. String Interpolation

**Frame Currently Lacks:**
```frame
// Not supported:
var msg = f"Hello {name}"  // Python f-strings
var msg = "Hello " + name   // Works but verbose
```

**Should Add:**
- Python f-string support
- Makes Frame more Pythonic
- Very useful feature

## Part 3: Frame-Unique Syntax to Preserve

These are Frame innovations that don't conflict with Python:

### ✅ State Machine Syntax
```frame
system Name { }
$State { }
-> $NextState
=> $^
```
**Keep:** Core Frame value proposition

### ✅ Event Handlers
```frame
eventName(params) { }
$>() { }  // Enter
<$() { }  // Exit
```
**Keep:** Frame's domain-specific features

### ✅ Interface/Machine/Actions Blocks
```frame
interface:
machine:
actions:
operations:
domain:
```
**Keep:** Frame's organization structure

### ✅ Frame Event References
```frame
$@  // Current event
$^  // Parent state
$$  // State stack
```
**Keep:** State machine specific

## Part 4: Implementation Priority

### Phase 1: Critical Removals (v0.38)
1. **Deprecate C-style logical operators** (`&&`, `||`, `!`)
2. **Add Python logical keywords** (`and`, `or`, `not`)
3. **Deprecate `loop` keyword** (use `while True`)

### Phase 2: Enhanced Expression Support (v0.39)
1. **Remove backtick requirement** by implementing:
   - Dictionary literals
   - Full indexing support
   - Method chaining
   - Complex expression parsing

### Phase 3: Python Feature Parity (v0.40)
1. **Add f-strings** for string interpolation
2. **Add remaining Python operators**
3. **Full Python expression compatibility**

## Part 5: Migration Examples

### Before (Current Frame):
```frame
system Example {
    machine:
        $Ready {
            process(data) {
                if data && !self.busy || override {
                    var result = `json.dumps(data)`
                    self.cache`[str(id)]` = result
                    loop {
                        if condition {
                            break
                        }
                    }
                }
            }
        }
}
```

### After (Aligned with Python):
```frame
system Example {
    machine:
        $Ready {
            process(data) {
                if data and not self.busy or override {
                    var result = json.dumps(data)
                    self.cache[str(id)] = result
                    while True {
                        if condition {
                            break
                        }
                    }
                }
            }
        }
}
```

## Part 6: Benefits of Removal

1. **Reduced Cognitive Load**: One way to do things
2. **Better Python Integration**: Frame code looks more like Python
3. **Simpler Parser**: Fewer special cases
4. **Easier Learning Curve**: Python developers feel at home
5. **Cleaner Documentation**: No need to explain two ways
6. **Less Maintenance**: Fewer features to support

## Part 7: Compatibility Strategy

### Deprecation Warnings (v0.38)
```
Warning: '&&' is deprecated. Use 'and' instead.
Warning: 'loop' is deprecated. Use 'while True' instead.
```

### Migration Tool
Create `frame-migrate` tool:
```bash
frame-migrate --from=v037 --to=v038 mycode.frm
```

Automatically converts:
- `&&` → `and`
- `||` → `or`
- `!` → `not`
- `loop` → `while True`

### Gradual Transition
1. **v0.38**: Warnings only, both syntaxes work
2. **v0.39**: Old syntax generates errors
3. **v0.40**: Old tokens removed from scanner

## Part 8: Summary Table

| Feature | Current Frame | Keep/Remove | Replacement |
|---------|--------------|-------------|-------------|
| Logical AND | `&&` | ❌ Remove | `and` |
| Logical OR | `\|\|` | ❌ Remove | `or` |
| Logical NOT | `!` | ❌ Remove | `not` |
| Backticks | `` `expr` `` | ❌ Remove | Native parsing |
| Infinite loop | `loop` | ❌ Remove | `while True` |
| Null values | `null`, `nil` | ✅ Removed | `None` |
| Variables | `var`/`const` | ✅ Keep | Useful intent |
| Blocks | `{ }` | ✅ Keep | Core design |
| State syntax | `$State` | ✅ Keep | Frame unique |
| Events | `$>`, `<$` | ✅ Keep | Frame unique |
| Transitions | `->` | ✅ Keep | Frame unique |

## Conclusion

Removing Frame-specific syntax that duplicates Python functionality will make Frame cleaner, more maintainable, and more approachable for Python developers. The key is to remove redundant general-purpose syntax while preserving Frame's domain-specific innovations for state machines.

The migration should be gradual with clear deprecation warnings and tool support to help users transition their code.