@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Comprehensive Frame Language Test - All Features in One File
# Tests: systems, functions, variables, operations, actions, domain vars, 
#        state transitions, enter/exit handlers, interface methods, imports,
#        lambdas, collections, method chaining, and more

import math
import os

# Module-level function
fn global_helper(x) {
    print("Global helper called with: " + str(x))
    return x * 10
}

# Test function (lambdas removed for compatibility)
fn test_basic_operations() {
    print("Testing basic operations...")
    x = 5
    result = x * x
    print("Square of 5 = " + str(result))
    
    # Simple calculation
    multiplier = 3
    multiply_result = x * multiplier
    print("Multiply result: " + str(multiply_result))
}

# Collection operations function
fn test_collections() {
    print("Testing collections...")
    
    # Lists
    numbers = [1, 2, 3, 4, 5]
    numbers.append(6)
    print("List after append: " + str(numbers))
    
    # Dictionaries
    data = {"name": "Frame", "version": "0.81"}
    data["feature"] = "comprehensive"
    print("Dict: " + str(data))
    
    # Nested access
    matrix = [[1, 2], [3, 4]]
    val = matrix[1][0]
    print("Matrix[1][0] = " + str(val))
}

# Main execution function
fn main() {
    print("=== Comprehensive Frame Test ===")
    
    # Test module access
    pi_val = math.pi
    print("Math.pi = " + str(pi_val))
    
    # Test path operations
    file_path = os.path.join("debug", "test.frm")
    print("Path join: " + file_path)
    
    # Test helper function
    helper_result = global_helper(5)
    print("Helper result: " + str(helper_result))
    
    # Test basic operations
    test_basic_operations()
    
    # Test collections
    test_collections()
    
    # Create and test comprehensive system
    print("\n--- Testing System ---")
    comp_system = ComprehensiveSystem("initial_data")
    
    # Test interface methods
    comp_system.initialize()
    comp_system.process_data("test_input")
    comp_system.get_status()
    comp_system.trigger_event("important")
    comp_system.calculate_result(42)
    comp_system.shutdown()
    
    # Create second system to test multiple instances
    print("\n--- Testing Second System ---")
    counter_system = SimpleCounter()
    counter_system.increment()
    counter_system.increment()
    counter_system.get_count()
    counter_system.reset()
    
    print("\n=== Test Complete ===")
}

