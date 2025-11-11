@target csharp

system S {
    machine:
        $A {
            e() { int n=1; string s="x"; => $^; s.ToUpper(); }
        }
}

