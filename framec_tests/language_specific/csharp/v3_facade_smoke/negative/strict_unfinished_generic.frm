@target csharp

system S {
    machine:
        $A {
            e() {
                -> $B();
                List<int x; // missing '>'
            }
        }
        $B {
        }
}

