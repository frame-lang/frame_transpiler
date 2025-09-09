# Test dictionary merging methods

fn test_update_method() {
    print("=== Testing dict.update() ===")
    
    # Basic update
    var d1 = {"a": 1, "b": 2}
    var d2 = {"c": 3, "d": 4}
    var d3 = {"e": 5}
    
    print("Before update:")
    print("d1 = " + str(d1))
    
    # Merge into existing (mutates d1)
    d1.update(d2)
    d1.update(d3)
    print("After d1.update(d2) and d1.update(d3):")
    print("d1 = " + str(d1))  # {'a': 1, 'b': 2, 'c': 3, 'd': 4, 'e': 5}
    
    # From keyword arguments - NOT YET SUPPORTED in Frame
    # d.update(b=2, c=3)
    print("")
    print("Note: Keyword arguments in update() not yet supported")
    return
}

fn test_unpacking_merge() {
    print("")
    print("=== Testing manual merge (unpacking not supported) ===")
    
    var d1 = {"a": 1, "b": 2}
    var d2 = {"c": 3, "d": 4}
    var d3 = {"b": 99, "e": 5}
    
    # Manual merge simulation (unpacking not supported in Frame)
    var merged = {"a": 1, "b": 99, "c": 3, "d": 4, "e": 5}
    print("merged = simulated {**d1, **d2, **d3}")
    print("Result: " + str(merged))
    
    # With literals
    var merged2 = {"a": 1, "b": 2, "f": 6, "g": 7}
    print("")
    print("merged2 = simulated {**d1, 'f': 6, 'g': 7}")
    print("Result: " + str(merged2))
    return
}

fn test_union_operator() {
    print("")
    print("=== Testing manual union (operators not supported) ===")
    
    var d1 = {"a": 1, "b": 2}
    var d2 = {"c": 3, "d": 4}
    
    # Manual union simulation (operators not supported in Frame)
    var merged = {"a": 1, "b": 2, "c": 3, "d": 4}
    print("merged = simulated d1 | d2")
    print("Result: " + str(merged))
    
    # In-place union simulation
    print("")
    print("Before simulated union: d1 = " + str(d1))
    d1 = {"a": 1, "b": 2, "c": 3, "d": 4}
    print("After simulated d1 |= d2")
    print("d1 = " + str(d1))
    return
}

fn main() {
    test_update_method()
    test_unpacking_merge()
    test_union_operator()
    return
}