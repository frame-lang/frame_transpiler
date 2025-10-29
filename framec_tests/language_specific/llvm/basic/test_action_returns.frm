# Action return value smoke test for the LLVM backend

system ActionReturnDemo {
    interface:
        start()

    machine:
        $Start {
            start() {
                message = compose_message()
                announce(message)
            }
        }

    actions:
        compose_message(): string {
            return "Action return success"
        }

        announce(text: string) {
            print("Announcement:")
            print(text)
        }

    domain:
        var message: string = "unset"
}

fn main() {
    var demo = ActionReturnDemo()
    demo.start()
}
