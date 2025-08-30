system TestStaticError {
    operations:
        @staticmethod
        bad_static(): String {
            // This should cause an error - can't use self in static method
            return self.name
        }
        
        good_instance(): String {
            // This is OK - instance method can use self
            return self.name
        }
    
    domain:
        var name = "Test"
}

fn main() {
    print("This file should fail to parse")
}