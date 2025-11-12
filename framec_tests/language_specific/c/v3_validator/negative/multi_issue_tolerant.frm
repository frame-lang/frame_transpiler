@target c

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

