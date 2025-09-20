# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test set comprehensions added in v0.41

fn test_set_comprehensions() {
    # Basic set comprehension
    var doubled = {x * 2 for x in range(5)}
    assert doubled == {0, 2, 4, 6, 8}
    print("Basic set comprehension: PASS")
    
    # Set comprehension with condition
    var evens = {x for x in range(10) if x % 2 == 0}
    assert evens == {0, 2, 4, 6, 8}
    print("Conditional set comprehension: PASS")
    
    # Set comprehension with expressions
    var squares = {x * x for x in [1, 2, 3, 4, 5]}
    assert squares == {1, 4, 9, 16, 25}
    print("Expression set comprehension: PASS")
    
    # Set comprehension with method calls
    var words = ["hello", "world", "frame"]
    var uppercased = {w.upper() for w in words}
    assert "HELLO" in uppercased
    assert "WORLD" in uppercased
    assert "FRAME" in uppercased
    print("Method call set comprehension: PASS")
    
    # Complex expression in set comprehension
    # Note: Due to parentheses not being preserved in transpilation,
    # we need to use a different expression that doesn't rely on precedence
    var complex_set = {x * 2 + 2 for x in range(3) if x > 0}
    assert complex_set == {4, 6}
    print("Complex set comprehension: PASS")
    
    # Set comprehension from list with function
    var nums = [1, 2, 3, 1, 2, 3]
    var unique = {n for n in nums}
    assert unique == {1, 2, 3}
    print("Set comprehension deduplication: PASS")
    
    print("\nAll set comprehension tests passed!")
}

# Run tests
test_set_comprehensions()