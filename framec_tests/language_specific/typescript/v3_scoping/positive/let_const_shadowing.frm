@target typescript

system S {
    machine:
        $A {
            e() {
                let x = 1;
                {
                    const x = 2;
                    => $^; x.toString();
                }
                x = 4;
            }
        }
}

