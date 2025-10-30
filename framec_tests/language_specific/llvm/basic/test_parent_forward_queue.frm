# Validates that parent forwarding enqueues the event and executes once.

system ParentForwardQueue {
    interface:
        start()
        report()

    machine:
        $Child => $Parent {
            start() {
                print("child before parent")
                => $^
            }

            report() {
                print(count)
            }
        }

        $Parent {
            start() {
                print("parent handling")
                count = count + 1
            }
        }
    }

    domain:
        var count: int = 0
}

fn main() {
    var system = ParentForwardQueue()
    system.start()
    system.report()
}
