# Validates state parameters propagation in the LLVM backend.

system StateParametersBasic {
    interface:
        start()
        report()

    machine:
        $Idle {
            start() {
                print("idle start")
                -> $Active(42, "ready")
            }
        }

        $Active(timeout: int, label: string) {
            $>() {
                print("active enter")
                print(timeout)
                print(label)
            }

            report() {
                print("active report")
                print(timeout)
                print(label)
            }
        }
}

fn main() {
    var machine = StateParametersBasic()
    machine.start()
    machine.report()
}
