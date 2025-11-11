@target typescript

system S {
    machine:
        $A {
            e() {
                => $^ /* multiline
                         comment */ nativeAfter();
            }
        }
}

