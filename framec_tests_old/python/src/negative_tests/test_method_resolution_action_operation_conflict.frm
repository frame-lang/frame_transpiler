# Test Method Call Resolution Policy - Action/Operation Conflict
# Should fail: action and operation with same name creates ambiguity

fn main() {
    var sys = ConflictSystem()
    sys.test_method()
}

system ConflictSystem {
    operations:
        conflicting_method() {
            print("Operation version")
        }
        
    interface:
        test_method()
        
    machine:
        $Ready {
            test_method() {
                # This should fail - ambiguous call due to both action and operation existing
                self.conflicting_method()
                return
            }
        }
        
    actions:
        conflicting_method() {
            print("Action version")
        }
}