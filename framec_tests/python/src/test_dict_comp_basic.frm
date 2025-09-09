# Basic dictionary comprehension test without tuple unpacking

fn test_basic_dict_comprehensions() {
    print("=== Basic Dictionary Comprehensions ===")
    
    # Simple key:value mapping
    var squares = {x: x * x for x in range(6)}
    print("Squares:", squares)
    
    # With conditional filtering
    var even_squares = {x: x * x for x in range(10) if x % 2 == 0}
    print("Even squares:", even_squares)
    
    # String keys and values
    var words = ["apple", "banana", "cherry"]
    var word_lengths = {word: len(word) for word in words}
    print("Word lengths:", word_lengths)
    
    # Complex expressions in key and value
    var nums = [1, 2, 3, 4, 5]
    var string_mapping = {"num_" + str(x): x * 2 for x in nums if x > 2}
    print("String mapping:", string_mapping)
    
    return
}

fn main() {
    test_basic_dict_comprehensions()
    return
}