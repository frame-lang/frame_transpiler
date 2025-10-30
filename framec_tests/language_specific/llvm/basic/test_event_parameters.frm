# Validates interface event parameters are passed through handler and prints correctly.

system EventParameters {
    interface:
        start(count: int, label: string)
        complete()

    machine:
        $Idle {
            start(count: int, label: string) {
                print("idle start")
                print(count)
                print(label)
                self.total = self.total + count
                -> $Active
            }
        }

        $Active {
            $>() {
                print("active enter")
                print(self.total)
                return
            }

            complete() {
                print("done")
            }
        }

    domain:
        var total: int = 0
}

fn main() {
    var runner = EventParameters()
    runner.start(3, "launch")
    runner.complete()
}
