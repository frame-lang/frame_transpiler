@target csharp

system S {
    machine:
        $A {
            e() {
                var s = "-> $B() and => $^ ignored";
                a();
            }
        }
}

