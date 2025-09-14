# Test dictionary comprehensions from the user's images

fn test_basic() {
    # Basic dictionary comprehension - square numbers as values
    var squares = {x: x * x for x in range(5)}
    print(squares)  # {0: 0, 1: 1, 2: 4, 3: 9, 4: 16}
}

fn test_with_conditional() {
    # With conditional - only even numbers
    var even_squares = {x: x * x for x in range(10) if x % 2 == 0}
    print(even_squares)  # {0: 0, 2: 4, 4: 16, 6: 36, 8: 64}
}

fn test_complex_filtering() {
    # Complex filtering - words longer than 4 characters
    var words = ["hello", "world", "hi", "test", "python", "frame"]
    var word_lengths = {word: len(word) for word in words if len(word) > 4}
    print(word_lengths)  # {"hello": 5, "world": 5, "python": 6, "frame": 5}
}

fn test_complex_expressions() {
    # Complex expression in key and value
    var nums = [1, 2, 3, 4, 5]
    var complex_dict = {str(x): x * 2 + 1 for x in nums if x > 2}
    print(complex_dict)  # {"3": 7, "4": 9, "5": 11}
}

fn main() {
    test_basic()
    test_with_conditional()
    test_complex_filtering()
    test_complex_expressions()
}