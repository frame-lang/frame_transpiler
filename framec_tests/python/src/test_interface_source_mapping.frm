# Test file for interface method source mapping
# This verifies that interface method declarations get proper source map entries

system NoneChecker {
    interface:
        check(value)        # Line 6 - Should have source mapping
        getValue() : int    # Line 7 - Should have source mapping
        process()          # Line 8 - Should have source mapping
        
    machine:
        $Idle {
            check(value) {  # Line 12 - State handler
                if value == None {
                    print("Value is None")
                } else {
                    print("Value is not None")
                }
            }
            
            getValue() : int {  # Line 20
                system.return = 42
            }
            
            process() {        # Line 24
                print("Processing")
            }
        }
}

fn main() {
    var checker = NoneChecker()
    checker.check(None)        # Line 32 - Call site
    var val = checker.getValue()
    checker.process()
    print("Got value: " + str(val))
}