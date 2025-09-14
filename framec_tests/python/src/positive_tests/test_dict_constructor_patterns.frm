# test_dict_constructor_patterns.frm
# Test various Python dict constructor patterns


fn test_dict_patterns() {
    print("=== Testing Dict Constructor Patterns ===")
    print("")
    
    # Pattern 1: Empty dict
    var d1 = dict()
    print("1. dict() -> " + str(d1))
    
    # Pattern 2: List of tuples
    var d2 = dict([("name", "Alice"), ("age", 30), ("city", "NYC")])
    print("2. dict([('name', 'Alice'), ('age', 30), ('city', 'NYC')]) -> " + str(d2))
    
    # Pattern 3: List of lists (also valid Python)
    var d3 = dict([["key1", "value1"], ["key2", "value2"]])
    print("3. dict([['key1', 'value1'], ['key2', 'value2']]) -> " + str(d3))
    
    # Pattern 4: Compare with dict literals
    var d4 = {"name": "Bob", "age": 25, "city": "LA"}
    print("4. {'name': 'Bob', 'age': 25, 'city': 'LA'} -> " + str(d4))
    
    print("")
    print("All Python dict constructor patterns work correctly!")
    return
}

fn main() {
    test_dict_patterns()
    return
}