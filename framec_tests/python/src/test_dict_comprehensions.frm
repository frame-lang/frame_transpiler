// Test dictionary comprehensions from the user's images

fn test_dict_comprehensions() {
    // Basic dictionary comprehension - square numbers as values
    var squares = {x: x * x for x in range(5)}
    print(squares)  // {0: 0, 1: 1, 2: 4, 3: 9, 4: 16}
    
    // Dictionary from two lists using manual iteration
    var keys = ["a", "b", "c"]  
    var values = [1, 2, 3]
    var dict_from_lists = {keys[i]: values[i] for i in range(len(keys))}
    print(dict_from_lists)  // {"a": 1, "b": 2, "c": 3}
    
    // With conditional - only even numbers
    var even_squares = {x: x * x for x in range(10) if x % 2 == 0}
    print(even_squares)  // {0: 0, 2: 4, 4: 16, 6: 36, 8: 64}
    
    // Skip ternary operator test as it's not supported in Frame yet
    // This would be: var even_odd = {x: ("even" if x % 2 == 0 else "odd") for x in range(5)}
    
    // Complex filtering - words longer than 4 characters
    var words = ["hello", "world", "hi", "test", "python", "frame"]
    var word_lengths = {word: len(word) for word in words if len(word) > 4}
    print(word_lengths)  // {"hello": 5, "world": 5, "python": 6, "frame": 5}
    
    // Nested comprehensions - multiplication table
    var multiplication_table = {i: {j: i*j for j in range(3)} for i in range(3)}
    print(multiplication_table)  // {0: {0: 0, 1: 0, 2: 0}, 1: {0: 0, 1: 1, 2: 2}, 2: {0: 0, 1: 2, 2: 4}}
    
    // String manipulation - simple character filtering
    var text = "hello world"
    var filtered_chars = {char: len(char) for char in text if char != " "}
    print(filtered_chars)  // Simple char filtering
    
    // Skip enumerate and items() - tuple unpacking not supported in comprehensions
    // This would be: var indexed_names = {idx: name for idx, name in enumerate(names)}
    // This would be: var inverted = {v: k for k, v in original.items()}
    
    // Complex expression in key and value
    var nums = [1, 2, 3, 4, 5]
    var complex_dict = {str(x): x * 2 + 1 for x in nums if x > 2}
    print(complex_dict)  // {"3": 7, "4": 9, "5": 11}
}

fn main() {
    test_dict_comprehensions()
}