fn main() {
    print("Testing basic scope")
    var module_var = "module_value"
    print(module_var)
    
    test_function()
    print("Back in main")
}

fn test_function() {
    print("In test function")
    var func_var = "function_value"
    print(func_var)
}