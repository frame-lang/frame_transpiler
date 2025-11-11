@target rust

system S {
    machine:
        $A => $P {
            e() {
                => $^ /* multiline
                         comment */ native_after();
            }
        }
        $P { }
}
