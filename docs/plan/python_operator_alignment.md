# Frame Python Operator Alignment Plan

**Version**: v0.38 Planning Document  
**Date**: January 2025  
**Goal**: Complete alignment of Frame operators with Python 3.x

## Executive Summary

This document outlines a comprehensive plan to align Frame's operator set with Python, ensuring maximum compatibility and minimal surprises for developers familiar with Python. The plan includes removing Frame-specific operators that conflict with Python semantics, adopting Python's operator precedence and semantics, and implementing missing Python operators.

## Part 1: Current State Analysis

### Frame's Current Operators

#### Arithmetic Operators ✅
- `+` Addition
- `-` Subtraction  
- `*` Multiplication
- `/` Division
- `%` Modulo

#### Comparison Operators ✅
- `==` Equal
- `!=` Not equal
- `<` Less than
- `<=` Less than or equal
- `>` Greater than
- `>=` Greater than or equal

#### Logical Operators ✅
- `&&` Logical AND
- `||` Logical OR (via `PipePipe` token)
- `!` Logical NOT

#### Assignment Operators ⚠️ (Partial)
- `=` Assignment
- Missing: `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `//=`, `&=`, `|=`, `^=`, `>>=`, `<<=`

#### Bitwise Operators ❌ (Missing)
- Missing: `&` (AND), `|` (OR), `^` (XOR), `~` (NOT), `<<` (left shift), `>>` (right shift)

