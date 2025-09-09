# Test dict.fromkeys() method

fn test_fromkeys() {
    # Test 1: All keys with same value
    var keys = ["a", "b", "c"]
    var d1 = dict.fromkeys(keys, 0)
    print("dict.fromkeys(['a', 'b', 'c'], 0):")
    print(d1)  # {'a': 0, 'b': 0, 'c': 0}
    
    # Test 2: Default value is None
    var d2 = dict.fromkeys(["x", "y", "z"])
    print("dict.fromkeys(['x', 'y', 'z']):")
    print(d2)  # {'x': None, 'y': None, 'z': None}
    
    # Test 3: From string (iterates over characters)
    var d3 = dict.fromkeys("abc", 1)
    print("dict.fromkeys('abc', 1):")
    print(d3)  # {'a': 1, 'b': 1, 'c': 1}
    
    # Test 4: From range
    var d4 = dict.fromkeys(range(3), "default")
    print("dict.fromkeys(range(3), 'default'):")
    print(d4)  # {0: 'default', 1: 'default', 2: 'default'}
    
    # Test 5: WARNING - Mutable default values share reference
    # This would be dangerous in real code:
    # var d5 = dict.fromkeys(["a", "b"], [])
    # d5["a"].append(1)  // Would affect both 'a' and 'b'!
    
    # Test 6: Better approach for mutable defaults using dict comprehension
    var d6 = {key: [] for key in ["a", "b"]}
    print("Better approach with dict comprehension:")
    print(d6)  # {'a': [], 'b': []}
}

fn main() {
    test_fromkeys()
}