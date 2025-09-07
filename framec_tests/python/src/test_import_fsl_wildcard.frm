// Test wildcard FSL imports - v0.34
// Expected output: All FSL operations work with wildcard import

// Wildcard FSL import - should import all available FSL functions

fn test_all_conversions() {
    // Test all conversion functions available via wildcard
    print("=== Testing All FSL Conversions via Wildcard ===")
    
    // str() conversion
    var num = 42
    var pi = 3.14159
    var flag = true
    print("str(42): " + str(num))
    print("str(3.14159): " + str(pi))
    print("str(true): " + str(flag))
    
    // int() conversion  
    var text_int = "123"
    var decimal = 45.67
    print("int('123'): " + str(int(text_int)))
    print("int(45.67): " + str(int(decimal)))
    print("int(true): " + str(int(true)))
    
    // float() conversion
    var text_float = "99.9"
    var whole = 100
    print("float('99.9'): " + str(float(text_float)))
    print("float(100): " + str(float(whole)))
    
    // bool() conversion - if available
    var zero = 0
    var one = 1
    var empty = ""
    print("bool(0): " + str(bool(zero)))
    print("bool(1): " + str(bool(one)))
    print("bool(''): " + str(bool(empty)))
}

fn test_combined_operations() {
    print("=== Testing Combined FSL Operations ===")
    
    // Combining multiple FSL operations
    var input = "456"
    var as_int = int(input)
    var as_float = float(as_int)
    var as_str = str(as_float)
    
    print("Original input: " + input)
    print("After int(): " + str(as_int))
    print("After float(): " + str(as_float))
    print("Final str(): " + as_str)
    
    // Chain operations with temp variables
    var temp1 = float("78.9")
    var temp2 = int(temp1)
    var result = str(temp2)
    print("float('78.9') -> int() -> str(): " + result)
}

fn main() {
    print("=== FSL Wildcard Import Test ===")
    test_all_conversions()
    test_combined_operations()
    print("=== Test Complete ===")
}