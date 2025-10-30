# Validates state stack push/pop semantics in the LLVM backend.

system StateStackPop {
    interface:
        start()
        finish()
        report()

    machine:
        $A {
            $>() {
                print("enter A")
                return
            }

            <$() {
                print("exit A")
                return
            }

            start() {
                print("A start")
                $$[+]
                -> $B
            }

            finish() {
                print("A finish (noop)")
            }

            report() {
                print("A report")
            }
        }

        $B {
            $>() {
                print("enter B")
                return
            }

            <$() {
                print("exit B")
                return
            }

            finish() {
                print("B finish")
                -> $$[-]
            }

            start() {
                print("B start (noop)")
            }

            report() {
                print("B report (noop)")
            }
        }

    domain:
        var counter: int = 0
}

fn main() {
    var stack = StateStackPop()
    stack.start()
    stack.finish()
    stack.report()
}
