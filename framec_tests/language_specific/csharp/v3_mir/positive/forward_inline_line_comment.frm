@target csharp

system S {
    machine:
        $A => $P {
            e() {
                => $^ // inline ok
            }
        }
        $P { }
}
