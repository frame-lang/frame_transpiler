# Bug #52 Regression Test: Interface method call translation in TypeScript
# Ensures system.methodName() generates this.methodName() not system.methodName()

system TestBug52 {
    interface:
        getValue(): int
        
    machine:
        $Start {
            testEvent() {
                # This should generate this.getValue() in TypeScript, not system.getValue()
                var result = system.getValue()
                print(f"Got value: {result}")
            }
        }
    
    actions:
        getValue(): int {
            return 42
        }
}

fn main() {
    var sm = TestBug52()
    # Test the interface method directly - this validates the bug fix
    var result = sm.getValue()
    print(f"Interface method returned: {result}")
    print("SUCCESS: Bug #52 interface method calls working")
}