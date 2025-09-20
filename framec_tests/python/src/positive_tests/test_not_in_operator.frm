# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test the 'not in' operator - now works directly!

fn test_not_in_lists() {
    print("=== Testing 'not in' with lists ===")
    var numbers = [1, 2, 3, 4, 5]
    
    # Direct 'not in' syntax now works!
    if 10 not in numbers {
        print("✓ 10 is not in the list")
    }
    
    if 3 not in numbers {
        print("✗ 3 should be in the list")
    } else {
        print("✓ 3 is in the list")
    }
    
    var absent = [6, 7, 8]
    var i = 0
    while i < len(absent) {
        if absent[i] not in numbers {
            print("✓ " + str(absent[i]) + " is not in numbers")
        }
        i = i + 1
    }
}

fn test_not_in_strings() {
    print("\n=== Testing 'not in' with strings ===")
    var text = "Hello, World!"
    
    if "xyz" not in text {
        print("✓ 'xyz' is not in the string")
    }
    
    if "Hello" not in text {
        print("✗ 'Hello' should be in the string")
    } else {
        print("✓ 'Hello' is in the string")  
    }
}

fn test_not_in_dicts() {
    print("\n=== Testing 'not in' with dictionaries ===")
    var data = {"name": "Alice", "age": 30}
    
    if "email" not in data {
        print("✓ 'email' key is not in the dictionary")
    }
    
    if "name" not in data {
        print("✗ 'name' key should be in the dictionary")
    } else {
        print("✓ 'name' key is in the dictionary")
    }
}

fn test_not_in_complex() {
    print("\n=== Testing 'not in' in complex expressions ===")
    var nums = [1, 2, 3]
    var chars = "abc"
    
    # Combining with and/or
    if 5 not in nums and "d" not in chars {
        print("✓ Both are absent")
    }
    
    if 1 not in nums or "z" not in chars {
        print("✓ At least one is absent")
    }
    
    # In boolean variable
    var missing = 10 not in nums
    if missing {
        print("✓ missing is True")
    }
}

fn main() {
    print("Frame v0.38 - 'not in' Operator Test")
    print("=" * 50)
    print("Direct 'not in' syntax now works!")
    print("")
    
    test_not_in_lists()
    test_not_in_strings()
    test_not_in_dicts()
    test_not_in_complex()
    
    print("\n" + "=" * 50)
    print("All 'not in' tests completed successfully!")
}