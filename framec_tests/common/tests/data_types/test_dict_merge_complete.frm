# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Complete dictionary merging test

fn test_all_merge_methods() {
    print("Frame v0.38 - Dictionary Merging Methods")
    print("=" * 50)
    
    # Method 1: dict.update() - mutates original
    print("")
    print("1. dict.update() - Mutates Original")
    var d1 = {"a": 1, "b": 2}
    var d2 = {"c": 3, "d": 4}
    var d3 = {"e": 5}
    print("   Before: d1 = " + str(d1))
    d1.update(d2)
    d1.update(d3)
    print("   After d1.update(d2) and d1.update(d3):")
    print("   d1 = " + str(d1))
    
    # Method 2: Unpacking with ** operator
    print("")
    print("2. Dictionary Unpacking {**d1, **d2}")
    var dict1 = {"a": 1, "b": 2}
    var dict2 = {"c": 3, "d": 4}
    var dict3 = {"b": 99, "e": 5}
    var merged = {**dict1, **dict2, **dict3}
    print("   dict1 = " + str(dict1))
    print("   dict2 = " + str(dict2))
    print("   dict3 = " + str(dict3))
    print("   merged = {**dict1, **dict2, **dict3}")
    print("   Result: " + str(merged))
    
    # Method 3: Union operator | (Python 3.9+)
    print("")
    print("3. Union Operator | (Python 3.9+)")
    var dx = {"a": 1, "b": 2}
    var dy = {"c": 3, "d": 4}
    var dz = {"b": 100, "f": 6}
    var union_result = dx | dy | dz
    print("   dx = " + str(dx))
    print("   dy = " + str(dy))
    print("   dz = " + str(dz))
    print("   union_result = dx | dy | dz")
    print("   Result: " + str(union_result))
    
    # Method 4: dict.fromkeys()
    print("")
    print("4. dict.fromkeys() - Create with Same Value")
    var keys = ["x", "y", "z"]
    var from_keys = dict.fromkeys(keys, 0)
    print("   keys = " + str(keys))
    print("   dict.fromkeys(keys, 0) = " + str(from_keys))
    
    # Method 5: Dictionary comprehension for conditional merging
    print("")
    print("5. Dict Comprehension for Conditional Merge")
    var d_comp = {k: v for k, v in [("a", 1), ("b", 2), ("c", 3)] if v > 1}
    print("   {k: v for k, v in [('a', 1), ('b', 2), ('c', 3)] if v > 1}")
    print("   Result: " + str(d_comp))
    
    print("")
    print("=" * 50)
    print("Summary of Supported Methods:")
    print("  [OK] dict.update() - Mutates original dictionary")
    print("  [OK] {**d1, **d2} - Dictionary unpacking (creates new dict)")
    print("  [OK] d1 | d2 - Union operator (Python 3.9+, creates new dict)")
    print("  [OK] dict.fromkeys() - Create dict with same value for all keys")
    print("  [OK] Dict comprehensions - Flexible dictionary creation")
    print("")
    print("Note: In-place union (|=) requires compound assignment support")
    print("      Workaround: d1 = d1 | d2")
}

fn main() {
    test_all_merge_methods()
}