fn main() {
    var sys1 = SystemA()
}

system SystemA {
    machine:
        $Idle {
            $>() {
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
    machine:
        $Start {
            $>() {
                return
            }
        }
}