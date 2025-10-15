# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test class decorators pass-through for Python (v0.58)
# Simplified version without complex decorator arguments

from dataclasses import dataclass

# Test basic dataclass decorator
@dataclass
class Point {
    fn init(x, y) {
        self.x = x
        self.y = y
    }
}

# Test function
fn test_decorators() {
    # Create instance
    var p1 = Point(3, 4)
    
    # Test that it works
    print("Point: (" + str(p1.x) + ", " + str(p1.y) + ")")
}

fn main() {
    # Run the test
    test_decorators()
}