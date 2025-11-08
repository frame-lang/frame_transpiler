fn main() {
    var sys = LifecycleDemo()
    sys.start()
}

system LifecycleDemo {
    interface:
        start()

    machine:
        $StateA {
            start() {
                console.log("Starting lifecycle demo");
                -> $StateB
            }
        }

        $StateB {
            $>() {
                console.log("Entered StateB");
                return;
            }
        }
}

