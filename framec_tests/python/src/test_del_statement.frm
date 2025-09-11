# Test del statement support - v0.50
# Expected: Various del operations work correctly

fn test_del_list_element() {
    var mylist = [1, 2, 3, 4, 5]
    print("Original list: " + str(mylist))
    
    # Delete element at index 2
    del mylist[2]
    print("After del mylist[2]: " + str(mylist))
    
    # Delete last element
    del mylist[-1]
    print("After del mylist[-1]: " + str(mylist))
    
    return mylist
}

fn test_del_dict_entry() {
    var mydict = {"a": 1, "b": 2, "c": 3}
    print("Original dict: " + str(mydict))
    
    # Delete a key
    del mydict["b"]
    print("After del mydict['b']: " + str(mydict))
    
    # Delete another key with variable
    var key = "a"
    del mydict[key]
    print("After del mydict[key]: " + str(mydict))
    
    return mydict
}

fn test_del_variable() {
    var x = 42
    var y = "hello"
    print("x = " + str(x))
    print("y = " + y)
    
    # Delete a variable
    del x
    # After del, x should not be accessible
    # print(x) would cause NameError
    
    # Delete another variable
    del y
    
    # Create new variables with different names
    var new_x = 100
    var new_y = "world"
    print("new_x = " + str(new_x))
    print("new_y = " + new_y)
    
    return new_x
}

fn test_del_slice() {
    var nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    print("Original: " + str(nums))
    
    # Delete a slice
    del nums[2:5]
    print("After del nums[2:5]: " + str(nums))
    
    # Delete every other element
    del nums[::2]
    print("After del nums[::2]: " + str(nums))
    
    return nums
}

fn test_del_nested_structures() {
    var data = {
        "users": [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25},
            {"name": "Charlie", "age": 35}
        ],
        "count": 3
    }
    
    print("Original data: " + str(data))
    
    # Delete nested list element
    del data["users"][1]
    print("After deleting Bob: " + str(data))
    
    # Delete nested dict field
    del data["users"][0]["age"]
    print("After deleting Alice's age: " + str(data))
    
    # Delete entire key
    del data["count"]
    print("After deleting count: " + str(data))
    
    return data
}

fn main() {
    print("=== Testing Del Statement ===")
    print("")
    
    print("--- Test 1: Delete list elements ---")
    var result1 = test_del_list_element()
    print("Final result: " + str(result1))
    print("")
    
    print("--- Test 2: Delete dict entries ---")
    var result2 = test_del_dict_entry()
    print("Final result: " + str(result2))
    print("")
    
    print("--- Test 3: Delete variables ---")
    var result3 = test_del_variable()
    print("Final result: " + str(result3))
    print("")
    
    print("--- Test 4: Delete slices ---")
    var result4 = test_del_slice()
    print("Final result: " + str(result4))
    print("")
    
    print("--- Test 5: Delete in nested structures ---")
    var result5 = test_del_nested_structures()
    print("Final result: " + str(result5))
    print("")
    
    print("=== All Del Tests Complete ===")
}