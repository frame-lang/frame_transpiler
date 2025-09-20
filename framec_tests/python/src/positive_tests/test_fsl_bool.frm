# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test FSL bool() conversion operation

fn main() {
    # Test various bool conversions
    var b1 = bool(1)        # Should be True
    var b2 = bool(0)        # Should be False
    var b3 = bool(42)       # Should be True (non-zero)
    var b4 = bool(-1)       # Should be True (non-zero)
    
    var b5 = bool("hello")  # Should be True (non-empty string)
    var b6 = bool("")       # Should be False (empty string)
    
    var b7 = bool(3.14)     # Should be True (non-zero float)
    var b8 = bool(0.0)      # Should be False (zero float)
    
    # Print results
    print("bool(1):", b1)
    print("bool(0):", b2)
    print("bool(42):", b3)
    print("bool(-1):", b4)
    print("bool('hello'):", b5)
    print("bool(''):", b6)
    print("bool(3.14):", b7)
    print("bool(0.0):", b8)
    
    # Test in conditionals
    if bool(1) {
        print("bool(1) is truthy")
    }
    
    if not bool(0) {
        print("bool(0) is falsy")
    }
    
    # Test with variables
    var x = 100
    var y = 0
    print("bool(x=100):", bool(x))
    print("bool(y=0):", bool(y))
    
    return
}