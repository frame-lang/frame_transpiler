@target typescript

system S {
    machine:
        $A {
            e() {
                let a = [1,2];
                a.push(3);
                => $^; a.length;
            }
        }
}

