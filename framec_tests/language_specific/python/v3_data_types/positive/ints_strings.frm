@target python

system S {
    machine:
        $A {
            e() {
                n = 42
                s = "hello"
                => $^
                s.upper()
            }
        }
}

