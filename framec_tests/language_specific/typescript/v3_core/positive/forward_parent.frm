@target typescript

system S {
    machine:
        $A {
            e() {
                => $^
                x();
            }
        }
}

