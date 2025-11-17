@target python

# System blocks intentionally out of order: machine: appears before interface:.
# Expect a semantic error (E113) from the outer grammar validator.

system S {
    machine:
        $A {
            e() { x() }
        }
    interface:
        e()
}

