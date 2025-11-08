# TS override: controlled HSM loop with counter and debug output
fn main() {
    var hsm = SimpleHSM()
    hsm.trigger()
    console.log("Done!");
}

system SimpleHSM {
    interface:
        trigger()
    
    machine:
        $Parent {
            var count = 0
            
            trigger() {
                console.log("Parent.trigger() called");
                count = count + 1;
                console.log("Count is now: " + String(count));
                if (count < 10) {
                    console.log("Transitioning to Child");
                    -> $Child
                } else {
                    console.log("Count >= 10, stopping");
                }
                return;
            }
            
            $>() {
                console.log("Parent enter event");
                return;
            }
        }
        
        $Child => $Parent {
            $>() {
                console.log("Child enter - dispatching to parent");
                => $^
                console.log("After parent dispatch");
                return;
            }
        }
}

