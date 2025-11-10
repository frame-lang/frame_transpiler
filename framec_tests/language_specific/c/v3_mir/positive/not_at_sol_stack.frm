@target c

system S {
    machine:
        $A {
            e() {
                if (x) { a(); $$[+]; b(); }
            }
        }
}
