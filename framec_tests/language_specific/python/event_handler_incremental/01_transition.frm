@target python

# Python incremental: single transition Frame statement

system IncEv1 {
    interface:
        start()

    machine:
        $A {
            start() {
                -> $B
            }
        }

        $B {
            $>() {
                print("entered B")
                return
            }
        }
}

fn main() {
    t = IncEv1()
    t.start()
}
