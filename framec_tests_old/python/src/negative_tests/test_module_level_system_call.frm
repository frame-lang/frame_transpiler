# NEGATIVE TEST - Cannot instantiate systems at module level
# This test should fail with: "Module-level function calls are not allowed"

system TestSystem {
    interface:
        doSomething()
    
    machine:
        $Start {
            doSomething() {
                print("Doing something")
                return
            }
        }
    }
}

fn main() {
    var sys = TestSystem()
    sys.doSomething()
}

# ERROR: Cannot instantiate system at module scope
TestSystem()