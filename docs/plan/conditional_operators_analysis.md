# Frame Conditional Operators - Current State & Proposals

## Current State (v0.37)

### What Frame Has Now

#### Basic Conditionals ✅
```frame
if condition {
    // true branch
} elif other_condition {
    // elif branch  
} else {
    // else branch
}
```

#### Logical Operators (C-style) ✅
- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT

#### Comparison Operators ✅
- `==`, `!=` - Equality
- `<`, `<=`, `>`, `>=` - Relational

### What Frame Removed (v0.30)

#### All Ternary Operators ❌
- `?` - Boolean test (REMOVED)
- `?!` - Boolean test false (REMOVED)
- `?~` - String match test (REMOVED) 
- `?#` - Number match test (REMOVED)
- `?:` - Enum match test (REMOVED)
- `:|` and `::` - Test terminators (REMOVED)

## Problems with Current Implementation

1. **No inline conditional expressions**
   - Cannot write: `var x = condition ? true_val : false_val`
   - Must use full if/else blocks even for simple assignments

2. **C-style operators in Python context**
   - Frame uses `&&`, `||`, `!`
   - Python uses `and`, `or`, `not`
   - Generated Python code doesn't look idiomatic

3. **No chained comparisons**
   - Cannot write: `if 0 < x < 10`
   - Must write: `if x > 0 && x < 10`

## Proposed Changes

### Priority 1: Python Ternary Expression

**Add Python's conditional expression syntax:**
```frame
// Proposed Frame syntax (matches Python exactly)
var status = "pass" if score >= 60 else "fail"
var max_val = a if a > b else b
```

**Benefits:**
- Concise for simple conditionals
- Direct mapping to Python
- Familiar to Python developers

**Implementation:**
- Parser: Recognize `expr if condition else expr` pattern
- AST: Add TernaryExprNode
- Visitor: Generate Python ternary directly

### Priority 2: Python Logical Keywords

**Support Python keywords alongside C-style:**
```frame
// Both styles supported (backwards compatible)
if x && y { }      // C-style (current)
if x and y { }     // Python-style (new)

if !ready { }      // C-style (current)
if not ready { }   // Python-style (new)

if a || b { }      // C-style (current)
if a or b { }      // Python-style (new)
```

**Benefits:**
- More Pythonic code
- Better readability
- Backward compatibility maintained

**Implementation:**
- Scanner: Add `and`, `or`, `not` as keywords
- Parser: Treat them as aliases for `&&`, `||`, `!`
- No AST changes needed

### Priority 3: Chained Comparisons

**Enable Python-style chained comparisons:**
```frame
// Proposed syntax
if 0 < x < 10 { }
if 10 <= age <= 65 { }
if a < b <= c < d { }
```

**Benefits:**
- More readable
- Natural mathematical notation
- Direct Python mapping

**Implementation:**
- Parser: Detect comparison chains
- Transform to: `(0 < x) and (x < 10)`
- Preserve comparison semantics

### Priority 4: Identity Operators

**Add Python's identity operators:**
```frame
// Check identity (not just equality)
if obj is None { }
if obj is not None { }
if x is y { }  // Same object
```

**Benefits:**
- Proper None checking
- Object identity testing
- Python compatibility

## Migration Strategy

### Phase 1 (v0.38.0)
- Add Python ternary expression
- Add `and`, `or`, `not` keywords
- Maintain backward compatibility

### Phase 2 (v0.39.0)
- Add chained comparisons
- Add identity operators
- Deprecation warnings for C-style (optional)

### Phase 3 (v0.40.0)
- Consider making Python-style the default
- C-style becomes optional/deprecated

## Comparison Table

| Feature | Current Frame | Python | Proposed Frame |
|---------|--------------|--------|----------------|
| If/else blocks | ✅ | ✅ | ✅ |
| Ternary expression | ❌ | `a if c else b` | `a if c else b` |
| Logical AND | `&&` | `and` | Both |
| Logical OR | `\|\|` | `or` | Both |
| Logical NOT | `!` | `not` | Both |
| Chained comparisons | ❌ | `0 < x < 10` | `0 < x < 10` |
| Identity test | ❌ | `is`, `is not` | `is`, `is not` |

## Code Examples After Implementation

```frame
fn improved_conditionals() {
    // Python ternary
    var status = "active" if user.logged_in else "inactive"
    
    // Python logical keywords
    if user.verified and not user.banned {
        grant_access()
    }
    
    // Chained comparisons
    if 18 <= age < 65 {
        full_price()
    }
    
    // Identity checks
    if result is None {
        result = compute_default()
    }
    
    // Mixed styles (backward compatible)
    if (x > 0 && y > 0) or override {
        process()
    }
}
```

## Summary

Frame's conditional operators need modernization to align with Python. The removal of ternary operators in v0.30 was a good simplification, but we should add Python's more elegant conditional expression syntax. Supporting Python logical keywords will make Frame code more readable and the generated Python more idiomatic. These changes maintain backward compatibility while moving Frame toward Python alignment.