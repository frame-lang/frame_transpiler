@target python

system S {
    machine:
        $A {
            e() {
                x = 1 + 2 - 3 * 4 / 5
                x += 2
                => $^
                str(x)
            }
        }
}

