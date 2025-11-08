system LifecycleSystem {
    interface:
        next()

    machine:
        $StateA {
            next() {
                console.log("StateA.next() -> StateB");
                -> $StateB
            }
        }

        $StateB => $StateA {
            $>() {
                console.log("StateB enter - forwarding to parent");
                => $^
                return;
            }
        }
}

fn main() {
    var sys = LifecycleSystem()
    sys.next()
}

