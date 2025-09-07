// test_dict_unpacking.frm
// Test dictionary unpacking operator (**) support in Frame v0.38


fn test_dict_unpacking() {
    print("=== Testing Dict Unpacking Operator (**) ===")
    print("")
    
    // Create some base dictionaries
    var base = {"a": 1, "b": 2}
    var extra = {"c": 3, "d": 4}
    
    // Test 1: Basic unpacking
    print("1. Basic unpacking:")
    var combined = {**base}
    print("   {**base} -> " + str(combined))
    
    // Test 2: Combining two dicts
    print("")
    print("2. Combining two dicts:")
    var merged = {**base, **extra}
    print("   {**base, **extra} -> " + str(merged))
    
    // Test 3: Unpacking with overrides
    print("")
    print("3. Unpacking with overrides:")
    var updated = {**base, "b": 20, "e": 5}
    print("   {**base, 'b': 20, 'e': 5} -> " + str(updated))
    
    // Test 4: Multiple unpacking with additional keys
    print("")
    print("4. Multiple unpacking with additional keys:")
    var complex_merge = {"x": 0, **base, "y": 10, **extra, "z": 100}
    print("   {'x': 0, **base, 'y': 10, **extra, 'z': 100} -> " + str(complex_merge))
    
    // Test 5: Unpacking at the beginning
    print("")
    print("5. Unpacking at the beginning:")
    var start_unpack = {**extra, "a": 10}
    print("   {**extra, 'a': 10} -> " + str(start_unpack))
    
    // Test 6: Chain unpacking (later values override earlier)
    print("")
    print("6. Chain unpacking (override behavior):")
    var override1 = {"a": 1, "b": 2}
    var override2 = {"b": 20, "c": 30}
    var result = {**override1, **override2}
    print("   {**{'a': 1, 'b': 2}, **{'b': 20, 'c': 30}} -> " + str(result))
    print("   Note: 'b' should be 20 (from override2)")
    
    return
}

fn main() {
    print("Frame v0.38 - Dict Unpacking Operator Test")
    print("=" * 50)
    print("")
    
    test_dict_unpacking()
    
    print("")
    print("=" * 50)
    print("All dict unpacking tests completed!")
    return
}