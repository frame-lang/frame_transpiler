fn main() {
    var sys1 = SystemA()
    sys1.start()
}

system SystemA {
    interface:
        start()
        
    machine:
        $Start {
            $>() {
                return
            }
            start() {
                return
            }
        }
}

system SystemB {
    interface:
        activate()
        
    machine:
        $Start {
            $>() {
                return
            }
            activate() {
                return
            }
        }
}