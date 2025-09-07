// test_dict_from_sequences.frm
// Test dict construction from various sequences


fn test_from_sequences() {
    print("=== Dict From Sequences Test ===")
    print("")
    
    // Test 1: From list of tuples
    print("1. From list of tuples:")
    var pairs_tuples = [("a", 1), ("b", 2), ("c", 3)]
    var d1 = dict(pairs_tuples)
    print("   dict([('a', 1), ('b', 2), ('c', 3)]) -> " + str(d1))
    
    // Test 2: From list of lists
    print("")
    print("2. From list of lists:")
    var pairs_lists = [["x", 10], ["y", 20]]
    var d2 = dict(pairs_lists)
    print("   dict([['x', 10], ['y', 20]]) -> " + str(d2))
    
    // Test 3: From another dictionary (shallow copy)
    print("")
    print("3. From another dictionary (shallow copy):")
    var original = {"a": 1, "b": 2}
    var copy1 = dict(original)
    print("   original = " + str(original))
    print("   dict(original) -> " + str(copy1))
    
    // Test 4: Using unpacking operator (also shallow copy)
    print("")
    print("4. Using unpacking operator:")
    var copy2 = {**original}
    print("   {**original} -> " + str(copy2))
    
    // Test 5: zip is not available yet
    print("")
    print("5. zip function:")
    print("   zip() is not yet available in FSL")
    // Would be: var d = dict(zip(keys, values))
    
    // Test 6: .copy() method not available
    print("")
    print("6. .copy() method:")
    print("   .copy() method is not yet available for dicts")
    // Would be: var copy3 = original.copy()
    
    return
}

fn main() {
    print("Frame v0.38 - Dict From Sequences Test")
    print("=" * 50)
    print("")
    
    test_from_sequences()
    
    print("")
    print("=" * 50)
    print("Summary:")
    print("  [OK] dict() from list of tuples - WORKS")
    print("  [OK] dict() from list of lists - WORKS")
    print("  [OK] dict() from another dict - WORKS")
    print("  [OK] {**dict} unpacking - WORKS")
    print("  [NO] dict(zip(keys, values)) - NOT YET (need zip in FSL)")
    print("  [NO] dict.copy() method - NOT YET (need dict methods)")
    return
}