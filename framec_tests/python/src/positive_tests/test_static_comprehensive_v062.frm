# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Comprehensive test for static method calls in Frame v0.62
# Tests all patterns of @staticmethod usage on both classes and systems

fn main() {
    # Test static calls from standalone function
    var result1 = MathUtils.add(10, 20)
    print("From function: " + str(result1))
    
    # Test static calls on class
    var origin = Point.origin()
    print("Origin: " + origin.toString())
    
    # Test instance calling static (allowed but not idiomatic)
    var p = Point(3, 4)
    var origin2 = p.origin()  # Works but should use Point.origin()
    
    # Test system with static operations
    var calc = Calculator()
    calc.performCalculations()
    
    return
}

# Class with static methods
class Point {
    fn init(x, y) {
        self.x = x
        self.y = y
    }
    
    fn toString() {
        return "(" + str(self.x) + ", " + str(self.y) + ")"
    }
    
    @staticmethod
    fn origin() {
        return Point(0, 0)
    }
    
    @staticmethod
    fn distance(p1, p2) {
        var dx = p2.x - p1.x
        var dy = p2.y - p1.y
        return ((dx * dx) + (dy * dy)) ** 0.5
    }
}

# System with static operations
system MathUtils {
    operations:
        @staticmethod
        add(a, b) {
            return a + b
        }
        
        @staticmethod
        multiply(a, b) {
            return a * b
        }
        
        # Non-static operation for comparison
        instanceMethod() {
            print("This requires an instance")
            return
        }
}

# System that calls static methods
system Calculator {
    interface:
        performCalculations()
    
    machine:
        $Idle {
            performCalculations() {
                # Static calls from within system
                var sum = MathUtils.add(5, 7)
                print("5 + 7 = " + str(sum))
                
                var product = MathUtils.multiply(3, 4)
                print("3 * 4 = " + str(product))
                
                # Test static class method call
                var p1 = Point(0, 0)
                var p2 = Point(3, 4)
                var dist = Point.distance(p1, p2)
                print("Distance: " + str(dist))
                
                # This would fail - can't call instance method statically
                # MathUtils.instanceMethod()  # ERROR
                
                return
            }
        }
}

# Advanced patterns
system AdvancedStatic {
    operations:
        @staticmethod
        factory(type) {
            if type == "point" {
                return Point.origin()
            }
            return None
        }
        
        @staticmethod
        processWithStatic(x) {
            # Static calling another static in same system
            return AdvancedStatic.helper(x * 2)
        }
        
        @staticmethod
        helper(value) {
            return value + 100
        }
}

main()