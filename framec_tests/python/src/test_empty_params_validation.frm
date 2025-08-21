fn main() {
    var service = ValidationService()
}

system ValidationService {

    operations:
        test_empty_params() {
            print("Operation with empty params called")
            self.interface_method()
        }

    interface:
        interface_method()

    machine:

    $Start {
        $>() {
            print("Enter event with empty params")
            self.test_empty_params()
            -> $Done
        }
    }

    $Done {
        interface_method() {
            print("Interface method with empty params called")
            -> $Finished
        }
    }

    $Finished {
        $>() {
            print("Test completed successfully!")
        }
    }
}