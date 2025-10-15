# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test list comprehension support - v0.34
# Expected: List comprehensions work with Frame syntax


fn test_basic_comprehension() {
    # Basic list comprehension
    var squares = [x * x for x in range(10)]
    print("Squares: " + str(squares))
    
    # Should be [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
    return squares
}

fn test_comprehension_with_condition() {
    var numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    # List comprehension with if condition
    var evens = [x for x in numbers if x % 2 == 0]
    print("Even numbers: " + str(evens))
    
    # Should be [2, 4, 6, 8, 10]
    return evens
}

fn test_nested_comprehension() {
    # Nested list comprehension for matrix
    var matrix = [[i * j for j in range(3)] for i in range(3)]
    print("Matrix: " + str(matrix))
    
    # Should be [[0,0,0], [0,1,2], [0,2,4]]
    return matrix
}

fn test_comprehension_with_complex_expression() {
    # Using simple strings instead of dict objects
    var names = ["Alice", "Bob", "Charlie", "David"]
    
    # Process strings with comprehension
    var upper_names = [str(name) for name in names]
    print("Names: " + str(upper_names))
    
    return upper_names
}

fn main() {
    print("=== Testing List Comprehensions ===")
    
    var squares = test_basic_comprehension()
    print("Basic comprehension result: " + str(squares))
    
    var evens = test_comprehension_with_condition() 
    print("Conditional comprehension: " + str(evens))
    
    var matrix = test_nested_comprehension()
    print("Nested comprehension result: " + str(matrix))
    
    var names = test_comprehension_with_complex_expression()
    print("Complex expression result: " + str(names))
    
    print("=== All Comprehension Tests Complete ===")
}