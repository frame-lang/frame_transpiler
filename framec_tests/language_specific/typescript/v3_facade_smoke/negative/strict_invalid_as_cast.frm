@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B();
                const x = 1 as ; // missing type after 'as'
            }
        }
        $B {
        }
}

