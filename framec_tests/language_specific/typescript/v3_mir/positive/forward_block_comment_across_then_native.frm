@target typescript

system S {
    machine:
        $A => $P {
            e() {
                => $^ /* multiline
                         comment */ nativeAfter();
            }
        }
        $P { }
}
