// Test: return = expr syntax in action methods
fn main() {
    var processor = DataProcessor()
    
    // Test action return vs interface return
    var result = processor.process("test")
    print("Interface return: " + result)
}

system DataProcessor {
    interface:
        process(input: str): str
        
    machine:
        $Active {
            process(input: str): str {
                // Call action and use its return value
                var actionResult = validateAndProcess(input)
                print("Action returned: " + actionResult)
                
                // Interface return was set by action
                return
            }
        }
        
    actions:
        validateAndProcess(data: str): str {
            // Validate input
            if data == "" {
                return = "error: empty input"  // Set interface return
                return "validation_failed"     // Return to event handler
            }
            
            if data == "test" {
                return = "success: processed test data"  // Set interface return
                return "validation_passed"               // Return to event handler
            }
            
            // Default case
            return = "processed: " + data  // Set interface return
            return "processed_default"     // Return to event handler
        }
}