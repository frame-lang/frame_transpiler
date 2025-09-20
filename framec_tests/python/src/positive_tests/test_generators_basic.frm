# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test basic generator features in Frame v0.42

# Test generator function with yield
fn count_up_to(n) {
    var i = 0
    while i < n {
        yield i
        i = i + 1
    }
}

# Test generator function with yield from
fn delegated_generator() {
    yield from range(3)
    yield from [10, 20, 30]
}

# Test generator expressions
fn test_generator_expressions() {
    # Generator expression
    var gen = (x * 2 for x in range(5))
    
    # Generator with condition
    var even_gen = (x for x in range(10) if x % 2 == 0)
    
    # Convert to list to see results
    var results = list(gen)
    print("Generator results: " + str(results))
    
    var even_results = list(even_gen)
    print("Even generator results: " + str(even_results))
}

# Run tests
print("=== Testing Generators ===")
test_generator_expressions()

print("\n=== Testing yield functions ===")
for val in count_up_to(5) {
    print("Value: " + str(val))
}

print("\n=== Testing yield from ===")
for val in delegated_generator() {
    print("Delegated: " + str(val))
}