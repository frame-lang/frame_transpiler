@target csharp

system S {
    machine:
        $A => $P {
            e() {
                => $^ /* inline block ok */
            }
        }
        $P { }
}
