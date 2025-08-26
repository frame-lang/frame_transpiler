// Basic LEGB scope resolution test - simplified syntax

fn main() {
    print("=== Basic LEGB Test ===")
    
    var module_var = "MODULE"
    print(module_var)
    
    test_function()
    
    print("Back in main")
    print(module_var)
}

fn test_function() {
    print("=== Function Scope ===")
    
    var func_var = "FUNCTION"
    print(func_var)
    
    if true {
        var block_var = "BLOCK"
        print(block_var)
        print(func_var)
    }
    
    print(func_var)
}