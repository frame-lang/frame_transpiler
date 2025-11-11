@target typescript

system S {
    machine:
        $A => $P {
            e() {
                let a = [1,2];
                a.push(3);
                => $^; a.length;
            }
        }
        $P { }
}
