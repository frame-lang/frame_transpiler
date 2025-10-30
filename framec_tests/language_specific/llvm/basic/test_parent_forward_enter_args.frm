# Validates parent forwarding preserves queued enter args.

system ParentForwardEnterArgs {
    interface:
        start()

    machine:
        $Child => $Parent {
            start() {
                print("child forward")
                => $^
            }
        }

        $Parent {
            start() {
                print("parent handling")
                -> ("from parent", 7) $Active
            }
        }

        $Active {
            $>(msg: string, value: int) {
                print("active enter")
                print(msg)
                print(value)
            }
        }
}

fn main() {
    var runner = ParentForwardEnterArgs()
    runner.start()
}
