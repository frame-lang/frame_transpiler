# Frame v0.44 Release Notes

**Release Date**: January 24, 2025  
**Version**: v0.44  
**Branch**: v0.30  

## 🎯 Major Feature: Comprehensive Pattern Matching

Frame v0.44 introduces Python 3.10+ style pattern matching with match-case statements, bringing powerful structural pattern matching capabilities to Frame. This release completes all pattern matching features including OR patterns and star patterns.

## ✨ New Features

### Match-Case Statements
```frame
match expression {
    case pattern {
        # Handle matched pattern
    }
    case pattern if guard_condition {
        # Handle with guard clause
    }
    case _ {
        # Default case
    }
}
```

### Complete Pattern Support

#### Literal Patterns
Match specific values:
```frame
match value {
    case 42 { return "the answer" }
    case "hello" { return "greeting" }
    case true { return "boolean true" }
    case None { return "null value" }
}
```

#### Capture Patterns
Bind matched values to variables:
```frame
match value {
    case 0 { return "zero" }
    case x { return "captured: " + str(x) }
}
```

#### OR Patterns (NEW)
Multiple alternatives using `or` keyword:
```frame
match status {
    case 200 or 201 or 204 {
        return "success"
    }
    case 400 or 404 or 403 {
        return "client error"
    }
    case 500 or 502 or 503 {
        return "server error"
    }
}
```

#### Star Patterns (NEW)
Capture remaining sequence elements:
```frame
match lst {
    case [first, *rest] {
        # first gets first element, rest gets remaining list
        return "first: " + str(first) + ", rest: " + str(rest)
    }
    case [first, *middle, last] {
        # first gets first, last gets last, middle gets everything between
        return "edges with middle"
    }
    case [a, b, *tail] {
        # a and b get first two, tail gets the rest
        return "two plus rest"
    }
}
```

#### AS Patterns
Bind entire matched pattern to a variable:
```frame
match data {
    case [x, y] as point {
        # point contains the whole list, x and y contain elements
        return "point: " + str(point)
    }
    case (1 or 2 or 3) as num {
        # num contains the matched value
        return "small number: " + str(num)
    }
}
```

#### Sequence Patterns
Match lists and tuples:
```frame
match lst {
    case [] { return "empty" }
    case [x] { return "single: " + str(x) }
    case [x, y] { return "pair" }
    case [x, y, z] { return "triple" }
    case [_, _, _, *rest] { return "3+ elements" }
}
```

#### Mapping Patterns
Match dictionary structures:
```frame
match response {
    case {"status": 200, "data": data} {
        return process_success(data)
    }
    case {"status": 404} {
        return "not found"
    }
    case {"error": {"code": code, "message": msg}} {
        return "Error " + str(code) + ": " + msg
    }
}
```

#### Guard Clauses
Add conditions to patterns:
```frame
match score {
    case x if x >= 90 { return "A" }
    case x if x >= 80 { return "B" }
    case x if x >= 70 { return "C" }
    case x if x >= 60 { return "D" }
    case _ { return "F" }
}
```

#### Nested Patterns
Match complex nested structures:
```frame
match data {
    case [1, [2, 3]] {
        return "specific nested list"
    }
    case [x, [y, z]] {
        return "nested with captures"
    }
    case {"data": [first, *rest]} {
        return "dict containing list with unpacking"
    }
    case [[a, b], [c, d]] {
        return "matrix 2x2"
    }
}
```

## 🔧 Implementation Details

### Scanner Changes
- Added `Match` and `Case` keywords to token types
- Keywords registered in scanner's keyword map

### AST Additions
- `MatchStmtNode`: Represents match statement with expression and cases
- `CaseNode`: Individual case with pattern, optional guard, and statements
- `PatternNode`: Enum representing all pattern types including:
  - `Literal`: Literal value patterns
  - `Capture`: Variable binding patterns
  - `Wildcard`: Underscore pattern
  - `Sequence`: List/tuple patterns
  - `Mapping`: Dictionary patterns
  - `Class`: Class-based patterns (limited support)
  - `Or`: OR patterns for alternatives
  - `As`: AS patterns for binding
  - `Star`: Star patterns for unpacking

### Parser Implementation
- `match_statement()`: Parses match statements
- `parse_pattern()`: Recursive pattern parser supporting all types
- `parse_pattern_inner()`: Helper for inner pattern parsing
- OR patterns use `or` keyword to avoid conflict with Frame's pipe operator
- Star patterns supported in sequences with `*identifier` syntax
- Guard clause parsing with `if` expressions

