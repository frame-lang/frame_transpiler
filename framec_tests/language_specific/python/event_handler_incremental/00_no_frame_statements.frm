@target python

# Python incremental: handler with no Frame statements

system IncEv0 {
    interface:
        go()

    machine:
        $Idle {
            go() {
                x = 1
                y = 2
                print(x + y)
                return
            }
        }
}

fn main() {
    t = IncEv0()
    t.go()
}
