# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test module scope variables

import math

# Module-level variables (Frame doesn't support const)
var PI = 3.14159
var VERSION = "1.0.0"

# Module-level variables
var counter = 0
var config = None
var initialized = false

# Module-level system instantiation
system Logger {
    interface:
        log(msg: str)
    
    machine:
        $Start {
            log(msg: str) {
                print("[LOG] " + msg)
                return
            }
        }
}

# Module-level function
fn setup() {
    counter = 1
    config = None
    initialized = true
}

# Test function
fn test_module_vars() {
    print("PI = " + str(PI))
    print("VERSION = " + VERSION)
    print("counter = " + str(counter))
    print("initialized = " + str(initialized))
}

# Main entry point
fn main() {
    setup()  # Initialize module
    var logger = Logger()  # Instantiate system
    logger.log("Module initialized")
    
    test_module_vars()
    logger.log("Main function called")
    counter = counter + 1
    print("Final counter = " + str(counter))
}
