# Frame Event Syntax Migration Plan

## Overview

Frame currently uses `@` for decorators/attributes (like `@staticmethod`), which aligns with Python. The `$@` syntax represents the current event reference. This document outlines the complete removal of any conflicting `@` usage to enable Python's matrix multiplication operator.

## Current State

### `@` Usage in Frame
1. **Decorators/Attributes**: `@staticmethod`, `@async` (Python-compatible) ✅
2. **Historical**: Previously used for events, now uses `$@` for current event

### `$` Prefixed Symbols
- `$` - State reference prefix
- `$@` - Current event reference  
- `$>` - Enter event
- `$<` - Exit event (alternate syntax)
- `<$` - Exit event (primary syntax)
- `$^` - Parent state reference
- `$$` - State stack operations

## No Changes Needed

After analysis, Frame's `@` usage is already Python-compatible:
- `@` is only used for decorators/attributes (Python-style)
- Event references use `$@` (Frame-specific, no conflict)
- No migration needed for existing code

## Enabling Matrix Multiplication

To support Python's matrix multiplication operator (`@`):

### Implementation Steps
1. **Scanner**: Already recognizes `@` token ✅
2. **Parser**: Add matrix multiplication to binary operators
3. **AST**: Add `MatMul` to BinaryOp enum
4. **Visitor**: Generate `@` operator in Python output
5. **Precedence**: Place between `*` and `+` (Python standard)

### Example After Implementation
```frame
fn matrix_operations() {
    var result = matrix_a @ matrix_b  // Matrix multiplication
    var transformed = rotation @ vector
    
    // Decorators still work
    @staticmethod
    fn helper() { }
    
    // Event syntax unchanged  
    var current = $@  // Current event
}
```

## Summary

- **No breaking changes required**
- Frame's current `@` usage is Python-compatible
- Can add matrix multiplication operator immediately
- Event syntax (`$@`) remains unchanged
- Decorators (`@attr`) continue to work