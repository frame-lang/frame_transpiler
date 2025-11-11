@target typescript

system S {
    machine:
        $A {
            e() {
                let x = 1;
                {
                    let x = 2;
                    => $^; x.toString();
                }
                x = 3;
            }
        }
}

