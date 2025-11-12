@target python

system S {
    machine:
        $A {
            e() {
                x = """
                literal with -> $B() and => $^
                """
                native()
            }
        }
}

