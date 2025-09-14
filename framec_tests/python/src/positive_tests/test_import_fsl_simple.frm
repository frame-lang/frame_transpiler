# Simple FSL import test

fn test_fsl() {
    var number = 42
    var text = "123"
    
    var str_result = str(number)
    var int_result = int(text)
    
    print("str(42): " + str_result)
    print("int('123'): " + str(int_result))
}

fn main() {
    print("=== Simple FSL Import Test ===")
    test_fsl()
    print("=== Test Complete ===")
}