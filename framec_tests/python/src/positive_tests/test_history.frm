# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    var sys = History103()
    sys.gotoC()
    sys.ret()
}

system History103 {
    interface:
        gotoC()
        ret()
    
    machine:
        $A {
            $>() {
                print("In $A")
                return
            }
            
            gotoC() {
                print("$A pushing to stack and going to $C")
                $$[+]
                -> "$$[+]" $C
            }
        }

        $B {
            $>() {
                print("In $B")
                return
            }
            
            gotoC() {
                print("$B pushing to stack and going to $C")
                $$[+]
                -> "$$[+]" $C
            }
        }

        $C {
            $>() {
                print("In $C")
                return
            }
            
            ret() {
                print("Popping from stack and returning")
                -> "$$[-]" $$[-]
            }
        }
}