# Test Method Call Resolution Policy - Valid Cases
# Tests all valid method call patterns with proper resolution

fn main() {
    sys = MethodResolutionSystem()
    sys.test_all_valid_calls()
    print("All valid method call patterns work correctly")
}

system MethodResolutionSystem {
    operations:
        operation_method_a(): string {
            print("Operation A called")
            return "operation_result"
        }
        
        operation_method_b(x: int, y: int): int {
            print("Operation B called")
            return x + y
        }
        
        operation_only_method(): bool {
            print("Operation-only method called")
            return true
        }
        
        @staticmethod
        static_operation(a: int, b: int): int {
            return a * b
        }
        
    interface:
        test_all_valid_calls()
        interface_method_a(): string
        interface_method_b(x: int): int
        
    machine:
        $Ready {
            test_all_valid_calls() {
                print("=== Testing Valid Method Call Patterns ===")
                
                # 1. Valid self.action calls
                print("1. Action calls:")
                self.action_method_a()
                self.action_method_b(42)
                self.action_only_method()
                
                # 2. Valid self.operation calls  
                print("2. Operation calls:")
                result_a = self.operation_method_a()
                print("Got: " + result_a)
                result_b = self.operation_method_b(10, 20)
                print("Got: " + str(result_b))
                result_c = self.operation_only_method()
                print("Got: " + str(result_c))
                
                # 3. Valid system.interface calls
                print("3. Interface calls:")
                interface_result_a = system.interface_method_a()
                print("Interface result: " + interface_result_a)
                interface_result_b = system.interface_method_b(100)
                print("Interface result: " + str(interface_result_b))
                
                # 4. Valid static operation calls
                print("4. Static operation calls:")
                static_result = MethodResolutionSystem.static_operation(5, 6)
                print("Static result: " + str(static_result))
                
                print("=== All valid calls completed successfully ===")
                return
            }
            
            interface_method_a(): string {
                # Interface method can call actions and operations
                self.action_method_a()
                op_result = self.operation_method_a()
                system.return = "interface_" + op_result
            }
            
            interface_method_b(x: int): int {
                # Interface method using actions and operations
                self.action_method_b(x)
                calc_result = self.operation_method_b(x, x)
                system.return = calc_result * 2
            }
        }
        
    actions:
        action_method_a() {
            print("Action A called")
        }
        
        action_method_b(value: int) {
            print("Action B called with: " + str(value))
            self.counter = value
        }
        
        action_only_method() {
            print("Action-only method called")
        }
        
    domain:
        counter: int = 0
}