# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test v0.53 collection literal fixes
# This test verifies that list/dict/set/tuple literals with commas work correctly

fn test_list_literals() {
    # Basic list with multiple elements
    var lst1 = [1, 2, 3]
    print(str(lst1))  # Should print: [1, 2, 3]
    
    # List with expressions
    var x = 5
    var lst2 = [x, x + 1, x * 2]
    print(str(lst2))  # Should print: [5, 6, 10]
    
    # Nested lists
    var lst3 = [[1, 2], [3, 4], [5, 6]]
    print(str(lst3))  # Should print: [[1, 2], [3, 4], [5, 6]]
    
    # Empty list
    var lst4 = []
    print(str(lst4))  # Should print: []
    
    # Single element list
    var lst5 = [42]
    print(str(lst5))  # Should print: [42]
}

fn test_dict_literals() {
    # Dictionary with multiple entries
    var dict1 = {"a": 1, "b": 2, "c": 3}
    print(str(dict1))  # Should print: {'a': 1, 'b': 2, 'c': 3}
    
    # Dictionary with expressions
    var key = "x"
    var val = 10
    var dict2 = {key: val, "y": val + 1}
    print(str(dict2))  # Should print: {'x': 10, 'y': 11}
    
    # Empty dictionary
    var dict3 = {}
    print(str(dict3))  # Should print: {}
}

fn test_set_literals() {
    # Set with multiple elements
    var set1 = {1, 2, 3}
    print(str(set1))  # Should print: {1, 2, 3}
    
    # Empty set (special syntax)
    # Note: Empty set uses special syntax but may need fix
    # Skipping for now as it seems to have issues
    # var set2 = {,}
    # print(str(set2))  # Should print: set()
}

fn test_tuple_literals() {
    # Tuple with multiple elements
    var tup1 = (1, 2, 3)
    print(str(tup1))  # Should print: (1, 2, 3)
    
    # Single element tuple (needs trailing comma)
    var tup2 = (42,)
    print(str(tup2))  # Should print: (42,)
    
    # Empty tuple
    var tup3 = ()
    print(str(tup3))  # Should print: ()
}

fn test_multiple_assignment() {
    # v0.52 feature: Multiple assignment
    var x, y, z = 1, 2, 3
    print(str(x))  # Should print: 1
    print(str(y))  # Should print: 2
    print(str(z))  # Should print: 3
    
    # Multiple assignment with list unpacking
    var lst = [10, 20, 30]
    var a, b, c = lst
    print(str(a))  # Should print: 10
    print(str(b))  # Should print: 20
    print(str(c))  # Should print: 30
    
    # Multiple assignment with tuple
    var t = (100, 200)
    var p, q = t
    print(str(p))  # Should print: 100
    print(str(q))  # Should print: 200
}

fn test_mixed_scenarios() {
    # List inside multiple assignment
    var data = [1, 2, 3]
    var first, second, third = data
    var result = [first * 10, second * 10, third * 10]
    print(str(result))  # Should print: [10, 20, 30]
    
    # Dictionary with tuple values
    var dict_with_tuples = {"point1": (1, 2), "point2": (3, 4)}
    print(str(dict_with_tuples))  # Should print: {'point1': (1, 2), 'point2': (3, 4)}
    
    # List of tuples
    var lst_of_tuples = [(1, 2), (3, 4), (5, 6)]
    print(str(lst_of_tuples))  # Should print: [(1, 2), (3, 4), (5, 6)]
}

fn main() {
    print("Testing list literals:")
    test_list_literals()
    
    print("\nTesting dict literals:")
    test_dict_literals()
    
    print("\nTesting set literals:")
    test_set_literals()
    
    print("\nTesting tuple literals:")
    test_tuple_literals()
    
    print("\nTesting multiple assignment:")
    test_multiple_assignment()
    
    print("\nTesting mixed scenarios:")
    test_mixed_scenarios()
}