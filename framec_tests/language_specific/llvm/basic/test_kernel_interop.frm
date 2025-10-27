# Validates that systems allocate a runtime kernel handle.

system RuntimeProbe {
    interface:
        noop()

    machine:
        $Start {
            noop() {
                print("noop")
            }
        }
}

fn main() {
    var probe = RuntimeProbe()
    probe.noop()
}
