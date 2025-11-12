@target typescript

system S {
    machine:
        $A {
            e() {
                => $^ oops
                -> $ZZ() ; a();
                -> $ (1
            }
        }
}

