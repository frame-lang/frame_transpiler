@target c

system S {
    machine:
        $A
        e() {
            => $^ oops
            -> $ZZ();
        }
}

