// Test FSL conversion operations (str, int, float) without backticks

fn test_string_conversion() {
    var num = 42
    var pi = 3.14159
    var flag = true
    
    // FSL operations without backticks!
    var num_str = str(num)
    var pi_str = str(pi)
    var flag_str = str(flag)
    
    print("Number as string:", num_str)
    print("Pi as string:", pi_str)
    print("Boolean as string:", flag_str)
}

fn test_int_conversion() {
    var text = "123"
    var decimal = 45.67
    var bool_val = true
    
    // FSL int conversions
    var from_text = int(text)
    var from_decimal = int(decimal)
    var from_bool = int(bool_val)
    
    print("String to int:", from_text)
    print("Float to int:", from_decimal)
    print("Bool to int:", from_bool)
}

fn test_float_conversion() {
    var text = "3.14"
    var whole = 42
    
    // FSL float conversions
    var from_text = float(text)
    var from_int = float(whole)
    
    print("String to float:", from_text)
    print("Int to float:", from_int)
}

fn test_complex_expressions() {
    var x = 10
    var y = 3
    
    // Using FSL operations in expressions
    var result = float(x) / float(y)
    var formatted = str(result)
    
    print("Division result:", formatted)
    
    // Nested conversions
    var complex = int(float("123.456"))
    print("Nested conversion:", complex)
}

fn main() {
    print("=== Testing FSL Conversion Operations ===")
    test_string_conversion()
    test_int_conversion()
    test_float_conversion()
    test_complex_expressions()
    print("=== All tests complete ===")
}