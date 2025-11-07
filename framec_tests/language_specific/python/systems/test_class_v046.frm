# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Frame v0.46 Class Enhancements Test
# Test inheritance, properties, class methods, and special methods

# Animal base class
class Animal {
    var species:str = "Unknown"
    var count:int = 0  # Class variable to track instances
    
    fn init(name) {
        self.name = name
        Animal.count = Animal.count + 1
    }
    
    fn speak() {
        return "Some sound from " + self.name
    }
    
    fn move() {
        return self.name + " is moving"
    }
    
    @classmethod
    fn get_count(cls) {
        return cls.count
    }
    
    @staticmethod
    fn info() {
        return "Animals are living organisms"
    }
}

# Dog class extending Animal
class Dog(Animal) {
    fn init(name, breed) {
        super.init(name)  # Call parent constructor
        self.breed = breed
        self.species = "Canis familiaris"
    }
    
    fn speak() {
        # Override parent method
        return "Woof! My name is " + self.name
    }
    
    fn fetch() {
        return self.name + " is fetching!"
    }
}

# Temperature class with properties
class Temperature {
    fn init(celsius) {
        self._celsius = celsius
    }
    
    @property
    fn celsius() {
        return self._celsius
    }
    
    @celsius.setter
    fn celsius(value) {
        if value < -273.15 {
            print("Error: Temperature below absolute zero")
            return
        }
        self._celsius = value
    }
    
    @property
    fn fahrenheit() {
        return self._celsius * 9.0 / 5.0 + 32.0
    }
    
    @fahrenheit.setter
    fn fahrenheit(value) {
        self.celsius = (value - 32.0) * 5.0 / 9.0
    }
}

# Vector class with special methods
class Vector {
    fn __init__(x, y) {
        self.x = x
        self.y = y
    }
    
    fn __str__() {
        return "Vector(" + str(self.x) + ", " + str(self.y) + ")"
    }
    
    fn __add__(other) {
        return Vector(self.x + other.x, self.y + other.y)
    }
    
    fn __eq__(other) {
        return self.x == other.x and self.y == other.y
    }
    
    fn __len__() {
        # Return magnitude as integer
        return int((self.x * self.x + self.y * self.y) ** 0.5)
    }
}

# Factory pattern with class methods
class User {
    var user_count:int = 0
    
    fn init(name, email) {
        self.name = name
        self.email = email
        User.user_count = User.user_count + 1
    }
    
    @classmethod
    fn from_string(cls, user_string) {
        # Parse "name:email" format
        parts = user_string.split(":")
        if len(parts) == 2:
            return cls(parts[0], parts[1])
        return None
    }
    
    @classmethod
    fn get_user_count(cls) {
        return cls.user_count
    }
    
    @staticmethod
    fn validate_email(email) {
        return "@" in email and "." in email
    }
    
    fn __str__() {
        return "User(" + self.name + ", " + self.email + ")"
    }
}

# Main test function
fn main() {
    print("=== Testing v0.46 Class Features ===")
    
    # Test inheritance
    print("\n--- Testing Inheritance ---")
    dog = Dog("Buddy", "Golden Retriever")
    print("Dog name: " + dog.name)
    print("Dog breed: " + dog.breed)
    print("Dog species: " + dog.species)
    print("Dog speaks: " + dog.speak())
    print("Dog moves: " + dog.move())  # Inherited method
    print("Dog fetches: " + dog.fetch())
    
    # Test class methods and static methods
    print("\n--- Testing Class/Static Methods ---")
    print("Animal count: " + str(Animal.get_count()))
    print("Animal info: " + Animal.info())
    
    # Test properties
    print("\n--- Testing Properties ---")
    temp = Temperature(25.0)
    print("Temperature in Celsius: " + str(temp.celsius))
    print("Temperature in Fahrenheit: " + str(temp.fahrenheit))
    
    temp.celsius = 0.0
    print("After setting to 0°C:")
    print("Celsius: " + str(temp.celsius))
    print("Fahrenheit: " + str(temp.fahrenheit))
    
    temp.fahrenheit = 100.0
    print("After setting to 100°F:")
    print("Celsius: " + str(temp.celsius))
    print("Fahrenheit: " + str(temp.fahrenheit))
    
    # Test special methods
    print("\n--- Testing Special Methods ---")
    v1 = Vector(3, 4)
    v2 = Vector(1, 2)
    print("v1: " + str(v1))
    print("v2: " + str(v2))
    
    v3 = v1 + v2
    print("v1 + v2: " + str(v3))
    
    print("v1 == v2: " + str(v1 == v2))
    print("v1 == Vector(3, 4): " + str(v1 == Vector(3, 4)))
    print("Length of v1: " + str(len(v1)))
    
    # Test factory pattern with class methods
    print("\n--- Testing Factory Pattern ---")
    user1 = User("Alice", "alice@example.com")
    print("User 1: " + str(user1))
    
    user2 = User.from_string("Bob:bob@example.com")
    if user2 is not None {
        print("User 2: " + str(user2))
    }
    
    print("Total users: " + str(User.get_user_count()))
    print("Valid email test: " + str(User.validate_email("test@example.com")))
    print("Invalid email test: " + str(User.validate_email("invalid-email")))
    
    print("\n=== All v0.46 Tests Complete ===")
}
