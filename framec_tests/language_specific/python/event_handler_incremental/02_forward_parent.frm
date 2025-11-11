@target python

# Python incremental: parent forward Frame statement

system IncEv2 {
    interface:
        ping()

    machine:
        $Parent {
            $Child {
                ping() {
                    => $^
                }
            }

            ping() {
                print("handled in parent")
                return
            }
        }
}

fn main() {
    t = IncEv2()
    t.ping()
}
