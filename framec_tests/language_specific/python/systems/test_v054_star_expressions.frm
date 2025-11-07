# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Frame v0.54 - Star Expressions for Unpacking

fn test_star_basic() {
    # Basic star expression unpacking
    first, *rest = [1, 2, 3, 4, 5]
    print("first = " + str(first))
    print("rest = " + str(rest))
    
    # Star at beginning
    *beginning, last = [10, 20, 30, 40]
    print("beginning = " + str(beginning))
    print("last = " + str(last))
    
    # Star in middle
    head, *middle, tail = [100, 200, 300, 400, 500]
    print("head = " + str(head))
    print("middle = " + str(middle))
    print("tail = " + str(tail))
}

fn test_star_with_tuples() {
    # Star with tuple unpacking
    x, *rest = (1, 2, 3, 4)
    print("x = " + str(x))
    print("rest (from tuple) = " + str(rest))
    
    # Multiple elements before star
    a, b, *remainder = (10, 20, 30, 40, 50)
    print("a = " + str(a) + ", b = " + str(b))
    print("remainder = " + str(remainder))
}

fn test_star_edge_cases() {
    # Star gets empty list when no elements left
    only, *empty = [42]
    print("only = " + str(only))
    print("empty = " + str(empty))
    
    # Star gets most elements
    single, *most_items = [1, 2, 3, 4, 5]
    print("single = " + str(single))
    print("most_items = " + str(most_items))
    
    # Two regular vars with star in middle
    first, *mid, last = [1, 2]
    print("first = " + str(first) + ", last = " + str(last))
    print("mid (should be empty) = " + str(mid))
}

fn get_sequence() {
    return [10, 20, 30, 40, 50, 60]
}

fn test_star_with_functions() {
    # Unpack function return value with star
    x, y, *rest = get_sequence()
    print("x = " + str(x) + ", y = " + str(y))
    print("rest from function = " + str(rest))
}

fn test_star_practical() {
    # Practical example: processing command arguments
    args = ["script.py", "-v", "--output", "file.txt", "input1.txt", "input2.txt"]
    script, *options = args
    print("Script: " + script)
    print("Options and files: " + str(options))
    
    # Another practical example: head and tail of list
    numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    first_three_0, first_three_1, first_three_2, *remaining = numbers
    print("First three: " + str([first_three_0, first_three_1, first_three_2]))
    print("Remaining: " + str(remaining))
}

fn main() {
    print("=== Star Expression Tests ===")
    test_star_basic()
    print("\n=== Star with Tuples ===")
    test_star_with_tuples()
    print("\n=== Edge Cases ===")
    test_star_edge_cases()
    print("\n=== With Functions ===")
    test_star_with_functions()
    print("\n=== Practical Examples ===")
    test_star_practical()
    print("\n=== All tests complete ===")
}
