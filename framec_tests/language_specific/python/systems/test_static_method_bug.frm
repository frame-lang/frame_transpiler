@target python

# Test for Bug #10: Static method call incorrectly generates with "self"
fn main() {
    print("Testing static method bug #10")
    service = TestService()
    service.initialize()
    return
}

system TestService {
    operations:
        @staticmethod
        getDefaultConfig() {
            return {"timeout": 30, "retries": 3}
        }
    
    interface:
        initialize()
    
    machine:
        $Ready {
            initialize() {
                # This should generate: self.config = TestService.getDefaultConfig()
                # But incorrectly generates: self.config = TestService.self.getDefaultConfig()
                self.config = TestService.getDefaultConfig()
                print("Config initialized: " + str(self.config))
                return
            }
        }
    
    domain:
        config = None
}
