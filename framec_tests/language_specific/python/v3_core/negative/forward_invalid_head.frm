@target python

system S {
    machine:
        $A {
            e() {
                => $B  # invalid forward head
            }
        }
}

