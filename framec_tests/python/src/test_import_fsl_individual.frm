// Test individual FSL imports - v0.34 comprehensive test
// Expected output: All FSL operations work correctly with individual imports

// Individual FSL imports - specific functions only

fn test_str_operations() {
    var num = 42
    var pi = 3.14159
    var flag = true
    
    var num_str = str(num)
    var pi_str = str(pi)
    var flag_str = str(flag)
    
    print("str(42): " + num_str)           // Expected: str(42): 42
    print("str(3.14159): " + pi_str)       // Expected: str(3.14159): 3.14159
    print("str(true): " + flag_str)        // Expected: str(true): True
}

fn test_int_operations() {
    var text = "123"
    var decimal = 45.67
    var bool_val = true
    
    var from_text = int(text)
    var from_decimal = int(decimal)
    var from_bool = int(bool_val)
    
    print("int('123'): " + str(from_text))      // Expected: int('123'): 123
    print("int(45.67): " + str(from_decimal))   // Expected: int(45.67): 45
    print("int(true): " + str(from_bool))       // Expected: int(true): 1
}

fn test_float_operations() {
    var text = "3.14"
    var whole = 42
    
    var from_text = float(text)
    var from_int = float(whole)
    
    print("float('3.14'): " + str(from_text))   // Expected: float('3.14'): 3.14
    print("float(42): " + str(from_int))        // Expected: float(42): 42.0
}

fn test_nested_operations() {
    // Test nested FSL operations
    var temp1 = float("123.456")
    var complex = int(temp1)
    var temp2 = int("999")
    var formatted = str(temp2)
    
    print("int(float('123.456')): " + str(complex))  // Expected: int(float('123.456')): 123
    print("str(int('999')): " + formatted)           // Expected: str(int('999')): 999
}

fn test_in_system_context() {
    // Test FSL operations that would be in system context
    var value = 100
    var converted = str(value)
    print("System context str(100): " + converted)
    
    var input = "456"
    var result = int(input)
    var result_str = str(result)
    print("System context int('456'): " + result_str)
}

fn main() {
    print("=== Testing Individual FSL Imports ===")
    
    test_str_operations()
    test_int_operations() 
    test_float_operations()
    test_nested_operations()
    test_in_system_context()
    
    print("=== All individual FSL import tests complete ===")
}