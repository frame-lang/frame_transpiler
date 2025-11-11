@target csharp

system S {
    actions:
        bad() { => $^; }
    machine:
        $A {
            e() { x(); }
        }
}

