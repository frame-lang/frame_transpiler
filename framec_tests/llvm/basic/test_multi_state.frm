# Multi-state dispatch smoke test for LLVM backend

system MultiStateDemo {
    interface:
        trigger()
        status()

    machine:
        $Start {
            trigger() {
                print("Start -> Running")
                -> $Running
            }
            status() {
                print("status: start")
            }
        }

        $Running {
            trigger() {
                print("Already running")
            }
            status() {
                print("status: running")
                -> $Done
            }
        }

        $Done {
            status() {
                print("status: done")
            }
        }
}

fn main() {
    var demo = MultiStateDemo()
    demo.trigger()
    demo.status()
    demo.status()
}
