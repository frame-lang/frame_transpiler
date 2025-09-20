# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test advanced match-case patterns in Frame v0.44
# Tests OR patterns (using 'or' keyword) and star patterns

# Test OR patterns using 'or' keyword
fn test_or_patterns(value) {
    match value {
        case 1 or 2 or 3 {
            return "small number"
        }
        case "yes" or "y" or "true" {
            return "affirmative"
        }
        case "no" or "n" or "false" {
            return "negative"
        }
        case 10 or 20 or 30 {
            return "round number"
        }
        case _ {
            return "other"
        }
    }
}

# Test star patterns in sequences
fn test_star_patterns(lst) {
    match lst {
        case [] {
            return "empty"
        }
        case [x] {
            return "single: " + str(x)
        }
        case [first, *rest] {
            return "first: " + str(first) + ", rest: " + str(rest)
        }
        case [first, *middle, last] {
            return "first: " + str(first) + ", middle: " + str(middle) + ", last: " + str(last)
        }
        case [a, b, *rest] {
            return "two elements then rest: " + str(a) + ", " + str(b) + ", " + str(rest)
        }
        case _ {
            return "no match"
        }
    }
}

# Test combining OR patterns with guard clauses
fn test_or_with_guards(value) {
    match value {
        case x if x == 1 or x == 2 {
            return "one or two"
        }
        case x if x > 10 and x < 20 {
            return "between 10 and 20"
        }
        case 100 or 200 or 300 {
            return "hundred multiple"
        }
        case _ {
            return "other"
        }
    }
}

# Test nested OR patterns
fn test_nested_or_patterns(data) {
    match data {
        case [1 or 2, "a" or "b"] {
            return "matched nested or"
        }
        case (_, 100) {
            return "tuple with 100" 
        }
        case {"type": "error" or "warning"} {
            return "error or warning type"
        }
        case _ {
            return "no match"
        }
    }
}

# Test AS patterns with OR
fn test_as_with_or(value) {
    match value {
        case (1 or 2 or 3) as num {
            return "small number: " + str(num)
        }
        case [_, _] as lst {
            return "two element list: " + str(lst)
        }
        case [_, _, _] as lst3 {
            return "three element list: " + str(lst3)
        }
        case _ {
            return "no match"
        }
    }
}

# Main test function
fn main() {
    # Test OR patterns
    print("=== OR Patterns ===")
    print(test_or_patterns(1))
    print(test_or_patterns(2))
    print(test_or_patterns(3))
    print(test_or_patterns("yes"))
    print(test_or_patterns("y"))
    print(test_or_patterns("no"))
    print(test_or_patterns(20))
    print(test_or_patterns(42))
    
    # Test star patterns
    print("\n=== Star Patterns ===")
    print(test_star_patterns([]))
    print(test_star_patterns([1]))
    print(test_star_patterns([1, 2]))
    print(test_star_patterns([1, 2, 3]))
    print(test_star_patterns([1, 2, 3, 4, 5]))
    
    # Test OR with guards
    print("\n=== OR with Guards ===")
    print(test_or_with_guards(1))
    print(test_or_with_guards(2))
    print(test_or_with_guards(15))
    print(test_or_with_guards(100))
    print(test_or_with_guards(50))
    
    # Test nested OR patterns
    print("\n=== Nested OR Patterns ===")
    print(test_nested_or_patterns([1, "a"]))
    print(test_nested_or_patterns([2, "b"]))
    print(test_nested_or_patterns((42, 100)))
    print(test_nested_or_patterns({"type": "error"}))
    print(test_nested_or_patterns({"type": "info"}))
    
    # Test AS with OR
    print("\n=== AS with OR ===")
    print(test_as_with_or(2))
    print(test_as_with_or([10, 20]))
    print(test_as_with_or([10, 20, 30]))
    print(test_as_with_or("string"))
}

# Run the tests
main()