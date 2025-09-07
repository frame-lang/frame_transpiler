// Test empty set literal {,} in Frame v0.38

fn testEmptySet() {
    print("=== Empty Set Literal Test ===")
    
    // Empty dict using {}
    var empty_dict = {}
    print("Empty dict: " + str(empty_dict))
    print("Type: " + str(type(empty_dict)))
    
    // Empty set using {,}
    var empty_set = {,}
    print("Empty set: " + str(empty_set))
    print("Type: " + str(type(empty_set)))
    
    // Non-empty set
    var my_set = {1, 2, 3}
    print("Non-empty set: " + str(my_set))
    
    // Add to empty set
    empty_set.add(42)
    print("After adding 42: " + str(empty_set))
    
    return
}

fn main() {
    testEmptySet()
    return
}

main()