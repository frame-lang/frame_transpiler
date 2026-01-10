@@target python

system Simple {
    interface:
        go()
    
    machine:
        $StateA {
            go() {
                print("In A")
                -> $StateB
            }
        }
        
        $StateB {
            go() {
                print("In B")
            }
        }
}