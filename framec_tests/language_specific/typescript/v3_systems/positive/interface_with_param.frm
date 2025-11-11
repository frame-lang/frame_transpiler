@target typescript

system S {
    machine:
        $A {
            ev(n:number) {
                => $^; n.toString();
            }
        }
}
