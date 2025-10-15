# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test v0.56 features: walrus operator, numeric underscores, complex numbers, type aliases

# Type aliases (Python 3.12+)
type Point = tuple[float, float]
type Matrix = list[list[float]]
type Config = dict[str, str]

fn test_walrus_operator() {
    # Basic walrus operator
    if (n := 5) > 0 {
        print("n is positive: " + str(n))
    }
    
    # Walrus in while loop (simplified - ternary in while not supported)
    var items = [1, 2, 3, 4, 5]
    var total = 0
    while len(items) > 0 {
        var item = items.pop()
        total = total + item
        print("Added " + str(item) + ", total: " + str(total))
    }
    
    # Walrus in list comprehension
    var data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    var filtered = [y for x in data if (y := x * 2) > 5]
    print("Filtered: " + str(filtered))
    
    # Walrus with expression (nested functions not supported)
    var values = [42, 100, 200]
    if (result := values[0]) == 42 {
        print("Got the answer: " + str(result))
    }
}

fn test_numeric_underscores() {
    # Decimal with underscores
    var million = 1_000_000
    print("Million: " + str(million))
    
    # Float with underscores
    var pi = 3.141_592_653
    print("Pi: " + str(pi))
    
    # Binary with underscores
    var binary = 0b1111_0000_1111_0000
    print("Binary: " + str(binary))
    
    # Hex with underscores
    var hex_val = 0xFF_FF_FF_FF
    print("Hex: " + str(hex_val))
    
    # Octal with underscores  
    var octal = 0o755_644
    print("Octal: " + str(octal))
    
    # Scientific notation (underscores in exponent not commonly used)
    var scientific = 1.234e10
    print("Scientific: " + str(scientific))
}

fn test_complex_numbers() {
    # Basic complex numbers
    var c1 = 3 + 4j
    var c2 = 2.5 - 1.5j
    print("Complex 1: " + str(c1))
    print("Complex 2: " + str(c2))
    
    # Complex arithmetic
    var sum = c1 + c2
    var product = c1 * c2
    print("Sum: " + str(sum))
    print("Product: " + str(product))
    
    # Pure imaginary
    var imaginary = 5j
    print("Pure imaginary: " + str(imaginary))
    
    # Complex with uppercase J
    var c3 = 1.1 + 2.2J
    print("Complex with J: " + str(c3))
}

fn test_type_aliases() {
    # Using type aliases (note: these are mainly for type hints, not runtime)
    var point = (3.14, 2.71)  # Would be type Point
    var matrix = [[1.0, 2.0], [3.0, 4.0]]  # Would be type Matrix
    var config = {"debug": "true", "port": "8080"}  # Would be type Config
    
    print("Point: " + str(point))
    print("Matrix: " + str(matrix))
    print("Config: " + str(config))
}

fn main() {
    print("=== Testing v0.56 Features ===")
    print("\n--- Walrus Operator ---")
    test_walrus_operator()
    print("\n--- Numeric Underscores ---")
    test_numeric_underscores()
    print("\n--- Complex Numbers ---")
    test_complex_numbers()
    print("\n--- Type Aliases ---")
    test_type_aliases()
    print("\n=== All v0.56 Features Tested ===")
}
