# Test basic class features in Frame v0.45

class Point {
    # Static/class variable
    var instance_count = 0
    
    # Constructor (implicit init method)
    fn init(x, y) {
        self.x = x
        self.y = y
        Point.instance_count = Point.instance_count + 1
    }
    
    # Instance method
    fn distance_to(other) {
        var dx = self.x - other.x
        var dy = self.y - other.y
        var sum_of_squares = (dx * dx) + (dy * dy)
        return sum_of_squares ** 0.5
    }
    
    # Static method
    @staticmethod
    fn origin() {
        return Point(0.0, 0.0)
    }
    
    fn __str__() {
        return "Point(" + str(self.x) + ", " + str(self.y) + ")"
    }
}

class Circle {
    fn init(center, radius) {
        self.center = center
        self.radius = radius
    }
    
    fn area() {
        return 3.14159 * self.radius * self.radius
    }
    
    fn contains(point) {
        var distance = self.center.distance_to(point)
        return distance <= self.radius
    }
}

# Test the classes
fn main() {
    # Create points
    var p1 = Point(3.0, 4.0)
    var p2 = Point(6.0, 8.0)
    var origin = Point.origin()
    
    # Test instance methods
    var dist = p1.distance_to(p2)
    print("Distance from p1 to p2: " + str(dist))
    
    # Test static variable
    print("Total points created: " + str(Point.instance_count))
    
    # Test Circle class
    var circle = Circle(origin, 5.0)
    print("Circle area: " + str(circle.area()))
    
    if circle.contains(p1) {
        print("p1 is inside the circle")
    } else {
        print("p1 is outside the circle")
    }
    
    if circle.contains(p2) {
        print("p2 is inside the circle")  
    } else {
        print("p2 is outside the circle")
    }
}