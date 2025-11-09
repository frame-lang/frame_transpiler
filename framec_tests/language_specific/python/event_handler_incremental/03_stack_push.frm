# Python incremental: stack push

system IncEv3 {
    interface:
        enter()

    machine:
        $A {
            enter() {
                $$[+]
                -> $B
            }
        }
        $B {
            $>() { return }
        }
}

fn main() {
    t = IncEv3()
    t.enter()
}

