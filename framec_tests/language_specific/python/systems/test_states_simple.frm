# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    eemd = EnterExitMessagesDemo()
    eemd.next()
    eemd.next()
}

system EnterExitMessagesDemo {
    interface:
        next()

    machine:
        $Begin {
            $>() {
                print("entering $Begin")
                return
            }
            <$() {
                print("exiting $Begin")
                return
            }

            next() {
                -> $Working
            }
        }

        $Working {
            $>() {
                print("entering $Working")
                return
            }
            <$() {
                print("exiting $Working")
                return
            }

            next() {
                -> $End
            }
        }

        $End {
            $>() {
                print("entering $End")
                return
            }
        }
}