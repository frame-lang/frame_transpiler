@target csharp

system S {
    machine:
        $P { e() { } }
        $A => $P {
            e() {
                if (true) {
                    => $^;
                } else {
                    -> $B();
                }
            }
        }
        $B { e() { } }
}
