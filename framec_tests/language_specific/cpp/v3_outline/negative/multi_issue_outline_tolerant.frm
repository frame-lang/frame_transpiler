@target cpp

system S {
    machine:
        $A
        e() {
            => $^ oops
            -> $ZZ();
        }
}

