# TS override: History103 using TS-native bodies
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
                console.log("In $A");
                return;
            }

            gotoC() {
                console.log("$A pushing to stack and going to $C");
                $$[+]
                -> $C
            }
        }

        $B {
            $>() {
                console.log("In $B");
                return;
            }

            gotoC() {
                console.log("$B pushing to stack and going to $C");
                $$[+]
                -> $C
            }
        }

        $C {
            $>() {
                console.log("In $C");
                return;
            }

            ret() {
                console.log("Popping from stack and returning");
                $$[-]
            }
        }
}

