@target python
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test class decorators pass-through for Python (v0.58)

from dataclasses import dataclass, field
from typing import ClassVar

# Test basic dataclass decorator
@dataclass
class Point {
    fn init(x, y) {
        self.x = x
        self.y = y
}

# Test multiple decorators
@dataclass
@dataclass(frozen=True)
class ImmutablePoint {
    fn init(x, y) {
        self.x = x
        self.y = y
}

# Test decorator with arguments
@dataclass(order=True, repr=False)
class OrderedPoint {
    fn init(x, y, z) {
        self.x = x
        self.y = y
        self.z = z
}

# Test custom decorator
@dataclass
@my_custom_decorator
class CustomPoint {
    class_counter = 0
    
    fn init(x, y) {
        self.x = x
        self.y = y
        CustomPoint.class_counter = CustomPoint.class_counter + 1
    
    fn distance_to_origin() {
        return (self.x ** 2 + self.y ** 2) ** 0.5
}

# Test function
fn test_decorators() {
    # Create instances
    p1 = Point(3, 4)
    p2 = ImmutablePoint(5, 12)
    p3 = OrderedPoint(1, 2, 3)
    p4 = CustomPoint(6, 8)
    
    # Test that they work
    print(f"Point: ({p1.x}, {p1.y})")
    print(f"ImmutablePoint: ({p2.x}, {p2.y})")
    print(f"OrderedPoint: ({p3.x}, {p3.y}, {p3.z})")
    print(f"CustomPoint: ({p4.x}, {p4.y})")
    print(f"CustomPoint instances: {CustomPoint.class_counter}")
    print(f"Distance: {p4.distance_to_origin()}")
}

# Define a dummy custom decorator for testing
fn my_custom_decorator(cls) {
    print(f"Decorating class {cls.__name__}")
    return cls
}

fn main() {
    # Run the test
    test_decorators()
}
