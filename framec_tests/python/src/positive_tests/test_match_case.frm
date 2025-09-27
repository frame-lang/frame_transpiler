# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test match-case pattern matching in Frame v0.44
# This test demonstrates Frame's Python match-case support

# Test basic literal matching
fn test_literal_patterns(value) {
    match value {
        case 1 {
            return "one"
        }
        case 2 {
            return "two"
        }
        case "hello" {
            return "greeting"
        }
        case true {
            return "boolean true"
        }
        case None {
            return "none value"
        }
        case _ {
            return "other"
        }
    }
}

# Test capture patterns
fn test_capture_patterns(value) {
    match value {
        case 0 {
            return "zero"
        }
        case x {
            return "captured: " + str(x)
        }
    }
}

# Test sequence patterns
fn test_sequence_patterns(lst) {
    match lst {
        case [] {
            return "empty list"
        }
        case [x] {
            return "single element: " + str(x)
        }
        case [x, y] {
            return "two elements: " + str(x) + ", " + str(y)
        }
        case [x, y, z] {
            return "three elements: " + str(x) + ", " + str(y) + ", " + str(z)
        }
        case _ {
            return "many elements"
        }
    }
}

# Test mapping patterns
fn test_mapping_patterns(dct) {
    match dct {
        case {"type": "error"} {
            return "error type"
        }
        case {"type": "warning", "code": code} {
            return "warning with code: " + str(code)
        }
        case {"name": name, "age": age} {
            return "person: " + name + " aged " + str(age)
        }
        case _ {
            return "other dict"
        }
    }
}

# Test guard clauses
fn test_guard_clauses(value) {
    match value {
        case x if x < 0 {
            return "negative"
        }
        case x if x == 0 {
            return "zero"
        }
        case x if x > 0 and x < 10 {
            return "small positive"
        }
        case x if x >= 10 {
            return "large positive"
        }
        case _ {
            return "unknown"
        }
    }
}

# Test class patterns (simulated with tuple matching)
fn test_class_patterns(obj) {
    match obj {
        case ("Point", x, y) {
            return "Point at " + str(x) + ", " + str(y)
        }
        case ("Circle", x, y, r) {
            return "Circle at " + str(x) + ", " + str(y) + " with radius " + str(r)
        }
        case _ {
            return "unknown shape"
        }
    }
}

# Test OR patterns - NOT SUPPORTED YET
# OR patterns using | conflict with Frame's existing syntax
# fn test_or_patterns(value) {
#     match value {
#         case 1 | 2 | 3 {
#             return "small number"
#         }
#         case "yes" | "y" | "true" {
#             return "affirmative"
#         }
#         case "no" | "n" | "false" {
#             return "negative"
#         }
#         case _ {
#             return "other"
#         }
#     }
# }

# Test nested patterns
fn test_nested_patterns(data) {
    match data {
        case [1, [2, 3]] {
            return "specific nested list"
        }
        case [x, [y, z]] {
            return "nested with captures: " + str(x) + ", [" + str(y) + ", " + str(z) + "]"
        }
        case {"data": [x, y]} {
            return "dict with list: " + str(x) + ", " + str(y)
        }
        case _ {
            return "other nested structure"
        }
    }
}

# Test wildcard patterns
fn test_wildcard_patterns(value) {
    match value {
        case [_, _, 3] {
            return "list ending with 3"
        }
        case [1, _, _] {
            return "list starting with 1"
        }
        case {"key": _, "value": v} {
            return "dict with value: " + str(v)
        }
        case _ {
            return "complete wildcard"
        }
    }
}

# Test as patterns
fn test_as_patterns(value) {
    match value {
        case [x, y] as lst {
            return "list " + str(lst) + " with elements " + str(x) + " and " + str(y)
        }
        case x as val {
            return "value captured as: " + str(val)
        }
    }
}

# Main test function
fn main() {
    # Test literal patterns
    print("=== Literal Patterns ===")
    print(test_literal_patterns(1))
    print(test_literal_patterns(2))
    print(test_literal_patterns("hello"))
    print(test_literal_patterns(true))
    print(test_literal_patterns(None))
    print(test_literal_patterns(99))
    
    # Test capture patterns
    print("\n=== Capture Patterns ===")
    print(test_capture_patterns(0))
    print(test_capture_patterns(42))
    
    # Test sequence patterns
    print("\n=== Sequence Patterns ===")
    print(test_sequence_patterns([]))
    print(test_sequence_patterns([1]))
    print(test_sequence_patterns([1, 2]))
    print(test_sequence_patterns([1, 2, 3]))
    print(test_sequence_patterns([1, 2, 3, 4, 5]))
    
    # Test mapping patterns
    print("\n=== Mapping Patterns ===")
    print(test_mapping_patterns({"type": "error"}))
    print(test_mapping_patterns({"type": "warning", "code": 404}))
    print(test_mapping_patterns({"name": "Alice", "age": 30}))
    print(test_mapping_patterns({"other": "data"}))
    
    # Test guard clauses
    print("\n=== Guard Clauses ===")
    print(test_guard_clauses(-5))
    print(test_guard_clauses(0))
    print(test_guard_clauses(5))
    print(test_guard_clauses(15))
    
    # Test class patterns
    print("\n=== Class Patterns ===")
    print(test_class_patterns(("Point", 10, 20)))
    print(test_class_patterns(("Circle", 5, 5, 3)))
    print(test_class_patterns(("Rectangle", 0, 0, 10, 10)))
    
    # Test OR patterns - NOT SUPPORTED YET
    # print("\n=== OR Patterns ===")
    # print(test_or_patterns(2))
    # print(test_or_patterns("yes"))
    # print(test_or_patterns("no"))
    # print(test_or_patterns("maybe"))
    
    # Test nested patterns
    print("\n=== Nested Patterns ===")
    print(test_nested_patterns([1, [2, 3]]))
    print(test_nested_patterns([4, [5, 6]]))
    print(test_nested_patterns({"data": [7, 8]}))
    print(test_nested_patterns([1, 2, 3]))
    
    # Test wildcard patterns
    print("\n=== Wildcard Patterns ===")
    print(test_wildcard_patterns([1, 2, 3]))
    print(test_wildcard_patterns([1, 4, 5]))
    print(test_wildcard_patterns({"key": "ignored", "value": 42}))
    print(test_wildcard_patterns("something else"))
    
    # Test as patterns
    print("\n=== As Patterns ===")
    print(test_as_patterns([10, 20]))
    print(test_as_patterns(999))
}

# Run the tests