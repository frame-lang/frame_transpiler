@target csharp

system S {
    machine:
        $A {
            e() { if (a) { => $^; } else { => $^; } }
        }
}

