# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system TestSystem {
    machine:
        $StateA {
            var sysA = None
            
            $>() {
                print("Enter StateA")
            }
        }
}