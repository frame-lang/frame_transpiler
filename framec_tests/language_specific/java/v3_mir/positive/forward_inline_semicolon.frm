@target java

system S {
    machine:
        $A => $P {
            e() {
                => $^; native();
            }
        }
        $P { }
}
