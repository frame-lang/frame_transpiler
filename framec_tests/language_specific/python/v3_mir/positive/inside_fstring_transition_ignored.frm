@target python

system S {
    machine:
        $A {
            e() {
                msg = f"transition token here: -> $B() ignored"
                msg2 = f"forward token here: => $^ ignored"
            }
        }
}

