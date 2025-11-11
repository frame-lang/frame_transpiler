@target python

system S {
    machine:
        $A {
            e() {
                -> $B(1)
                def f(: pass  # invalid def syntax
            }
        }
        $B {
        }
}

