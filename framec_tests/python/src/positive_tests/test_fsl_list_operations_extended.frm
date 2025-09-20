# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test FSL extended list operations
# These should work WITHOUT backticks

fn main() {
    # Test list.insert(index, item)
    var numbers = [1, 2, 4, 5]
    numbers.insert(2, 3)  # Insert 3 at index 2
    print("After insert:", numbers)  # [1, 2, 3, 4, 5]
    
    # Test list.remove(item)
    var items = [10, 20, 30, 20, 40]
    items.remove(20)  # Remove first occurrence of 20
    print("After remove:", items)  # [10, 30, 20, 40]
    
    # Test list.index(item) - find position
    var fruits = ["apple", "banana", "orange"]
    var pos = fruits.index("banana")
    print("Index of banana:", pos)  # 1
    
    # Test list.count(item)
    var nums = [1, 2, 3, 2, 4, 2]
    var count = nums.count(2)
    print("Count of 2:", count)  # 3
    
    # Test list.extend(other)
    var list1 = [1, 2, 3]
    var list2 = [4, 5, 6]
    list1.extend(list2)
    print("After extend:", list1)  # [1, 2, 3, 4, 5, 6]
    
    # Test list.reverse()
    var rev_list = [1, 2, 3, 4, 5]
    rev_list.reverse()
    print("After reverse:", rev_list)  # [5, 4, 3, 2, 1]
    
    # Test list.sort()
    var unsorted = [3, 1, 4, 1, 5, 9, 2, 6]
    unsorted.sort()
    print("After sort:", unsorted)  # [1, 1, 2, 3, 4, 5, 6, 9]
    
    # Test list.copy()
    var original = [1, 2, 3]
    var copied = original.copy()
    copied.append(4)
    print("Original:", original)  # [1, 2, 3]
    print("Copied:", copied)  # [1, 2, 3, 4]
    
    # Test list.is_empty property
    var empty_list = []
    var non_empty = [1]
    var is_empty1 = empty_list.is_empty
    var is_empty2 = non_empty.is_empty
    print("Empty list is_empty:", is_empty1)  # true
    print("Non-empty list is_empty:", is_empty2)  # false
    
    return
}