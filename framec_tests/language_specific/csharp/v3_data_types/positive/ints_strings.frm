@target csharp

system S {
    machine:
        $A => $P {
            e() { int n=1; string s="x"; => $^; s.ToUpper(); }
        }
        $P { }
}