#### Other Operators
- `@` Event reference (CONFLICTS with Python's matrix multiplication)
- `$` State reference
- `=>` Dispatch/transition
- `->` State transition

### Frame's Removed Operators (v0.30)
- `^` Return (replaced with `return` keyword)
- `^=` Return assignment (replaced with `return =`)
- Ternary operators (`?`, `?!`, `?~`, `?#`, `?:`)
- Pattern matching (`~/`, `#/`, `:/`)

## Part 2: Python Operators Inventory

### Essential Python Operators to Implement

#### 1. Power Operator ⭐ HIGH PRIORITY
- `**` Power/exponentiation
- Example: `2 ** 3` → 8
- Implementation: Add token, parser support, visitor generation

#### 2. Floor Division ⭐ HIGH PRIORITY
- `//` Integer/floor division
- Example: `7 // 2` → 3
- Implementation: Add token, parser support, visitor generation

#### 3. Augmented Assignment ⭐ HIGH PRIORITY
- `+=`, `-=`, `*=`, `/=`, `%=`
- `**=`, `//=`
- `&=`, `|=`, `^=`, `>>=`, `<<=`
- Implementation: Parser transforms to `x = x op y`

#### 4. Bitwise Operators 🔧 MEDIUM PRIORITY
- `&` Bitwise AND
- `|` Bitwise OR (conflicts with Frame's pipe)
- `^` Bitwise XOR (available since return removed)
- `~` Bitwise NOT
- `<<` Left shift
- `>>` Right shift

#### 5. Membership/Identity Operators ✅ PARTIAL
- `in` ✅ Already implemented
- `not in` ⚠️ Needs implementation
- `is` ❌ Needs implementation
- `is not` ❌ Needs implementation

#### 6. Walrus Operator 🔄 LOW PRIORITY
- `:=` Assignment expression (Python 3.8+)
- Example: `if (n := len(data)) > 10:`
- Complex implementation due to expression vs statement distinction

#### 7. Matrix Multiplication ⚠️ CONFLICT
- `@` Matrix multiplication operator
- CONFLICTS with Frame's event reference syntax
- Resolution: Remove `@` for events, use `$@` or explicit syntax

## Part 3: Conditional Operator Analysis

### Current Frame Conditionals
```frame
if condition {
    // true block
} elif other_condition {
    // elif block
} else {
    // else block
}
```

### Issues with Current Implementation
1. **No inline conditional expression** (Python's ternary)
2. **Logical operators use C-style** (`&&`, `||`) not Python (`and`, `or`)
3. **Boolean negation uses `!`** not Python's `not`

### Proposed Changes

#### 1. Add Python's Ternary Expression
```frame
// Python: value = true_val if condition else false_val
var value = true_val if condition else false_val
```
Implementation: Parse as special expression form, not statement

#### 2. Support Python Logical Keywords
```frame
// Option A: Support both styles
if x && y { }  // C-style (current)
if x and y { } // Python-style (new)

// Option B: Deprecate C-style, adopt Python
if x and y { }      // Python style only
if not x { }        // Python negation
if x or y { }       // Python OR
```

#### 3. Chained Comparisons
```frame
// Python: if 0 < x < 10:
if 0 < x < 10 { }  // Should work in Frame
```

## Part 4: Implementation Roadmap

### Phase 1: Remove Conflicting Operators (v0.38.1)
1. **Remove `@` as event syntax**
   - Update scanner to not treat `@` specially for events
   - Introduce `$@` for current event (already exists)
   - Update all tests and documentation

2. **Free up `^` for bitwise XOR** ✅ Already done
   - Confirmed: `^` removed in v0.30

### Phase 2: Core Python Operators (v0.38.2)
1. **Implement `**` (power)**
   - Add `TokenType::StarStar`
   - Update expression parser for right-associativity
   - Generate appropriate code in visitors

2. **Implement `//` (floor division)**
   - Add `TokenType::SlashSlash`
   - Update expression parser
   - Generate appropriate code in visitors

3. **Implement augmented assignments**
   - Add tokens: `+=`, `-=`, `*=`, `/=`, etc.
   - Parser desugars to `x = x op y`
   - No AST changes needed

### Phase 3: Bitwise Operators (v0.38.3)
1. **Implement bitwise operators**
   - `&` (AND) - Note: Already have `TokenType::Ampersand`
   - `|` (OR) - Conflicts need resolution
   - `^` (XOR) - Now available
   - `~` (NOT) - Add token
   - `<<`, `>>` (shifts) - Add tokens

2. **Resolve `|` conflict**
   - Currently used for event handlers (removed)
   - Can now be used for bitwise OR

### Phase 4: Logical Keywords (v0.38.4)
1. **Add Python logical keywords**
   - `and` keyword (alongside `&&`)
   - `or` keyword (alongside `||`)
   - `not` keyword (alongside `!`)

2. **Add identity operators**
   - `is` keyword
   - `is not` combination

### Phase 5: Advanced Features (v0.38.5)
1. **Ternary expression**
   - `expr if condition else expr`
   - Complex parser changes required

2. **Walrus operator**
   - `:=` assignment expression
   - Requires expression/statement distinction

3. **Chained comparisons**
   - `0 < x < 10` support
   - Parser needs special handling

## Part 5: Collection Literal Support

### Dictionary Literals (Priority 1)
```frame
var dict = {"key1": "value1", "key2": 42}
var empty_dict = {}
```

### Set Literals (Priority 2)
```frame
var set = {1, 2, 3}
// Note: {} is empty dict, not empty set
var empty_set = set()  // Constructor function
```

### Tuple Literals (Priority 3)
```frame
var tuple = (1, 2, 3)
var single = (1,)  // Trailing comma for single element
```

### List Comprehensions (Future)
```frame
var squares = [x**2 for x in range(10)]
var filtered = [x for x in data if x > 0]
```

## Part 6: Operator Precedence Table

Frame should adopt Python's precedence (highest to lowest):

1. `()` Parentheses
2. `**` Exponentiation
3. `+x`, `-x`, `~x` Unary operators
4. `*`, `/`, `//`, `%` Multiplicative
5. `+`, `-` Additive
6. `<<`, `>>` Bitwise shifts
7. `&` Bitwise AND
8. `^` Bitwise XOR
9. `|` Bitwise OR
10. `<`, `<=`, `>`, `>=`, `!=`, `==` Comparisons
11. `not` Boolean NOT
12. `and` Boolean AND
13. `or` Boolean OR
14. `if else` Ternary conditional
15. `:=` Walrus operator
16. `=`, `+=`, `-=`, etc. Assignment

## Part 7: Breaking Changes Summary

### Removals
1. `@` for event references → Use explicit event handler syntax
2. C-style logical operators (optional) → Use Python keywords

### Conflicts to Resolve
1. `@` operator: Remove from events, enable for matrix multiplication
2. `|` usage: Clarify bitwise OR vs any Frame-specific usage

### Migration Path
1. Provide deprecation warnings in v0.38.0
2. Remove deprecated syntax in v0.39.0
3. Full Python alignment in v0.40.0

## Part 8: Implementation Checklist

### Scanner Updates
- [ ] Add `**` token (StarStar)
- [ ] Add `//` token (SlashSlash)
- [ ] Add `~` token (Tilde)
- [ ] Add `<<` token (LeftShift)
- [ ] Add `>>` token (RightShift)
- [ ] Add augmented assignment tokens
- [ ] Add `and`, `or`, `not` keywords
- [ ] Add `is` keyword
- [ ] Remove special `@` handling for events

### Parser Updates
- [ ] Implement power operator with right associativity
- [ ] Implement floor division
- [ ] Implement augmented assignments (desugar)
- [ ] Implement bitwise operators
- [ ] Implement logical keywords
- [ ] Implement identity operators
- [ ] Implement ternary expression
- [ ] Implement dictionary literals
- [ ] Implement set literals
- [ ] Update operator precedence

### AST Updates
- [ ] Add BinaryOp variants for new operators
- [ ] Add DictLiteralNode
- [ ] Add SetLiteralNode
- [ ] Add TernaryExprNode

### Visitor Updates
- [ ] Generate Python code for all new operators
- [ ] Handle dictionary/set literals
- [ ] Handle ternary expressions

### Documentation Updates
- [ ] Update grammar.md
- [ ] Update operator reference
- [ ] Create migration guide
- [ ] Update all examples

### Test Suite
- [ ] Test each new operator
- [ ] Test precedence
- [ ] Test edge cases
- [ ] Migration tests

## Appendix A: Quick Reference Comparison

| Operation | Python | Current Frame | Proposed Frame |
|-----------|--------|---------------|----------------|
| Power | `x ** y` | N/A | `x ** y` |
| Floor div | `x // y` | N/A | `x // y` |
| Augmented | `x += 1` | N/A | `x += 1` |
| Bitwise AND | `x & y` | N/A | `x & y` |
| Bitwise OR | `x \| y` | N/A | `x \| y` |
| Bitwise XOR | `x ^ y` | N/A | `x ^ y` |
| Bitwise NOT | `~x` | N/A | `~x` |
| Left shift | `x << y` | N/A | `x << y` |
| Right shift | `x >> y` | N/A | `x >> y` |
| Logical AND | `x and y` | `x && y` | Both |
| Logical OR | `x or y` | `x \|\| y` | Both |
| Logical NOT | `not x` | `!x` | Both |
| Identity | `x is y` | N/A | `x is y` |
| Membership | `x in y` | `x in y` | `x in y` |
| Ternary | `a if c else b` | N/A | `a if c else b` |
| Matrix mult | `A @ B` | `@` (events) | `A @ B` |
| Walrus | `x := expr` | N/A | `x := expr` |
| Dict literal | `{"a": 1}` | N/A | `{"a": 1}` |
| Set literal | `{1, 2}` | N/A | `{1, 2}` |

## Appendix B: Examples After Implementation

```frame
// All Python operators working in Frame
fn python_operators_demo() {
    // Arithmetic
    var power = 2 ** 8          // 256
    var floor = 7 // 2          // 3
    
    // Augmented assignment
    var x = 10
    x += 5                      // x = 15
    x **= 2                     // x = 225
    
    // Bitwise
    var flags = 0b1010 & 0b1100  // 0b1000
    var mask = ~0b1111           // -16
    var shifted = 1 << 4         // 16
    
    // Logical (both styles)
    if x > 10 and y < 20 { }    // Python style
    if x > 10 && y < 20 { }     // C style (compatibility)
    
    // Identity and membership
    if obj is None { }
    if item in collection { }
    if item not in collection { }
    
    // Ternary expression
    var result = "pass" if score >= 60 else "fail"
    
    // Collections
    var dict = {"name": "Frame", "version": "0.38"}
    var nums = {1, 2, 3}  // set
    var coord = (10, 20)  // tuple
    
    // Walrus operator (advanced)
    if (n := len(data)) > 10 {
        print("Large dataset: " + str(n))
    }
    
    // Matrix multiplication (after @ freed up)
    var result = matrix1 @ matrix2
}
```

## Conclusion

This comprehensive plan aligns Frame with Python's operator set while preserving Frame's unique state machine features. The phased approach ensures backward compatibility during migration while moving toward full Python operator support. Priority is given to commonly used operators that provide immediate value to developers.

The removal of `@` for events is necessary to avoid confusion with Python's matrix multiplication operator. Alternative syntaxes like `$@` or explicit event handler declarations are clearer and more consistent with Frame's design philosophy.

Implementing this plan will make Frame more accessible to Python developers and ensure generated Python code uses idiomatic operators.