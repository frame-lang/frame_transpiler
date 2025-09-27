# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test native list methods - currently using backticks (v0.33 WIP)
fn main() {
    # Test list.append()
    var numbers = [1, 2, 3]
    numbers.append(4)
    numbers.append(5)
    print("After append:", numbers)  # [1, 2, 3, 4, 5]
    
    # Test list length using len()
    var length = len(numbers)
    print("Length:", length)  # 5
    
    # Test list emptiness check
    var empty_list = []
    var is_empty_val = len(empty_list) == 0
    print("Empty list is empty:", is_empty_val)  # true
    var nums_empty = len(numbers) == 0
    print("Numbers is empty:", nums_empty)  # false
    
    # Test list.clear()
    var temp = [1, 2, 3]
    temp.clear()
    print("After clear:", temp)  # []
    
    # Test list.pop()
    var items = [10, 20, 30, 40]
    var last = items.pop()
    print("Popped:", last)  # 40
    print("After pop:", items)  # [10, 20, 30]
    
    # Test list.pop(index)
    var middle = items.pop(1)
    print("Popped at 1:", middle)  # 20
    print("After pop(1):", items)  # [10, 30]
    
    # Test chaining
    var chain_test = []
    chain_test.append(1)
    chain_test.append(2)
    chain_test.append(3)
    print("Chain test:", chain_test)  # [1, 2, 3]
    
    # Test with different types
    var strings = []
    strings.append("hello")
    strings.append("world")
    print("String list:", strings)  # ["hello", "world"]
    
    # Test in expressions
    var data = [5, 10, 15]
    if len(data) > 2 {
        print("List has more than 2 items")
    }
    
    if len(data) > 0 {
        print("List is not empty")
    }
    
    # Test with function returns
    var result = create_list()
    result.append(100)
    print("Function result:", result)  # [1, 2, 3, 100]
}

fn create_list() {
    var list = [1, 2, 3]
    return list
}

# Call main to run the tests