@target c

system S {
    machine:
        $A {
            e() {
                => $^ /* multiline
                         comment */ native_after();
            }
        }
}

