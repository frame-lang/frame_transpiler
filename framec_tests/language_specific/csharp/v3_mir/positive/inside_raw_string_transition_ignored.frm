@target csharp

system S {
    machine:
        $A {
            e() {
                var s = @"raw string with -> $B() and => $^ inside";
                native();
            }
        }
}

