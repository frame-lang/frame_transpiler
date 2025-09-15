# Test class decorators with arguments (v0.58)

from dataclasses import dataclass

# Test decorator with arguments  
@dataclass(frozen=True)
class ImmutablePoint {
    fn init(x, y) {
        self.x = x
        self.y = y
    }
}

# Test function
fn test_decorators() {
    # Create instance
    var p = ImmutablePoint(5, 12)
    
    # Test that it works
    print("ImmutablePoint: (" + str(p.x) + ", " + str(p.y) + ")")
}

# Run the test
test_decorators()