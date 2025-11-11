@target typescript

system S {
    machine:
        $A {
            e() {
                const s = new Set<number>();
                s.add(1); s.add(2);
                => $^; s.has(1);
            }
        }
}

