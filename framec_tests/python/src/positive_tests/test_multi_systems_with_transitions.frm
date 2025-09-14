fn main() {
    var sys1 = SystemA()
    sys1.start()
}

system SystemA {
    interface:
        start()
        
    machine:
        $Idle {
            $>() {
                return
            }
            start() {
                -> $Running
                return
            }
        }
        
        $Running {
            $>() {
                return
            }
        }
}

system SystemB {
    interface:
        activate()
        
    machine:
        $Waiting {
            $>() {
                return
            }
            activate() {
                -> $Active
                return
            }
        }
        
        $Active {
            $>() {
                return
            }
        }
}