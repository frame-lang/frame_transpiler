@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Frame v0.56 - Walrus Operator, Numeric Underscores, and Complex Numbers

fn test_walrus_operator() {
    # Basic walrus operator usage
    if (n := len([1, 2, 3, 4, 5])) > 3:
        print("List is long: " + str(n) + " elements")
    
    # Walrus in while loop (simplified for now)
    counter = 0
    while (val := counter) < 5:
        print("Counter: " + str(val))
        counter = counter + 1
    
    # Walrus in list comprehension
    results = [y for x in range(10) if (y := x * 2) > 5]
    print("Filtered results: " + str(results))
}

fn test_numeric_underscores() {
    # Decimal numbers with underscores
    million = 1_000_000
    billion = 1_000_000_000
    print("Million: " + str(million))
    print("Billion: " + str(billion))
    
    # Float with underscores
    pi_approx = 3.141_592_653
    print("Pi: " + str(pi_approx))
    
    # Binary literal with underscores
    binary = 0b1111_0000_1111_0000
    print("Binary: " + str(binary))
    
    # Hexadecimal with underscores
    hex_val = 0xFF_FF_00_00
    print("Hex: " + str(hex_val))
    
    # Octal with underscores
    octal = 0o777_666_555
    print("Octal: " + str(octal))
}

fn test_complex_numbers() {
    # Complex number literals
    z1 = 3.5j
    print("Pure imaginary: " + str(z1))
    
    # Complex arithmetic (requires complex() function in Python)
    z2 = 2 + 3j
    z3 = 4 - 2j
    print("Complex 1: " + str(z2))
    print("Complex 2: " + str(z3))
    
    # Complex operations
    z_sum = z2 + z3
    z_product = z2 * z3
    print("Sum: " + str(z_sum))
    print("Product: " + str(z_product))
    
    # Complex with underscores
    big_complex = 1_000 + 2_000j
    print("Big complex: " + str(big_complex))
}

fn test_combined_features() {
    # Combine walrus with complex numbers
    if (z := 3 + 4j) != 0:
        print("Complex walrus: " + str(z))
    
    # Walrus with underscored numbers
    if (big := 1_000_000) > 999_999:
        print("Big number via walrus: " + str(big))
    
    # All features in one expression
    nums = [1_000, 2_000, 3_000]
    complex_list = [(z := n + 0.5j) for n in nums if n > 1_500]
    print("Complex filtered list: " + str(complex_list))
}

fn main() {
    print("=== Testing Walrus Operator ===")
    test_walrus_operator()
    
    print("\n=== Testing Numeric Underscores ===")
    test_numeric_underscores()
    
    print("\n=== Testing Complex Numbers ===")
    test_complex_numbers()
    
    print("\n=== Testing Combined Features ===")
    test_combined_features()
    
    print("\n=== All v0.56 features test complete ===")
}
