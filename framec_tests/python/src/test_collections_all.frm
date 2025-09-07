// test_collections_all.frm
// Test all collection syntax in Frame v0.38


fn test_lists() {
    print("=== Testing Lists ===")
    
    // List literal
    var l = [1, 2, 3]
    print("List literal: " + str(l))
    
    // Empty list
    var empty = []
    print("Empty list: " + str(empty))
    
    // Nested lists
    var nested = [[1, 2], [3, 4]]
    print("Nested list: " + str(nested))
    
    return
}

fn test_dictionaries() {
    print("\n=== Testing Dictionaries ===")
    
    // Dictionary literal
    var d = {"a": 1, "b": 2}
    print("Dict literal: " + str(d))
    
    // Empty dictionary
    var empty = {}
    print("Empty dict: " + str(empty))
    
    // Nested dictionaries
    var nested = {"user": {"name": "Alice", "age": 30}}
    print("Nested dict: " + str(nested))
    
    return
}

fn test_sets() {
    print("\n=== Testing Sets ===")
    
    // Set literal
    var s = {1, 2, 3}
    print("Set literal: " + str(s))
    
    // Single element set
    var single = {42}
    print("Single element set: " + str(single))
    
    // Set with duplicates (should deduplicate)
    var dedup = {1, 2, 2, 3, 3, 3}
    print("Set with duplicates: " + str(dedup))
    
    return
}

fn test_tuples() {
    print("\n=== Testing Tuples ===")
    
    // Tuple literal
    var t = (10, 20, 30)
    print("Tuple literal: " + str(t))
    
    // Single element tuple (requires trailing comma)
    var single = (42,)
    print("Single element tuple: " + str(single))
    
    // Empty tuple
    var empty = ()
    print("Empty tuple: " + str(empty))
    
    // Nested tuples
    var nested = ((1, 2), (3, 4))
    print("Nested tuple: " + str(nested))
    
    return
}

fn test_mixed_collections() {
    print("\n=== Testing Mixed Collections ===")
    
    // Dict with list values
    var dict_of_lists = {"numbers": [1, 2, 3], "letters": ["a", "b", "c"]}
    print("Dict of lists: " + str(dict_of_lists))
    
    // List of tuples
    var list_of_tuples = [(1, "one"), (2, "two"), (3, "three")]
    print("List of tuples: " + str(list_of_tuples))
    
    // Set in dict
    var dict_with_set = {"unique": {1, 2, 3}, "count": 3}
    print("Dict with set: " + str(dict_with_set))
    
    // Tuple with mixed types
    var mixed_tuple = (1, "hello", [1, 2], {"key": "value"})
    print("Mixed tuple: " + str(mixed_tuple))
    
    return
}

fn main() {
    print("Testing All Collection Types in Frame v0.38")
    print("=" * 45)
    
    test_lists()
    test_dictionaries()
    test_sets()
    test_tuples()
    test_mixed_collections()
    
    print("\n" + "=" * 45)
    print("All collection tests completed!")
    return
}