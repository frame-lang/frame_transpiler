// Minimal scope test to identify parser issues

fn main() {
    print("Testing basic scope")
    var x = "module"
    print(x)
    
    test_function()
}

fn test_function() {
    print("In function")
    var y = "function"
    print(y)
}