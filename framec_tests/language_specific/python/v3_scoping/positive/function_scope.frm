@target python

system S {
    machine:
        $A => $P {
            e() {
                def f():
                    return 1
                => $^
                f()
            }
        }
        $P { }
}
