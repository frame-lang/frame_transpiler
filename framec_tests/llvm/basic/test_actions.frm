# Actions support smoke test for LLVM backend

system ActionExample {
    interface:
        trigger()

    machine:
        $Start {
            trigger() {
                announce()
            }
        }

    actions:
        announce() {
            print("Action invoked")
            flag = false
            message = "Action complete"
            print(message)
        }

    domain:
        var flag: bool = true
        var message: string = "Action pending"
}

fn main() {
    var instance = ActionExample()
    instance.trigger()
}
