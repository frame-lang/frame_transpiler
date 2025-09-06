# Frame Parser Limitations (v0.37)

## Issue: Method Calls on Object Properties

### Problem
The parser cannot handle method calls on object properties. For example:
- `self.processed_data.append(item)` - FAILS
- `self.batch_data.length` - FAILS  

### Current Behavior
When the parser encounters `self.property.method()`, it fails with:
```
Error at '.' : Expected '}'
```

### Root Cause
The parser's call chain building logic doesn't properly handle:
1. Chained property access followed by method calls
2. FSL properties like `.length` on complex expressions
3. FSL methods like `.append()` on object properties

### Workaround
Currently, backticks must be used:
```frame
// Instead of:
self.processed_data.append(item)

// Use:
`self.processed_data.append(item)`

// Instead of:
self.batch_data.length

// Use:
`len(self.batch_data)`
```

### Fix Required
The parser needs to be enhanced to:
1. Build proper call chains for `object.property.method()` patterns
2. Recognize FSL operations (append, length, etc.) in chained contexts
3. Handle property access on complex expressions

### Related Code
- Parser call chain building: `framec/src/frame_c/parser.rs` lines 7960-8918
- FSL operation recognition: Lines 9040-9100
- Issue occurs during first pass (symbol table building)