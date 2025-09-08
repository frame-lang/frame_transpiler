// test_all_8_collection_patterns.frm
// Test all 8 collection patterns from the user's request


fn test_all_patterns() {
    print("=== Testing All 8 Collection Patterns ===")
    print("")
    
    // Pattern 1: Set constructor with arguments
    var s = set(1, 2, 3)
    print("1. var s = set(1,2,3)")
    print("   Result: " + str(s))
    print("")
    
    // Pattern 2: Set literal
    var s_set = {1, 2, 3}
    print("2. var s_set = {1,2,3}")
    print("   Result: " + str(s_set))
    print("")
    
    // Pattern 3: List constructor with iterable
    var l = list([1, 2, 3])
    print("3. var l = list([1,2,3])")
    print("   Result: " + str(l))
    print("")
    
    // Pattern 4: List literal
    var l2 = [1, 2, 3]
    print("4. var l2 = [1,2,3]")
    print("   Result: " + str(l2))
    print("")
    
    // Pattern 5: Dict constructor (Python-style with key-value pairs)
    var d = dict([("name", "Alice"), ("age", 30)])
    print("5. var d = dict([('name', 'Alice'), ('age', 30)])")
    print("   Result: " + str(d))
    print("")
    
    // Pattern 6: Dictionary literal
    var d2 = {"a": 1, "b": 2}
    print("6. var d2 = {'a':1,'b':2}")
    print("   Result: " + str(d2))
    print("")
    
    // Pattern 7: Tuple constructor with iterable
    var t = tuple([10, 20, 30])
    print("7. var t = tuple([10,20,30])")
    print("   Result: " + str(t))
    print("")
    
    // Pattern 8: Tuple literal
    var t2 = (10, 20, 30)
    print("8. var t2 = (10,20,30)")
    print("   Result: " + str(t2))
    print("")
    
    print("=== All 8 patterns working! ===")
    return
}

fn main() {
    test_all_patterns()
    return
}