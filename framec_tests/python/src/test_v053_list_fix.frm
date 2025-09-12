# Test v0.53 list literal comma fix
# This test verifies that list/dict/set/tuple literals with commas work correctly
# and don't get incorrectly wrapped in tuples

fn test_list_comma_fix() {
    # v0.53 FIX: Lists with multiple elements should NOT be wrapped in tuple
    var lst1 = [1, 2, 3]
    print("List with 3 elements: " + str(lst1))
    
    # Nested lists should work correctly
    var lst2 = [[1, 2], [3, 4], [5, 6]]
    print("Nested list: " + str(lst2))
    
    # List with expressions
    var x = 10
    var lst3 = [x, x + 1, x + 2, x * 2]
    print("List with expressions: " + str(lst3))
}

fn test_dict_comma_fix() {
    # Dictionaries should also work correctly
    var dict1 = {"a": 1, "b": 2, "c": 3}
    print("Dict with 3 entries: " + str(dict1))
    
    # Dict with expression values
    var val = 100
    var dict2 = {"x": val, "y": val + 10, "z": val * 2}
    print("Dict with expressions: " + str(dict2))
}

fn test_tuple_comma_fix() {
    # Tuples should still work as expected
    var tup1 = (1, 2, 3)
    print("Tuple with 3 elements: " + str(tup1))
    
    # Single element tuple (needs trailing comma)
    var tup2 = (42,)
    print("Single element tuple: " + str(tup2))
    
    # Empty tuple
    var tup3 = ()
    print("Empty tuple: " + str(tup3))
}

fn test_set_comma_fix() {
    # Sets should work correctly
    var set1 = {1, 2, 3}
    print("Set with 3 elements: " + str(set1))
}

fn test_mixed_collections() {
    # Test that collections inside collections work correctly
    var data = {
        "list": [1, 2, 3],
        "tuple": (4, 5, 6),
        "nested": [[7, 8], [9, 10]]
    }
    print("Mixed collections: " + str(data))
    
    # List of tuples should work
    var lst_of_tuples = [(1, 2), (3, 4), (5, 6)]
    print("List of tuples: " + str(lst_of_tuples))
}

fn test_multiple_assignment_v052() {
    # v0.52 feature: Multiple assignment works in expressions but not in var declarations
    # The following would fail: var a, b, c = 100, 200, 300
    # But this works in expressions:
    var vals = (100, 200, 300)
    var a = vals[0]
    var b = vals[1] 
    var c = vals[2]
    print("Tuple unpacking workaround - a: " + str(a))
    print("Tuple unpacking workaround - b: " + str(b))
    print("Tuple unpacking workaround - c: " + str(c))
}

fn main() {
    print("=== v0.53 List Literal Comma Fix Test ===")
    print("")
    
    print("--- Testing list comma fix ---")
    test_list_comma_fix()
    print("")
    
    print("--- Testing dict comma fix ---")
    test_dict_comma_fix()
    print("")
    
    print("--- Testing tuple comma fix ---")
    test_tuple_comma_fix()
    print("")
    
    print("--- Testing set comma fix ---")
    test_set_comma_fix()
    print("")
    
    print("--- Testing mixed collections ---")
    test_mixed_collections()
    print("")
    
    print("--- Testing v0.52 multiple assignment ---")
    test_multiple_assignment_v052()
    print("")
    
    print("=== All tests complete ===")
}