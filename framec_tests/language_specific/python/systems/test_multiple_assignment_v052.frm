# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Frame v0.52 - Multiple Assignment and Tuple Unpacking
# This test demonstrates the new multiple assignment and tuple unpacking features

fn test_basic_assignment() {
    # Basic multiple assignment
    x = 1
    y = 2
    print("Before: x=" + str(x) + ", y=" + str(y))
    
    x, y = 10, 20
    print("After: x=" + str(x) + ", y=" + str(y))
}

fn test_swap() {
    # Variable swapping
    a = 5
    b = 10
    print("Before swap: a=" + str(a) + ", b=" + str(b))
    
    a, b = b, a
    print("After swap: a=" + str(a) + ", b=" + str(b))
}

fn test_tuple_unpack() {
    # Unpacking from a tuple
    t = (100, 200, 300)
    p = 0
    q = 0
    r = 0
    
    p, q, r = t
    print("Unpacked from tuple: p=" + str(p) + ", q=" + str(q) + ", r=" + str(r))
}

fn test_list_unpack() {
    # Unpacking from a list
    # Note: Currently Frame converts [1, 2, 3] to [(1, 2, 3)] due to RHS parsing
    # This is a known limitation in v0.52
    lst = [1]
    lst.append(2)
    lst.append(3)
    x1 = 0
    y1 = 0
    z1 = 0
    
    x1, y1, z1 = lst
    print("Unpacked from list: x1=" + str(x1) + ", y1=" + str(y1) + ", z1=" + str(z1))
}

fn get_coordinates() {
    return (42, 73)
}

fn test_function_return() {
    # Function returning multiple values
    lat = 0
    lon = 0
    lat, lon = get_coordinates()
    print("Coordinates: lat=" + str(lat) + ", lon=" + str(lon))
}

fn test_complex_expressions() {
    # Multiple assignment with expressions
    n1 = 1
    n2 = 2
    n3 = 3
    
    n1, n2, n3 = n1 + 1, n2 * 2, n3 ** 2
    print("After expressions: n1=" + str(n1) + ", n2=" + str(n2) + ", n3=" + str(n3))
}

fn main() {
    print("=== Frame v0.52 Multiple Assignment Test ===")
    print("")
    
    test_basic_assignment()
    print("")
    
    test_swap()
    print("")
    
    test_tuple_unpack()
    print("")
    
    test_list_unpack()
    print("")
    
    test_function_return()
    print("")
    
    test_complex_expressions()
    
    print("")
    print("=== All Tests Complete ===")
}