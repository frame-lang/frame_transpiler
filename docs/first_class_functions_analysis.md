# First-Class Functions Analysis for Frame v0.38

**Date**: 2025-09-07  
**Author**: Analysis of current implementation

## Executive Summary

Frame v0.38 has **90% of first-class function support already working** for lambdas. The missing 10% is regular function references. This document analyzes the current state and what's needed for complete support.

## Current State: What Works ✅

### 1. Lambda Expressions as Values
```frame
var square = lambda x: x * x       // ✅ Works
var another = square                // ✅ Works - lambdas can be reassigned
var result = another(5)             // ✅ Works - called through variable
```

### 2. Lambdas as Parameters
```frame
fn apply(op, x, y) {
    return op(x, y)                 // ✅ Works - lambdas callable as params
}

var add = lambda a, b: a + b
var result = apply(add, 10, 5)      // ✅ Works - outputs 15
```

### 3. Lambdas in Collections
```frame
var ops = {
    "add": lambda a, b: a + b,       // ✅ Works
    "mul": lambda a, b: a * b        // ✅ Works
}
var result = ops["add"](5, 3)       // ✅ Works - outputs 8
```

### 4. Lambdas Returned from Functions
```frame
fn make_adder(n) {
    return lambda x: x + n          // ✅ Works - lambdas can be returned
}
var add5 = make_adder(5)
var result = add5(10)                // ✅ Works - outputs 15
```

## What Doesn't Work ❌

### Regular Function References
```frame
fn add(a, b) { return a + b }
var func = add                      // ❌ Error: "unknown scope identifier"
apply(add, 10, 5)                   // ❌ Error: function names not values
```

## Why This Happens

### Symbol Table Architecture

1. **Functions are FunctionScope symbols** - They exist in the symbol table but aren't treated as values
2. **Lambdas are expressions** - They're LambdaExprNode in the AST, treated as values from the start
3. **Parser distinction** - When parser sees an identifier:
   - If it's a variable: treated as value expression
   - If it's a function: only valid in call position `func()`

### Code Generation

The Python visitor generates:
- Lambdas: `lambda x: x * x` - Python expression, assignable
- Functions: `def func():` - Python statement, not directly assignable

## Do We Need Changes?

### For Python Target: **NO CHANGES NEEDED**

Since Frame transpiles to Python and:
1. **Lambdas already work** as first-class values
2. **Python handles the runtime** correctly
3. **Generated code is valid Python**

The transpiler doesn't need to track function types in the AST because Python does it at runtime.

### What We'd Need for Function References

If we wanted `var f = add` to work:

1. **Parser Change**: Recognize function names as valid value expressions
   ```rust
   // In parser.rs primary_expression()
   if let Some(func_symbol) = lookup_function(identifier) {
       return FunctionRefExpr { name: identifier }
   }
   ```

2. **AST Node**: Add FunctionRefNode for function references
   ```rust
   pub struct FunctionRefNode {
       pub name: String,
       pub symbol: Rc<RefCell<FunctionSymbol>>
   }
   ```

3. **Python Visitor**: Generate function name without parentheses
   ```rust
   fn visit_function_ref(&mut self, node: &FunctionRefNode) {
       self.add_code(&node.name);  // Just the name, not a call
   }
   ```

## Value Proposition

### Current Benefits (Already Working)
- ✅ **Functional programming** with lambdas
- ✅ **Higher-order functions** (map, filter, reduce patterns)
- ✅ **Callbacks and event handlers**
- ✅ **Strategy pattern** implementations
- ✅ **Dynamic dispatch** through dictionaries

### Additional Benefits of Full Support
- 🔄 **Consistency**: Functions and lambdas behave the same
- 🔄 **Refactoring**: Convert between lambda and function easily
- 🔄 **Testing**: Pass mock functions for testing
- 🔄 **Decorators**: Foundation for Python-style decorators

## Recommendation

### Don't Change Anything (Yet)

**Rationale**:
1. **Lambdas cover 90% of use cases** - Most first-class function needs are met
2. **Python compatibility** - Generated code already works correctly
3. **Simplicity** - Current distinction is clear: functions for named reusable code, lambdas for values
4. **Frame's focus** - State machines are the priority, not functional programming

### Future Enhancement (v0.40?)

If demand exists, add function references as a minor feature:
- Small parser change (~20 lines)
- One new AST node type
- Simple visitor update
- No breaking changes

### Alternative: Documentation

Instead of implementation, document the pattern:
```frame
// Instead of this (doesn't work):
fn add(a, b) { return a + b }
var f = add

// Use this (works today):
var add = lambda a, b: a + b
var f = add
```

## Conclusion

Frame v0.38 already has **working first-class functions through lambdas**. The only missing piece is regular function references, which:
- Is a nice-to-have, not essential
- Can be worked around with lambdas
- Would be easy to add if needed

The current implementation is **sufficient for Frame's goals** as a state machine language with Python as the target. The lambda support enables functional programming patterns while keeping the language simple and focused.