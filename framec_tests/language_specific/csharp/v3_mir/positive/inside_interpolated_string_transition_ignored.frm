@target csharp

system S {
    machine:
        $A {
            e() {
                var name = "B";
                var s = $"interpolated with -> $B() and => $^ {name}";
                native();
            }
        }
}

