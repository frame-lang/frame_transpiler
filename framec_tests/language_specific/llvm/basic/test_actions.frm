# Actions support smoke test for the LLVM backend

system ActionExample {
    interface:
        trigger()

    machine:
        $Start {
            trigger() {
                announce("Action complete")
            }
        }

    actions:
        announce(new_message: string) {
            print("Action invoked")
            flag = false
            message = new_message
            print(new_message)
        }

    domain:
        var flag: bool = true
        var message: string = "Action pending"
}

fn main() {
    var instance = ActionExample()
    instance.trigger()
}
