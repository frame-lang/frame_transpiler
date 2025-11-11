@target typescript

system S {
    machine:
        $A => $P {
            ev(n:number) {
                => $^; n.toString();
            }
        }
        $P { }
}
