// Test basic Frame v0.20 features from basics.rst
// Note: Frame v0.20 allows only one function (main) per module

fn main() {
    print("=== Frame v0.20 Basics Test ===")
    
    // Test variable declarations
    var x = 0  
    var name = "Spock"
    var array = `[4][2]int{{10, 11}, {20, 21}, {30, 31}, {40, 41}}`
    
    print("Variable x: " + str(x))
    print("Name: " + name)
    print("Array initialized")
    
    print("Basics test completed")
}