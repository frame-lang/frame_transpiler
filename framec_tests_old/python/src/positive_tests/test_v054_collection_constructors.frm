# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Frame v0.54 - Collection Constructor Arguments

fn test_list_constructor() {
    # Create list from iterable
    var from_string = list("hello")
    print("list('hello') = " + str(from_string))
    
    var from_tuple = list((1, 2, 3, 4, 5))
    print("list((1,2,3,4,5)) = " + str(from_tuple))
    
    # Empty list constructor
    var empty = list()
    print("list() = " + str(empty))
    
    # From range
    var from_range = list(range(5))
    print("list(range(5)) = " + str(from_range))
}

fn test_dict_constructor() {
    # Empty dict
    var empty_dict = dict()
    print("dict() = " + str(empty_dict))
    
    # Dict from list of tuples
    var pairs = [("a", 1), ("b", 2), ("c", 3)]
    var from_pairs = dict(pairs)
    print("dict from pairs = " + str(from_pairs))
    
    # Dict from list comprehension (simpler approach)
    var keys = ["x", "y", "z"]
    var values = [10, 20, 30]
    # Note: zip with multiple args doesn't parse correctly yet, using pairs instead
    var manual_pairs = [("x", 10), ("y", 20), ("z", 30)]
    var from_manual = dict(manual_pairs)
    print("dict from manual pairs = " + str(from_manual))
}

fn test_set_constructor() {
    # Empty set
    var empty_set = set()
    print("set() = " + str(empty_set))
    
    # Set from list
    var from_list = set([1, 2, 3, 2, 1, 4])
    print("set([1,2,3,2,1,4]) = " + str(from_list))
    
    # Set from string
    var from_string = set("hello")
    print("set('hello') = " + str(from_string))
}

fn test_tuple_constructor() {
    # Empty tuple
    var empty_tuple = tuple()
    print("tuple() = " + str(empty_tuple))
    
    # Tuple from list
    var from_list = tuple([1, 2, 3])
    print("tuple([1,2,3]) = " + str(from_list))
    
    # Tuple from single-arg range
    var from_range = tuple(range(5))
    print("tuple(range(5)) = " + str(from_range))
}

fn test_type_conversions() {
    # String conversion
    var num_str = str(42)
    print("str(42) = " + num_str)
    
    # Int conversion
    var str_int = int("123")
    print("int('123') = " + str(str_int))
    
    # Float conversion
    var str_float = float("3.14")
    print("float('3.14') = " + str(str_float))
    
    # Bool conversion
    var bool_empty = bool([])
    var bool_full = bool([1])
    print("bool([]) = " + str(bool_empty))
    print("bool([1]) = " + str(bool_full))
}

fn test_complex_constructors() {
    # Nested conversions
    var d = dict([("a", 1), ("b", 2)])
    var nested = list(d.keys())
    print("list(dict.keys()) = " + str(nested))
    
    # Chained operations
    var s = set([3, 1, 2, 1, 3, 2])
    var unique_sorted = list(s)
    unique_sorted.sort()
    print("sorted unique = " + str(unique_sorted))
}

fn main() {
    print("=== List Constructor ===")
    test_list_constructor()
    print("\n=== Dict Constructor ===")
    test_dict_constructor()
    print("\n=== Set Constructor ===")
    test_set_constructor()
    print("\n=== Tuple Constructor ===")
    test_tuple_constructor()
    print("\n=== Type Conversions ===")
    test_type_conversions()
    print("\n=== Complex Constructors ===")
    test_complex_constructors()
    print("\n=== All tests complete ===")
}
