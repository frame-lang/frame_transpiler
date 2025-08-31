// Simple test for system return semantics

system SimpleReturn {
    interface:
        getValue() : int = 42
        
    machine:
        $Start {
            getValue() {
                system.return = 100
                return
            }
        }
}

fn main() {
    var sr = SimpleReturn()
    var result = sr.getValue()
    print("getValue result: " + str(result))
    
    // Expect 100 from system.return override
    if result == 100 {
        print("SUCCESS: system.return override works!")
    } else {
        print("FAIL: Expected 100, got " + str(result))
    }
}