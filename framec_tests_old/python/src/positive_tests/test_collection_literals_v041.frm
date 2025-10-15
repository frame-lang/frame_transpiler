# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test Collection Literals - v0.41 Status Check
# What actually works vs what's documented

fn main() {
    print("=== Frame v0.41 Collection Support ===\n")
    
    # ✅ LIST LITERALS - FULLY WORKING
    print("1. Lists:")
    var list1 = [1, 2, 3]
    var list2 = []
    var nested_list = [[1, 2], [3, 4]]
    list1.append(4)
    print("  Basic: " + str(list1))
    print("  Empty: " + str(list2))
    print("  Nested: " + str(nested_list))
    
    # ✅ DICTIONARY LITERALS - FULLY WORKING
    print("\n2. Dictionaries:")
    var dict1 = {"a": 1, "b": 2}
    var dict2 = {}
    var nested_dict = {"outer": {"inner": "value"}}
    dict1["c"] = 3
    print("  Basic: " + str(dict1))
    print("  Empty: " + str(dict2))
    print("  Nested: " + str(nested_dict))
    
    # ✅ SET LITERALS - WORKING
    print("\n3. Sets:")
    var set1 = {1, 2, 3}
    var set2 = {,}  # Empty set special syntax
    set1.add(4)
    set2.add(42)
    print("  Basic: " + str(set1))
    print("  Empty: " + str(set2))
    
    # ✅ TUPLE LITERALS - WORKING
    print("\n4. Tuples:")
    var tuple1 = (1, 2, 3)
    var tuple2 = ()
    var tuple3 = (42,)  # Single element
    print("  Basic: " + str(tuple1))
    print("  Empty: " + str(tuple2))
    print("  Single: " + str(tuple3))
    
    # ✅ CONSTRUCTORS - WORKING
    print("\n5. Constructors:")
    var list_c = list()
    var dict_c = dict()
    var set_c = set()
    var tuple_c = tuple()
    print("  All empty constructors work")
    
    # ✅ LIST COMPREHENSIONS - WORKING
    print("\n6. List Comprehensions:")
    var squares = [x * x for x in range(5)]
    var filtered = [x for x in range(10) if x % 2 == 0]
    print("  Squares: " + str(squares))
    print("  Filtered: " + str(filtered))
    
    # ✅ DICT COMPREHENSIONS - WORKING
    print("\n7. Dict Comprehensions:")
    var dict_comp = {str(x): x*x for x in range(3)}
    print("  Dict comp: " + str(dict_comp))
    
    # ❌ SET COMPREHENSIONS - NOT WORKING
    # The following would fail:
    # var set_comp = {x*2 for x in range(4)}
    print("\n8. Set Comprehensions: NOT WORKING (parser bug)")
    
    print("\n=== SUMMARY ===")
    print("✅ Lists: Full support (literals, comprehensions)")
    print("✅ Dicts: Full support (literals, comprehensions)")
    print("✅ Sets: Partial (literals work, comprehensions broken)")
    print("✅ Tuples: Full support (literals)")
    print("✅ Constructors: All work (empty only)")
}
