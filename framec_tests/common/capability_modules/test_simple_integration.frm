# Simple test of capability module integration

import Errors from "./errors_simple.frm"

system SimpleTest {
    interface:
        test()
    
    machine:
        $Start {
            test() {
                var result = Errors::createOk("test")
                print("Test completed")
            }
        }
}