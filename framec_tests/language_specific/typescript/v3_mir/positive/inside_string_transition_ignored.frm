@target typescript

system S {
    machine:
        $A {
            e() {
                const s1 = "-> $B() should be ignored in string";
                const s2 = '=> $^ should be ignored in string';
                // no Frame segments should be detected in strings
            }
        }
}

