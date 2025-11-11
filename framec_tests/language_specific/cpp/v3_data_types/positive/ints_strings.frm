@target cpp

system S {
    machine:
        $A => $P {
            e() { int n=1; const char* s="x"; => $^; n=n+1; }
        }
        $P { }
}
