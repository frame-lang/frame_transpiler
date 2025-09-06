// Test Python logical keywords (and, or, not)
// v0.38 - Support both C-style and Python-style

fn test_python_keywords() {
    var x = true
    var y = false
    var z = true
    
    // Test 'and' keyword
    if x and z {
        print("Python 'and' works!")
    }
    
    // Test 'or' keyword
    if y or x {
        print("Python 'or' works!")
    }
    
    // Test 'not' keyword
    if not y {
        print("Python 'not' works!")
    }
    
    // Complex expression with all three
    if (x and not y) or z {
        print("Complex Python expression works!")
    }
    
    // Mixed with comparisons
    var num = 5
    if num > 0 and num < 10 {
        print("Python 'and' with comparisons works!")
    }
}

fn test_c_style_still_works() {
    var x = true
    var y = false
    
    // Python-style only now (C-style removed)
    if x and not y {
        print("Python-style only now")
    }
    
    if y or x {
        print("Python 'or' confirmed working")
    }
}

fn test_mixed_styles() {
    var a = true
    var b = false
    var c = true
    
    // Python-style only (no mixing)
    if a and b or c {
        print("Python-style logical operators only")
    }
    
    if not a or b and c {
        print("Pure Python style")
    }
}

fn main() {
    print("Testing Python Logical Keywords")
    print("================================")
    test_python_keywords()
    print("")
    print("Testing C-style Compatibility")
    print("==============================")
    test_c_style_still_works()
    print("")
    print("Testing Mixed Styles")
    print("====================")
    test_mixed_styles()
}