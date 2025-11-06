@target python
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test import edge cases and potential conflicts - v0.34
# Expected: User functions can coexist with imports when no FSL imported

# Test 1: User-defined function with same name as FSL (no FSL import)
fn str(x) {
    # Note: Cannot call Python's str() here without backticks
    return "[custom:value]"
}

fn int(x) {
    return 1000  # Always returns 1000
}

fn test_user_defined_shadows() {
    print("=== Testing User-Defined Functions (No FSL) ===")
    
    num = 42
    custom_str = str(num)      # Calls our str() function
    custom_int = int("999")    # Calls our int() function
    
    print("Custom str(42): " + custom_str)        # Expected: [custom:value]
    print("Custom int('999'): 1000")  # Expected: 1000
}

# Test 2: Multiple import styles together
import os
from os import path
from os.path import exists as file_exists

fn test_multiple_import_styles() {
    print("=== Testing Multiple Import Styles ===")
    
    # All three import styles for os module
    cwd = # Removed backticks - os.getcwd()
    joined = # Removed backticks - path.join('a', 'b')
    exists = # Removed backticks - file_exists('/tmp')
    
    print("os.getcwd(): " + cwd)
    print("path.join(): " + joined)
    print("file_exists(): " + # Removed backticks - str(exists))
}

fn main() {
    print("=== Import Edge Cases Test ===")
    test_user_defined_shadows()
    test_multiple_import_styles()
    print("=== Test Complete ===")
}
