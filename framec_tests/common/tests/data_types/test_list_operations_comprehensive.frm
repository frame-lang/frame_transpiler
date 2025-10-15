# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Comprehensive test for all list operations in Frame
# This test demonstrates that Frame supports all Python list methods
# through natural pass-through to the target language

fn test_list_creation() {
    print("\n=== List Creation ===")
    
    # Empty list
    var empty = []
    print("Empty list: " + str(empty))
    
    # List literal
    var numbers = [1, 2, 3, 4, 5]
    print("Number list: " + str(numbers))
    
    # Mixed types
    var mixed = [1, "hello", 3.14, true, None]
    print("Mixed list: " + str(mixed))
    
    # Nested lists
    var nested = [[1, 2], [3, 4], [5, 6]]
    print("Nested list: " + str(nested))
    
    # List from range
    var from_range = list(range(5))
    print("From range: " + str(from_range))
}

fn test_list_access_modification() {
    print("\n=== List Access and Modification ===")
    var list = [10, 20, 30, 40, 50]
    
    # Indexing
    print("list[0]: " + str(list[0]))  # 10
    print("list[-1]: " + str(list[-1]))  # 50
    
    # Assignment
    list[1] = 25
    print("After list[1] = 25: " + str(list))
    
    # Length
    print("len(list): " + str(len(list)))  # 5
}

fn test_list_methods_adding() {
    print("\n=== List Methods - Adding Elements ===")
    var list = [1, 2, 3]
    
    # append() - add single element to end
    list.append(4)
    print("After append(4): " + str(list))  # [1, 2, 3, 4]
    
    # insert() - add element at specific position
    list.insert(1, 99)
    print("After insert(1, 99): " + str(list))  # [1, 99, 2, 3, 4]
    
    # extend() - add multiple elements
    list.extend([5, 6, 7])
    print("After extend([5, 6, 7]): " + str(list))  # [1, 99, 2, 3, 4, 5, 6, 7]
    
    # += operator (same as extend) - Frame doesn't support += so use extend
    list.extend([8, 9])
    print("After extend([8, 9]) (simulating +=): " + str(list))
}

fn test_list_methods_removing() {
    print("\n=== List Methods - Removing Elements ===")
    var list = [1, 2, 3, 4, 5, 3, 6]
    
    # remove() - remove first occurrence of value
    list.remove(3)
    print("After remove(3): " + str(list))  # [1, 2, 4, 5, 3, 6]
    
    # pop() - remove and return element at index (default: last)
    var popped = list.pop()
    print("pop() returned: " + str(popped) + ", list: " + str(list))
    
    var popped_idx = list.pop(1)
    print("pop(1) returned: " + str(popped_idx) + ", list: " + str(list))
    
    # clear() - remove all elements
    var temp = [1, 2, 3]
    temp.clear()
    print("After clear(): " + str(temp))  # []
}

fn test_list_methods_searching() {
    print("\n=== List Methods - Searching ===")
    var list = [10, 20, 30, 20, 40]
    
    # index() - find index of first occurrence
    var idx = list.index(20)
    print("index(20): " + str(idx))  # 1
    
    # index() with start/end parameters
    var idx2 = list.index(20, 2)
    print("index(20, 2): " + str(idx2))  # 3
    
    # count() - count occurrences
    var count = list.count(20)
    print("count(20): " + str(count))  # 2
    
    # 'in' operator
    print("30 in list: " + str(30 in list))  # True
    print("99 in list: " + str(99 in list))  # False
    
    # 'not in' operator
    print("99 not in list: " + str(99 not in list))  # True
}

fn test_list_methods_ordering() {
    print("\n=== List Methods - Ordering ===")
    var list = [3, 1, 4, 1, 5, 9, 2, 6]
    
    # sort() - sort in place
    list.sort()
    print("After sort(): " + str(list))  # [1, 1, 2, 3, 4, 5, 6, 9]
    
    # sort() in reverse - Frame doesn't support keyword arguments yet
    # We'll reverse after sorting as a workaround
    list.reverse()
    print("After sort then reverse: " + str(list))  # [9, 6, 5, 4, 3, 2, 1, 1]
    
    # reverse() - reverse in place
    var list2 = [1, 2, 3, 4, 5]
    list2.reverse()
    print("After reverse(): " + str(list2))  # [5, 4, 3, 2, 1]
}

