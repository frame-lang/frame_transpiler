# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test class decorators for property decorators (v0.58)

# Simple class with property decorator usage demonstration
class Temperature {
    fn init(celsius) {
        self._celsius = celsius
    }
    
    @property
    fn fahrenheit() {
        return self._celsius * 9.0 / 5.0 + 32.0
    }
    
    @property  
    fn celsius() {
        return self._celsius
    }
}

# Test function
fn test_property() {
    var temp = Temperature(25.0)
    print("Celsius: " + str(temp.celsius))
    print("Fahrenheit: " + str(temp.fahrenheit))
}

fn main() {
    # Run test
    test_property()
}