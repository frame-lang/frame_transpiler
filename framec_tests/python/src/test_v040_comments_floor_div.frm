# Frame v0.40 Test - Python-style comments and floor division
# This file demonstrates the new comment syntax and floor division operator

fn test_floor_division() {
    # Test basic floor division
    var a = 10
    var b = 3
    var result = a // b  # Should be 3
    print("10 // 3 = " + str(result))
    
    # Test floor division with negative numbers
    var c = -10
    var d = 3
    var result2 = c // d  # Should be -4 (Python floor division behavior)
    print("-10 // 3 = " + str(result2))
    
    # Test floor division vs regular division
    var regular = 10 / 3   # Regular division: 3.333...
    var floor = 10 // 3    # Floor division: 3
    print("Regular: " + str(regular) + ", Floor: " + str(floor))
    
    # Compound assignment with floor division
    var x = 17
    x //= 5  # x = x // 5, should be 3
    print("17 //= 5 = " + str(x))
}

fn test_comment_styles() {
    # This is a Python-style single-line comment
    
    {-- 
    This is a Frame documentation comment.
    It can span multiple lines and is typically
    used for documentation purposes.
    --}
    
    var value = 42  # Inline comment after code
    
    # Multiple consecutive comments
    # Line 1 of comments
    # Line 2 of comments
    # Line 3 of comments
    
    print("Comments working correctly!")
}

fn main() {
    print("=== Frame v0.40 Features Test ===")
    test_floor_division()
    test_comment_styles()
}