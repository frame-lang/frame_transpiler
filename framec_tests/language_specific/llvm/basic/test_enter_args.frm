# Validates queued enter argument propagation in the LLVM backend.

system EnterArgsDemo {
    interface:
        trigger()
        check()

    machine:
        $Idle {
            trigger() {
                print("idle trigger")
                -> ("first", 1) $Active
            }
        }

        $Active {
            $>(message: string, count: int) {
                print("active enter")
                print(message)
                print(count)
            }

            trigger() {
                print("active trigger")
                -> ("again", 2) $Active
            }

            check() {
                print("active check done")
            }
        }
}

fn main() {
    var demo = EnterArgsDemo()
    demo.trigger()
    demo.trigger()
    demo.check()
}
