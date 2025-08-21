fn main() {
    var eemd = EnterExitMessagesDemo()
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
                return
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
                return
            }
        }

        $End {
            $>() {
                print("entering $End")
                return
            }
        }
}