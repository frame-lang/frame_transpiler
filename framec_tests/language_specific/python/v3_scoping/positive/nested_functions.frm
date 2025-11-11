@target python

system S {
    machine:
        $A => $P {
            e() {
                def outer():
                    v = 1
                    def inner():
                        return v + 1
                    return inner()
                => $^
                outer()
            }
        }
        $P { }
}