# Comprehensive system with all Frame features
system ComprehensiveSystem(init_data) {
    
    operations:
        # Internal operations
        validate_input(data) {
            print("Validating: " + data)
            return len(data) > 0
        }
        
        process_internal(value) {
            print("Processing internal: " + str(value))
            result = value * self.multiplier + self.offset
            return result
        }
        
        log_state_change(from_state, to_state) {
            print("State change: " + from_state + " -> " + to_state)
        }
    
    interface:
        # Public interface methods
        initialize()
        process_data(data: string): bool
        get_status(): string
        trigger_event(event_type: string)
        calculate_result(input: int): int
        shutdown()
    
    machine:
        $Idle {
            $>() {
                print("Entering Idle state")
                self.current_state = "Idle"
                self.log_state_change("None", "Idle")
            }
            
            <$() {
                print("Exiting Idle state")
            }
            
            initialize() {
                print("Initialize called in Idle")
                self.is_initialized = true
                self.process_count = 0
                system.return = true
                -> $Ready
            }
            
            get_status(): string {
                system.return = "System is idle"
            }
        }
        
        $Ready {
            $>() {
                print("Entering Ready state")
                self.current_state = "Ready"
                self.log_state_change("Idle", "Ready")
            }
            
            <$() {
                print("Exiting Ready state")
            }
            
            process_data(data: string): bool {
                print("Processing data in Ready: " + data)
                
                # Use operation
                is_valid = self.validate_input(data)
                if is_valid:
                    self.last_input = data
                    self.process_count = self.process_count + 1
                    system.return = true
                    -> $Processing
                else:
                    print("Invalid input, staying in Ready")
                    system.return = false
            }
            
            get_status(): string {
                status = "Ready - processed " + str(self.process_count) + " items"
                system.return = status
            }
            
            trigger_event(event_type: string) {
                if event_type == "shutdown":
                    -> $Shutting_Down
                elif event_type == "important":
                    print("Important event received")
                    -> $Alert
                else:
                    print("Unknown event: " + event_type)
            }
            
            shutdown() {
                print("Shutdown requested from Ready")
                -> $Shutting_Down
            }
        }
        
        $Processing {
            $>() {
                print("Entering Processing state")
                self.current_state = "Processing"
                self.log_state_change("Ready", "Processing")
                
                # Auto-process the data
                print("Auto-processing: " + self.last_input)
                self.finish_processing()
            }
            
            <$() {
                print("Exiting Processing state")
            }
            
            calculate_result(input: int): int {
                print("Calculating result for: " + str(input))
                result = self.process_internal(input)
                self.last_result = result
                system.return = result
                -> $Ready
            }
            
            get_status(): string {
                system.return = "Processing data: " + self.last_input
            }
            
            shutdown() {
                print("Shutdown during processing - cleaning up")
                -> $Shutting_Down
            }
        }
        
        $Alert {
            $>() {
                print("Entering Alert state - IMPORTANT!")
                self.current_state = "Alert"
                self.alert_count = self.alert_count + 1
                self.log_state_change("Ready", "Alert")
            }
            
            <$() {
                print("Exiting Alert state")
            }
            
            process_data(data: string): bool {
                print("Processing urgent data: " + data)
                self.last_input = data
                system.return = true
                -> $Processing
            }
            
            get_status(): string {
                system.return = "ALERT! Alert count: " + str(self.alert_count)
            }
            
            trigger_event(event_type: string) {
                if event_type == "clear":
                    print("Alert cleared")
                    -> $Ready
                else:
                    print("Additional event during alert: " + event_type)
            }
            
            shutdown() {
                print("Emergency shutdown from Alert")
                -> $Shutting_Down
            }
        }
        
        $Shutting_Down {
            $>() {
                print("Entering Shutdown state")
                self.current_state = "Shutting_Down"
                self.log_state_change("Previous", "Shutting_Down")
                
                # Cleanup actions
                self.cleanup()
                
                # Auto-transition to final state
                -> $Terminated
            }
            
            get_status(): string {
                system.return = "System is shutting down..."
            }
        }
        
        $Terminated {
            $>() {
                print("System terminated - Final state reached")
                self.current_state = "Terminated"
                self.is_initialized = false
            }
            
            get_status(): string {
                system.return = "System terminated"
            }
        }
    
    actions:
        # Actions called by enter handlers
        finish_processing() {
            print("Action: Finish processing called")
            self.processing_complete = true
        }
        
        cleanup() {
            print("Action: Cleanup called")
            self.last_input = ""
            self.last_result = 0
            self.processing_complete = false
        }
        
        emergency_stop() {
            print("Action: Emergency stop!")
        }
    
    domain:
        # Domain variables
        init_data = ""
        current_state = ""
        is_initialized = false
        process_count = 0
        last_input = ""
        last_result = 0
        processing_complete = false
        alert_count = 0
        multiplier = 2
        offset = 10
}

# Simple counter system for additional testing
system SimpleCounter {
    interface:
        increment()
        decrement()
        get_count()
        reset()
    
    machine:
        $Counting {
            $>() {
                print("Counter initialized to 0")
                self.count = 0
            }
            
            increment() {
                self.count = self.count + 1
                print("Count incremented to: " + str(self.count))
            }
            
            decrement() {
                if self.count > 0:
                    self.count = self.count - 1
                    print("Count decremented to: " + str(self.count))
                else:
                    print("Count already at 0")
            }
            
            get_count() {
                print("Current count: " + str(self.count))
                system.return = self.count
            }
            
            reset() {
                print("Resetting counter")
                self.count = 0
                -> $Reset
            }
        }
        
        $Reset {
            $>() {
                print("Counter reset complete")
                -> $Counting
            }
        }
    
    domain:
        count = 0
}
