# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys1 = SystemA()
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