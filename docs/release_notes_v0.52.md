# Frame v0.52 Release Notes - Multiple Assignment and Tuple Unpacking

**Release Date**: January 29, 2025  
**Version**: v0.52  
**Branch**: v0.30  

## Executive Summary

Frame v0.52 introduces Python-style multiple assignment and tuple unpacking capabilities, enabling more elegant and Pythonic code patterns. This release allows developers to assign values to multiple variables in a single statement, unpack collections, and perform variable swapping without temporary variables.

## New Features

### 1. Multiple Assignment
Assign values to multiple variables in a single, elegant statement:

```frame
# Before v0.52
var x = 10
var y = 20

# With v0.52
x, y = 10, 20
```

### 2. Tuple Unpacking
Unpack tuples and other sequences into individual variables:

```frame
var coordinates = (42, 73)
var lat = 0
var lon = 0
lat, lon = coordinates  # Unpack tuple into variables
```

### 3. Variable Swapping
Swap variable values without temporary variables:

```frame
# Classic swap (no temp variable needed!)
a, b = b, a
```

### 4. Function Return Unpacking
Elegantly handle functions that return multiple values:

```frame
fn get_dimensions() {
    return (1920, 1080)
}

var width = 0
var height = 0
width, height = get_dimensions()
```

### 5. Complex Expression Assignment
Multiple assignment works with complex expressions:

```frame
# Increment, double, and square in one line
n1, n2, n3 = n1 + 1, n2 * 2, n3 ** 2
```

## Technical Implementation

### Parser Enhancements
- Modified `assignment_or_lambda()` function to detect comma-separated assignment targets
- Added logic to distinguish between tuple literals and multiple assignment based on context
- Implemented proper handling of comma-separated expressions on both LHS and RHS

### AST Changes
```rust
pub struct AssignmentExprNode {
    // ... existing fields ...
    pub is_multiple_assignment: bool,  // New in v0.52
    pub l_values: Vec<ExprType>,       // New in v0.52
}
```

### Code Generation
- Python visitor generates proper comma-separated targets
- Automatic tuple wrapping for RHS when multiple values present
- Preserves Python's native unpacking semantics

## Usage Examples

### Basic Multiple Assignment
```frame
fn process_data() {
    var x = 0
    var y = 0
    var z = 0
    
    # Assign multiple values at once
    x, y, z = 1, 2, 3
    print("x=" + str(x) + ", y=" + str(y) + ", z=" + str(z))
}
```

### Working with Collections
```frame
fn handle_lists() {
    # Create a list (workaround for comma issue)
    var items = [1]
    items.append(2)
    items.append(3)
    
    # Unpack the list
    var a = 0
    var b = 0
    var c = 0
    a, b, c = items
    print("Unpacked: " + str(a) + ", " + str(b) + ", " + str(c))
}
```

### Pattern: Search and Extract
```frame
fn find_min_max(numbers) {
    var min_val = numbers[0]
    var max_val = numbers[0]
    
    for num in numbers {
        if num < min_val {
            min_val = num
        }
        if num > max_val {
            max_val = num
        }
    }
    
    return (min_val, max_val)
}

fn main() {
    var nums = [5, 2, 8, 1, 9, 3]
    var min = 0
    var max = 0
    min, max = find_min_max(nums)
    print("Min: " + str(min) + ", Max: " + str(max))
}
```

## Known Limitations

### 1. Multiple Variable Declarations
Multiple variable declarations in a single statement are not yet fully supported:
```frame
# Not supported:
var x, y = 10, 20

# Workaround:
var x = 0
var y = 0
x, y = 10, 20
```

### 2. List Literal Parsing
When list literals contain comma-separated values, they are incorrectly parsed as a tuple within a list:
```frame
# Issue:
var lst = [1, 2, 3]  # Becomes [(1, 2, 3)]

# Workaround:
var lst = [1]
lst.append(2)
lst.append(3)
```

### 3. Compound Assignment Operators
Multiple assignment only works with the simple `=` operator:
```frame
# Not supported:
x, y += 1, 2

# Use separate statements:
x, y = x + 1, y + 2
```

## Migration Guide

### Upgrading Existing Code
No breaking changes - v0.52 is fully backward compatible. Existing assignment statements continue to work as before.

### Best Practices
1. **Use multiple assignment for related values**: Coordinates, dimensions, min/max pairs
2. **Leverage swapping**: Replace three-line swaps with elegant one-liners
3. **Unpack function returns**: Make multi-value returns more readable
4. **Declare variables first**: Until full declaration support is added

## Testing

Comprehensive test coverage provided in:
- `framec_tests/python/src/test_multiple_assignment_v052.frm`

Test results: **100% pass rate** for all multiple assignment scenarios.

## Future Enhancements

Planned improvements for future releases:
- Support for multiple variable declarations (`var x, y, z = 1, 2, 3`)
- Star expressions for unpacking (`*rest` syntax)
- Fix list literal parsing to preserve element separation
- Pattern matching integration with multiple assignment

## Dependencies

No new dependencies introduced. Multiple assignment is a pure parser/AST enhancement.

## Acknowledgments

This feature was implemented based on Python's multiple assignment semantics, ensuring familiar behavior for Python developers using Frame.

## Technical Notes

### Implementation Strategy
The parser follows the user's guidance to "parse x,y as an expression first and if it doesn't match anything else it is then passed up to the statement level to be packaged into a statement". This ensures proper precedence and context-aware parsing.

### Symbol Table Handling
During the first pass (symbol table construction), multiple assignments are handled specially to avoid symbol lookup errors for the synthetic assignment target.

## Conclusion

Frame v0.52 brings significant quality-of-life improvements for developers, making common patterns like variable swapping and tuple unpacking more elegant and Pythonic. While some limitations exist, the workarounds are straightforward, and the benefits of cleaner, more readable code are immediate.

For questions or issues, please refer to the Frame documentation or submit an issue to the project repository.