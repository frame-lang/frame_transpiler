@target csharp

system S {
    machine:
        $A => $P {
            e() {
                => $^; native();
            }
        }
        $P { }
}
