@target csharp

system S {
    machine:
        $A => $P {
            e() {
                => $^
            }
        }
        $P { }
}
