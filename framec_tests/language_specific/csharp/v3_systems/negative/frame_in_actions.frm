@target csharp

system S {
    actions:
        fn bad() { => $^; }
    machine:
        $A {
            e() { x(); }
        }
}
