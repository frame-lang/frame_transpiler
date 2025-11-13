@target python

system S {
    machine:
        $A {
            e() {
                -> $B(1)
                if True:
                  x = 1
                     y = 2  # inconsistent indentation
            }
        }
        $B {
        }
}

