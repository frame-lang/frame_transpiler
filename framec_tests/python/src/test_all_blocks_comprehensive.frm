// Comprehensive test of all Frame block types
system AllBlocksTest {
    operations:
        setup() {
            print("Operations: setup called")
        }
        
        process_data(value) {
            print("Operations: processing " + value)
            return "processed_" + value
        }

    interface:
        start_test()
        get_result() : string

    machine:
    $Start {
        $>() {
            print("Machine: Start state entered")
            self.setup()
            -> $Processing
        }
    }
    
    $Processing {
        $>() {
            print("Machine: Processing state entered")
            result = self.process_data("test_data")
            self.test_result = result
            print("Machine: result stored as " + self.test_result)
            -> $Complete
        }
    }
    
    $Complete {
        $>() {
            print("Machine: Complete state entered")
            print("Actions: calling complete_process")
            self.complete_process()
        }
        
        start_test() {
            print("Machine: start_test interface called")
            -> $Start
        }
        
        get_result() : string {
            print("Machine: get_result interface called")
            return = self.test_result
        }
    }

    actions:
        complete_process() {
            print("Actions: complete_process called")
            print("Actions: stored result is " + self.test_result)
        }

    domain:
        var test_result : string = ""
}