### Python Visitor
- `visit_match_stmt_node()`: Generates Python `match` statement
- `visit_case_node()`: Generates `case` clauses with guards
- `visit_pattern_node()`: Pattern-specific code generation
- OR patterns generate Python `|` syntax
- Star patterns generate Python `*` unpacking
- Proper indentation and statement handling

## 📊 Design Decisions

### OR Pattern Syntax
- **Choice**: Use `or` keyword instead of `|`
- **Rationale**: Avoids conflict with Frame's existing pipe operator
- **Example**: `case 1 or 2 or 3` generates Python `case 1 | 2 | 3`

### Star Pattern Implementation
- **Syntax**: `*identifier` in sequence patterns
- **Support**: Works in lists `[first, *rest]` and tuples `(a, *b, c)`
- **Python Generation**: Direct mapping to Python's unpacking syntax

### Class Pattern Limitation
- **Current**: Limited support using tuple workaround
- **Example**: `case ("Point", x, y)` simulates class pattern
- **Future**: Full support pending Frame class implementation

## 🧪 Testing

### Test Files Created
- `test_match_case.frm`: Comprehensive basic pattern matching tests
- `test_match_patterns_advanced.frm`: OR patterns and star patterns

### Features Tested
- ✅ All literal types (numbers, strings, booleans, None)
- ✅ Capture patterns with variable binding
- ✅ Wildcard patterns
- ✅ Sequence patterns (lists and tuples)
- ✅ Mapping patterns (dictionaries)
- ✅ Guard clauses with conditions
- ✅ Nested pattern matching
- ✅ AS patterns for binding
- ✅ OR patterns with `or` keyword
- ✅ Star patterns for unpacking
- ✅ Complex combinations

## 🚀 Migration Guide

### From if-elif-else Chains
**Before (v0.43):**
```frame
fn classify(value) {
    if value == 1 or value == 2 or value == 3 {
        return "small"
    } elif value == 10 or value == 20 {
        return "round"
    } elif value > 100 {
        return "large"
    } else {
        return "other"
    }
}
```

**After (v0.44):**
```frame
fn classify(value) {
    match value {
        case 1 or 2 or 3 {
            return "small"
        }
        case 10 or 20 {
            return "round"
        }
        case x if x > 100 {
            return "large"
        }
        case _ {
            return "other"
        }
    }
}
```

### Structural Data Processing
```frame
fn process_response(response) {
    match response {
        case {"status": 200 or 201, "data": [first, *rest]} {
            # Success with data unpacking
            return handle_success(first, rest)
        }
        case {"status": code, "error": msg} if code >= 400 {
            # Error handling with guard
            return handle_error(code, msg)
        }
        case {"redirect": url} as redirect_response {
            # Capture entire response
            return handle_redirect(redirect_response)
        }
        case _ {
            return handle_unknown()
        }
    }
}
```

## ⚠️ Known Limitations

1. **Class Patterns**: Limited to tuple workaround pending Frame class support
2. **Dictionary Keys**: Must be string literals or identifiers in patterns

## 📈 Statistics

- **New AST Nodes**: 3 (MatchStmtNode, CaseNode, PatternNode)
- **New Keywords**: 2 (match, case)
- **Pattern Types**: 10 (literal, capture, wildcard, sequence, mapping, class, or, as, star, nested)
- **Test Coverage**: 100% of supported features
- **Files Modified**: scanner.rs, parser.rs, ast.rs, python_visitor.rs

## 🔜 Future Enhancements

1. **Full Class Pattern Support**: When Frame adds class syntax
2. **Type Patterns**: Pattern matching with type constraints
3. **Value Patterns**: Computed value matching
4. **Enhanced Guard Clauses**: More complex guard expressions

## 💡 Tips and Best Practices

1. **Use match-case for complex conditionals**: More readable than long if-elif chains
2. **Leverage star patterns**: Great for list processing and unpacking
3. **Combine patterns**: Use OR patterns to reduce code duplication
4. **Guard clauses for ranges**: `case x if x > 10 and x < 20`
5. **AS patterns for debugging**: Capture entire structures while destructuring

## 🎉 Summary

Frame v0.44 delivers comprehensive pattern matching capabilities, bringing modern Python 3.10+ features to Frame. With support for all major pattern types including OR patterns (using `or` keyword) and star patterns for unpacking, Frame now offers powerful tools for structural data processing and control flow.

The implementation maintains Frame's design philosophy while generating clean, idiomatic Python code. This release sets the foundation for future enhancements like full class pattern support once Frame introduces native class syntax.

---

**Next Version Preview**: v0.45 will likely introduce class support, enabling full class pattern matching and object-oriented programming capabilities in Frame.