@target csharp

system S {
    machine:
        $A {
            e() {
                var s = "inside -> $B() and => $^ should be ignored";
                native();
            }
        }
}

