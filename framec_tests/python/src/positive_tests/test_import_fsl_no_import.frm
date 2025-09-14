# Test FSL operations WITHOUT import - should use external functions
# This test verifies that without FSL import, operations are treated as external

# NO FSL import - operations should be treated as external Python functions

fn main() {
    print("=== Testing FSL Operations Without Import ===")
    
    # These will call external Python str(), int(), float() functions
    # NOT the FSL versions
    var num = 42
    var text = "123"
    
    # These use Python's built-in functions directly (via backticks in Frame normally)
    # Since we don't import FSL, these are external calls
    var str_result = str(num)      # External Python str()
    var int_result = int(text)     # External Python int()
    
    print("External str(42): " + str_result)
    print("External int('123'): " + str(int_result))
    
    print("=== Test Complete - Used External Functions ===")
}