system TestSystem {
    machine:
        $StateA {
            var sysA = None
            
            $>() {
                print("Enter StateA")
            }
        }
}