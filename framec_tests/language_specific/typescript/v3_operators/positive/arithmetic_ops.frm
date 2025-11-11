@target typescript

system S {
    machine:
        $A {
            e() {
                let x = 1 + 2 - 3 * 4 / 5;
                x += 2;
                => $^; x.toString();
            }
        }
}

