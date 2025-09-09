# Test dictionary update method

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
}

fn test_unpacking_merge() {
    print("")
    print("=== Testing unpacking merge ===")
    
    var d1 = {"a": 1, "b": 2}
    var d2 = {"c": 3, "d": 4}
    var d3 = {"b": 99, "e": 5}
    
    # Merge multiple dicts (later values override)
    var merged = {**d1, **d2, **d3}
    print("merged = {**d1, **d2, **d3}")
    print("Result: " + str(merged))  # {'a': 1, 'b': 99, 'c': 3, 'd': 4, 'e': 5}
    
    # With literals
    var merged2 = {**d1, "f": 6, "g": 7}
    print("")
    print("merged2 = {**d1, 'f': 6, 'g': 7}")
    print("Result: " + str(merged2))  # {'a': 1, 'b': 2, 'f': 6, 'g': 7}
}

fn main() {
    test_update_method()
    test_unpacking_merge()
}