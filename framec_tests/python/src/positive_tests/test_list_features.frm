# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test comprehensive list features in Frame
fn main() {
    # Test list literals
    var numbers = [1, 2, 3, 4, 5]
    var strings = ["hello", "world", "frame"]
    var mixed = [1, "two", 3, "four"]
    var empty = []
    
    # Test list indexing
    print("First number:", numbers[0])
    print("Last string:", strings[2])
    
    # Test list iteration
    print("\n=== Iterating numbers ===")
    for num in numbers {
        print("Number:", num)
    }
    
    print("\n=== Iterating strings ===")
    for str in strings {
        print("String:", str)
    }
    
    # Test nested lists
    var matrix = [[1, 2], [3, 4], [5, 6]]
    print("\n=== Matrix ===")
    for row in matrix {
        print("Row:", row)
    }
    
    # Test list operations
    numbers.append(6)
    
    print("\n=== After append ===")
    print("Numbers:", numbers)
    
    var length = len(numbers)
    print("Length:", length)
    
    # Test list as function parameter
    processList(numbers)
    
    # Test list as return value
    var newList = createList()
    print("\n=== Created list ===")
    print("New list:", newList)
}

fn processList(items) {
    print("\n=== Processing list ===")
    for item in items {
        print("Processing:", item)
    }
}

fn createList() {
    var result = [10, 20, 30]
    return result
}

# Call main to run the tests