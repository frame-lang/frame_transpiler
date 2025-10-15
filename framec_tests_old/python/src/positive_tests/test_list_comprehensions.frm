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

fn test_comprehension_with_expressions() {
    var nums = [1, 2, 3, 4, 5]
    
    # Comprehension with expressions
    var doubled = [x * 2 for x in nums]
    print("Doubled: " + str(doubled))
    
    # Should be [2, 4, 6, 8, 10]
    return doubled
}

fn test_comprehension_with_complex_expression() {
    # Using lists instead of dictionaries since Frame doesn't support dict syntax
    var numbers = [30, 25, 35, 28, 40, 22]
    
    # Filter adults (age >= 30)
    var adults = [age for age in numbers if age >= 30]
    print("Adult ages: " + str(adults))
    
    # Should be [30, 35, 40]
    return adults
}

fn test_comprehension_with_range() {
    var words = ["hello", "world", "from", "Frame"]
    
    # Create indices using range
    var indices = [i for i in range(4)]
    print("Indices: " + str(indices))
    
    # Should be [0, 1, 2, 3]
    return indices
}

fn test_comprehension_string_concatenation() {
    var names = ["alice", "bob", "charlie"]
    
    # String concatenation in comprehension
    var greetings = ["Hello " + name for name in names]
    print("Greetings: " + str(greetings))
    
    # Filter and concatenate (simplified without len)
    var long_names = [name + "!" for name in names]
    print("Long names: " + str(long_names))
    
    return long_names
}

fn main() {
    print("=== Testing List Comprehensions ===")
    
    var squares = test_basic_comprehension()
    print("Basic comprehension result: " + str(squares))
    
    var evens = test_comprehension_with_condition() 
    print("Conditional comprehension: " + str(evens))
    
    var matrix = test_nested_comprehension()
    print("Nested comprehension result: " + str(matrix))
    
    var doubled = test_comprehension_with_expressions()
    print("Expression comprehension: " + str(doubled))
    
    var adults = test_comprehension_with_complex_expression()
    print("Complex expression result: " + str(adults))
    
    var indices = test_comprehension_with_range()
    print("Range comprehension result: " + str(indices))
    
    var processed = test_comprehension_string_concatenation()
    print("String concatenation result: " + str(processed))
    
    print("=== All Comprehension Tests Complete ===")
}