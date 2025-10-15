# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test system return semantics and default values

system SystemReturnTest {
    interface:
        # Interface with default return value
        getValue() : int = 42
        check() : bool = false
        process()
        
    machine:
        $Start {
            # Event handler overrides default system.return
            getValue() : int = 100 {
                callAction()  # Action might modify system.return
                if true {
                    system.return = 200  # Explicit override
                }
                return
            }
            
            # Handler uses interface default
            check() {
                # Implicit system.return = false from interface
                return
            }
            
            # Handler sets system.return explicitly
            process() {
                callAction()
                return
            }
        }
        
    actions:
        # Action that sets system.return
        callAction() : string {
            system.return = 100  # Set interface return
            return "action_done"  # Return to handler
        }
}

fn main() {
    var tester = SystemReturnTest()
    var result = tester.getValue()
    print("getValue result: " + str(result))
    
    var check_result = tester.check()
    print("check result: " + str(check_result))
    
    tester.process()
    print("process completed")
}