# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test dictionary union operator

fn test_union_operator() {
    print("=== Testing union operator (|) ===")
    
    var d1 = {"a": 1, "b": 2}
    var d2 = {"c": 3, "d": 4}
    
    # Union operator
    var merged = d1 | d2
    print("d1 = " + str(d1))
    print("d2 = " + str(d2))
    print("merged = d1 | d2")
    print("Result: " + str(merged))  # {'a': 1, 'b': 2, 'c': 3, 'd': 4}
    
    # Multiple unions
    var d3 = {"e": 5, "b": 99}
    var merged2 = d1 | d2 | d3
    print("")
    print("d3 = " + str(d3))
    print("merged2 = d1 | d2 | d3")
    print("Result: " + str(merged2))  # {'a': 1, 'b': 99, 'c': 3, 'd': 4, 'e': 5}
}

fn main() {
    test_union_operator()
}