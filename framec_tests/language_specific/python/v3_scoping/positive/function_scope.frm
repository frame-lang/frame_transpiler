@target python

system S {
    machine:
        $A {
            e() {
                def f():
                    return 1
                => $^
                f()
            }
        }
}

