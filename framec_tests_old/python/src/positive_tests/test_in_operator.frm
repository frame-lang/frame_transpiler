# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test the 'in' operator for membership testing

fn test_in_with_lists() {
    print("=== Testing 'in' with lists ===")
    var numbers = [1, 2, 3, 4, 5]
    
    # Test positive cases
    if 3 in numbers {
        print("✓ 3 is in the list")
    }
    
    if 1 in numbers {
        print("✓ 1 is in the list")
    }
    
    # Test negative cases
    if 10 in numbers {
        print("✗ 10 should not be in the list")
    } else {
        print("✓ 10 is not in the list")
    }
    
    # With variables
    var target = 4
    if target in numbers {
        print("✓ " + str(target) + " is in the list")
    }
}

fn test_in_with_strings() {
    print("\n=== Testing 'in' with strings ===")
    var text = "Hello, World!"
    
    if "World" in text {
        print("✓ 'World' is in the string")
    }
    
    if "o" in text {
        print("✓ 'o' is in the string")
    }
    
    if "xyz" in text {
        print("✗ 'xyz' should not be in the string")
    } else {
        print("✓ 'xyz' is not in the string")
    }
}

fn test_in_with_dicts() {
    print("\n=== Testing 'in' with dictionaries ===")
    var data = {"name": "Alice", "age": 30, "city": "Paris"}
    
    # Test for keys
    if "name" in data {
        print("✓ 'name' key is in the dictionary")
    }
    
    if "age" in data {
        print("✓ 'age' key is in the dictionary")
    }
    
    if "country" in data {
        print("✗ 'country' key should not be in the dictionary")
    } else {
        print("✓ 'country' key is not in the dictionary")
    }
    
    # With variable
    var key = "city"
    if key in data {
        print("✓ '" + key + "' key is in the dictionary")
    }
}

fn test_in_with_sets() {
    print("\n=== Testing 'in' with sets ===")
    var fruits = {"apple", "banana", "orange"}
    
    if "apple" in fruits {
        print("✓ 'apple' is in the set")
    }
    
    if "grape" in fruits {
        print("✗ 'grape' should not be in the set")
    } else {
        print("✓ 'grape' is not in the set")
    }
}

fn test_not_in() {
    print("\n=== Testing 'not in' ===")
    var colors = ["red", "green", "blue"]
    
    if not "yellow" in colors {
        print("✓ 'yellow' is not in the list")
    }
    
    if not "red" in colors {
        print("✗ 'red' should be in the list")
    } else {
        print("✓ 'red' is in the list")
    }
}

fn test_in_expressions() {
    print("\n=== Testing 'in' in complex expressions ===")
    var nums = [1, 2, 3, 4, 5]
    var chars = "abcde"
    
    # Combining with and/or
    if 2 in nums and "b" in chars {
        print("✓ Both conditions are true")
    }
    
    if 10 in nums or "c" in chars {
        print("✓ At least one condition is true")
    }
    
    # In boolean variable
    var has_three = 3 in nums
    if has_three {
        print("✓ has_three is True")
    }
}

fn main() {
    print("Frame v0.38 - 'in' Operator Test")
    print("=" * 50)
    
    test_in_with_lists()
    test_in_with_strings()
    test_in_with_dicts()
    test_in_with_sets()
    test_not_in()
    test_in_expressions()
    
    print("\n" + "=" * 50)
    print("All 'in' operator tests completed!")
}