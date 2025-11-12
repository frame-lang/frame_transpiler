@target java

system S {
    machine:
        $A
        e() {
            => $^ oops
            -> $ZZ();
        }
}

