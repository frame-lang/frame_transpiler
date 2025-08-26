// Basic scope test - Fixed syntax

fn main() {
    print("=== Basic Scope Test ===")
    
    var module_var = "MODULE"
    print(module_var)
    
    test_function()
    
    print("Module var after function:")
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