@target csharp

system S {
    machine:
        $A => $P {
            e() {
                => $^ /* block */ // trailing
            }
        }
        $P { }
}