fn test_list_methods_copying() {
    print("\n=== List Methods - Copying ===")
    var original = [1, 2, [3, 4]]
    
    # copy() - shallow copy
    var copied = original.copy()
    copied[0] = 99
    print("Original after modifying copy: " + str(original))  # [1, 2, [3, 4]]
    print("Copied: " + str(copied))  # [99, 2, [3, 4]]
    
    # Note: nested list is still shared (shallow copy)
    # Frame doesn't support chained indexing yet, so we use a workaround
    var nested_list = copied[2]
    nested_list[0] = 88
    print("Original after modifying nested: " + str(original))  # [1, 2, [88, 4]]
}

fn test_list_slicing() {
    print("\n=== List Slicing Operations ===")
    var list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    # Basic slicing
    print("list[2:5]: " + str(list[2:5]))  # [2, 3, 4]
    print("list[:3]: " + str(list[:3]))  # [0, 1, 2]
    print("list[7:]: " + str(list[7:]))  # [7, 8, 9]
    print("list[:]: " + str(list[:]))  # [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    # Negative indexing
    print("list[-3:]: " + str(list[-3:]))  # [7, 8, 9]
    print("list[:-3]: " + str(list[:-3]))  # [0, 1, 2, 3, 4, 5, 6]
    
    # Step parameter
    print("list[::2]: " + str(list[::2]))  # [0, 2, 4, 6, 8]
    print("list[1::2]: " + str(list[1::2]))  # [1, 3, 5, 7, 9]
    print("list[::-1]: " + str(list[::-1]))  # [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    
    # Complex expressions in slices
    var start = 2
    var end = 8
    var step = 2
    print("list[start:end:step]: " + str(list[start:end:step]))  # [2, 4, 6]
    print("list[start+1:end-1]: " + str(list[start+1:end-1]))  # [3, 4, 5, 6]
}

fn test_list_comprehensions() {
    print("\n=== List Comprehensions ===")
    
    # Basic comprehension
    var squares = [x * x for x in range(5)]
    print("Squares: " + str(squares))  # [0, 1, 4, 9, 16]
    
    # With condition
    var evens = [x for x in range(10) if x % 2 == 0]
    print("Evens: " + str(evens))  # [0, 2, 4, 6, 8]
    
    # String operations in comprehension
    var words = ["hello", "world", "frame"]
    var upper_words = [w.upper() for w in words]
    print("Upper words: " + str(upper_words))  # ['HELLO', 'WORLD', 'FRAME']
    
    # Nested comprehension
    var matrix = [[i + j for j in range(3)] for i in range(3)]
    print("Matrix: " + str(matrix))  # [[0, 1, 2], [1, 2, 3], [2, 3, 4]]
}

fn test_list_unpacking() {
    print("\n=== List Unpacking ===")
    
    var list1 = [1, 2, 3]
    var list2 = [4, 5, 6]
    
    # Unpacking in list literal
    var combined = [*list1, *list2, 7, 8]
    print("Combined with unpacking: " + str(combined))  # [1, 2, 3, 4, 5, 6, 7, 8]
    
    # Multiple unpacking
    var all_together = [0, *list1, 99, *list2, 100]
    print("Multiple unpacking: " + str(all_together))
}

fn test_list_operations() {
    print("\n=== List Operations ===")
    
    var list1 = [1, 2, 3]
    var list2 = [4, 5, 6]
    
    # Concatenation
    var concat = list1 + list2
    print("list1 + list2: " + str(concat))  # [1, 2, 3, 4, 5, 6]
    
    # Repetition
    var repeated = list1 * 3
    print("list1 * 3: " + str(repeated))  # [1, 2, 3, 1, 2, 3, 1, 2, 3]
    
    # Min/Max
    var numbers = [5, 2, 8, 1, 9]
    print("min(numbers): " + str(min(numbers)))  # 1
    print("max(numbers): " + str(max(numbers)))  # 9
    
    # Sum
    print("sum(numbers): " + str(sum(numbers)))  # 25
    
    # All/Any
    var bools = [true, true, true]
    print("all(bools): " + str(all(bools)))  # True
    var mixed_bools = [true, false, true]
    print("any(mixed_bools): " + str(any(mixed_bools)))  # True
}

fn main() {
    print("=== COMPREHENSIVE LIST OPERATIONS TEST ===")
    
    test_list_creation()
    test_list_access_modification()
    test_list_methods_adding()
    test_list_methods_removing()
    test_list_methods_searching()
    test_list_methods_ordering()
    test_list_methods_copying()
    test_list_slicing()
    test_list_comprehensions()
    test_list_unpacking()
    test_list_operations()
    
    print("\n=== ALL LIST OPERATION TESTS COMPLETED ===")
}