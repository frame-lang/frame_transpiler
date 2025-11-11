@target typescript

system S {
    machine:
        $A => $P {
            e() {
                let n = 42;
                let s = "hello";
                => $^; n.toString();
            }
        }
        $P { }
}
