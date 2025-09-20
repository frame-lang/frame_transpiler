# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Frame v0.53 - Multiple Variable Declarations Test

fn test_multi_var_basic() {
    # Basic multiple variable declaration
    var x, y, z = 1, 2, 3
    print("x = " + str(x))
    print("y = " + str(y))
    print("z = " + str(z))
    
    # Multiple variables with different types
    var a, b, c = "hello", 42, 3.14
    print("a = " + a)
    print("b = " + str(b))
    print("c = " + str(c))
}

fn test_multi_var_unpacking() {
    # From tuple
    var tuple_val = (10, 20, 30)
    var p, q, r = tuple_val
    print("p = " + str(p))
    print("q = " + str(q))
    print("r = " + str(r))
    
    # From list
    var list_val = [100, 200, 300]
    var m, n, o = list_val
    print("m = " + str(m))
    print("n = " + str(n))
    print("o = " + str(o))
}

fn get_coords() {
    return (5, 10, 15)
}

fn test_multi_var_function() {
    # Unpack function return
    var x, y, z = get_coords()
    print("coords: " + str(x) + ", " + str(y) + ", " + str(z))
}

fn test_multi_var_mixed() {
    # Mixed with single declarations
    var single = 100
    var multi_a, multi_b = 200, 300
    var another_single = 400
    
    print("single = " + str(single))
    print("multi_a = " + str(multi_a))
    print("multi_b = " + str(multi_b))
    print("another_single = " + str(another_single))
}

fn main() {
    print("=== Multiple Variable Declarations Test ===")
    test_multi_var_basic()
    test_multi_var_unpacking()
    test_multi_var_function()
    test_multi_var_mixed()
    print("=== All tests complete ===")
}

main()