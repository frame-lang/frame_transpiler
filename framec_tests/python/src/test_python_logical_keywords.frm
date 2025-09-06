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
    
    // Test that C-style still works (for compatibility)
    if x && !y {
        print("C-style && and ! still work")
    }
    
    if y || x {
        print("C-style || still works")
    }
}

fn test_mixed_styles() {
    var a = true
    var b = false
    var c = true
    
    // Mix Python and C-style (not recommended but should work)
    if a and b || c {
        print("Mixed styles work (but not recommended)")
    }
    
    if !a or b and c {
        print("More mixed styles")
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