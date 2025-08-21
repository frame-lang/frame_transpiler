// Comprehensive test of Frame v0.20 features:
// - return statements as regular statements
// - return = expr for interface return assignment
// - if/elif/else chains with returns
// - Mixed usage in event handlers and actions
fn main() {
    print("=== Frame v0.20 Comprehensive Feature Test ===")
    
    var processor = AdvancedProcessor()
    
    // Test various scenarios
    var results = [
        processor.processData(""),
        processor.processData("test"),
        processor.processData("ERROR"),
        processor.processData("Hello World"),
        processor.processData("very long text that exceeds the normal length limits")
    ]
    
    print("\n=== Results ===")
    for result in results {
        print("Result: " + result)
    }
    
    print("\n=== State Management Test ===")
    processor.reset()
    processor.configure("debug")
    processor.processData("debug test")
}

system AdvancedProcessor {
    interface:
        processData(input: str): str
        reset()
        configure(mode: str)
        
    machine:
        $Idle {
            $>() {
                print("Processor ready in Idle state")
                return
            }
            
            processData(input: str): str {
                // Early validation with return = 
                if input == "" {
                    return = "error: empty input"
                    return
                }
                
                // Transition to processing state with data
                -> $Processing(input)
                return
            }
            
            configure(mode: str) {
                if mode == "debug" {
                    print("Enabling debug mode")
                    -> $Debug
                } elif mode == "fast" {
                    print("Enabling fast mode") 
                    -> $FastProcessing
                } else {
                    print("Unknown mode: " + mode)
                }
                return
            }
            
            reset() {
                print("Already in idle state")
                return
            }
        }
        
        $Processing(data: str) {
            $>() {
                print("Processing: " + data)
                
                // Complex processing logic with if/elif/else
                var result = processText(data)
                
                if result == "error" {
                    return = "processing failed"
                    -> $Idle
                    return
                } elif result == "warning" {
                    return = "processed with warnings"
                    -> $Idle  
                    return
                } else {
                    return = "success: " + result
                    -> $Idle
                    return
                }
            }
            
            reset() {
                print("Resetting from processing state")
                -> $Idle
                return
            }
        }
        
        $Debug {
            $>() {
                print("Debug mode active")
                return
            }
            
            processData(input: str): str {
                print("DEBUG: Processing '" + input + "'")
                
                if input == "debug test" {
                    return = "debug: test successful"
                    return
                }
                
                return = "debug: " + input
                return
            }
            
            reset() {
                print("Exiting debug mode")
                -> $Idle
                return
            }
        }
        
        $FastProcessing {
            processData(input: str): str {
                return = "fast: " + input
                -> $Idle
                return
            }
            
            reset() {
                -> $Idle
                return
            }
        }
        
    actions:
        processText(text: str): str {
            // Action with complex return logic
            if text == "ERROR" {
                return "error"
            }
            
            if len(text) > 50 {
                return "warning"  
            }
            
            if text == "test" {
                return "validated"
            }
            
            return "processed"
        }
        
        len(s: str): int {
            var count = 0
            for c in s {
                count = count + 1
            }
            return count
        }
}