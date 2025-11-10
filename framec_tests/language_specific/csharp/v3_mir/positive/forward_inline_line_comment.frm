@target csharp

system S {
    machine:
        $A {
            e() {
                => $^ // inline ok
            }
        }
}

