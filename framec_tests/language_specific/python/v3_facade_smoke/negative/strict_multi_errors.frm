@target python

system S {
    machine:
        $A {
            e() {
                -> $B(1)
                x =              # bad assignment
                def f(: pass      # malformed def
                if :
                    pass         # malformed if
            }
        }
        $B {
        }
}

