// POSITIVE TEST - What IS allowed at module level
// This should compile successfully

// Module-level imports are allowed
import math

// Module-level variables are allowed
var global_counter = 0
var global_message = "Hello"

// Module-level enums are allowed
enum Status {
    Active
    Inactive
    Pending
}

// Module-level classes are allowed
class DataHolder {
    fn init(value) {
        self.value = value
    }
    
    fn getValue() {
        return self.value
    }
}

// Functions declarations are allowed (just not calls)
fn helper(x) {
    return x * 2
}

// Systems declarations are allowed
system SimpleSystem {
    interface:
        process()
    
    machine:
        $Start {
            process() {
                print("Processing")
                return
            }
        }
    }
}

// main() function is special - automatically called
fn main() {
    print("Module-level declarations test")
    
    // All calls must be inside functions
    var result = helper(21)
    print("Helper result: " + str(result))
    
    var holder = DataHolder(42)
    print("Holder value: " + str(holder.getValue()))
    
    var sys = SimpleSystem()
    sys.process()
    
    print("Status: " + str(Status.Active))
    print("Global counter: " + str(global_counter))
    print("Global message: " + global_message)
}