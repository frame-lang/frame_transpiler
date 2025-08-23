system TestSystem {
    machine:
        $StateA {
            var sysA = nil
            
            $>() {
                print("Enter StateA")
            }
        }
}