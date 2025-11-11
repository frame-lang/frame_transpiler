@target typescript

system S {
    machine:
        $A => $P {
            e() {
                let x = cond ? 1 : 2;
                => $^; x.toString();
            }
        }
        $P { }
}
