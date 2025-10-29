# Exercises action locals and mixed typed/untyped domain updates.

system ActionLocals {
    interface:
        start()

    machine:
        $Idle {
            start() {
                announce(5, "first call")
                announce(3, "second call")
                print(total)
                print(note)
            }
        }

    actions:
        announce(step: int, message) {
            total = total + step + step
            note = message
            print(step + step)
        }

    domain:
        var total: int = 1
        var note = "unset"
}

fn main() {
    var runner = ActionLocals()
    runner.start()
}
