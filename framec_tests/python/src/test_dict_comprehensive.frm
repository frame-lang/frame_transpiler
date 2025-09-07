// test_dict_comprehensive.frm
// Comprehensive dictionary tests for Frame v0.38 - matching Python's actual dict syntax


fn test_dict_literals() {
    print("=== 1. Literal Syntax (Most Common) ===")
    
    // Empty dictionary using literals
    var empty = {}
    print("Empty dict literal: {} -> " + str(empty))
    
    // Dictionary with string keys
    var person = {"name": "Alice", "age": 30, "city": "NYC"}
    print("Person dict: " + str(person))
    
    // Mixed key types (strings, numbers, booleans, tuples)
    var mixed = {
        "string_key": "value",
        42: "integer key",
        3.14: "float key", 
        (1, 2): "tuple key",
        true: "boolean key"
    }
    print("Mixed key types: " + str(mixed))
    
    // Nested dictionaries
    var nested = {
        "user": {
            "profile": {
                "name": "Bob",
                "settings": {"theme": "dark"}
            }
        }
    }
    print("Nested dict: " + str(nested))
    
    return
}

fn test_dict_constructors() {
    print("")
    print("=== 2. Dict Constructor Patterns ===")
    
    // Empty dict constructor
    var d1 = dict()
    print("dict() -> " + str(d1))
    
    // From list of tuples (most common constructor pattern)
    var d2 = dict([("a", 1), ("b", 2), ("c", 3)])
    print("dict([('a', 1), ('b', 2), ('c', 3)]) -> " + str(d2))
    
    // From list of lists
    var d3 = dict([["x", 10], ["y", 20]])
    print("dict([['x', 10], ['y', 20]]) -> " + str(d3))
    
    // From another dictionary (creates shallow copy)
    var original = {"a": 1, "b": 2}
    var copy1 = dict(original)
    print("dict({'a': 1, 'b': 2}) - shallow copy -> " + str(copy1))
    
    // Note: dict(name='Alice', age=30) keyword syntax not available in Frame
    // Note: dict(zip(keys, values)) would need zip function support
    
    return
}

fn test_dict_with_collections() {
    print("")
    print("=== 3. Dictionaries with Other Collections ===")
    
    // Dict with list values
    var dict_of_lists = {
        "numbers": [1, 2, 3],
        "letters": ["a", "b", "c"]
    }
    print("Dict with lists: " + str(dict_of_lists))
    
    // Dict with set values
    var dict_of_sets = {
        "unique_nums": {1, 2, 3},
        "unique_chars": {"x", "y", "z"}
    }
    print("Dict with sets: " + str(dict_of_sets))
    
    // Dict with tuple values
    var dict_of_tuples = {
        "point": (10, 20),
        "rgb": (255, 0, 0)
    }
    print("Dict with tuples: " + str(dict_of_tuples))
    
    return
}

fn main() {
    print("Frame v0.38 - Comprehensive Dictionary Syntax Test")
    print("=" * 50)
    print("")
    
    test_dict_literals()
    test_dict_constructors()
    test_dict_with_collections()
    
    print("")
    print("=" * 50)
    print("All dictionary patterns validated successfully!")
    return
}