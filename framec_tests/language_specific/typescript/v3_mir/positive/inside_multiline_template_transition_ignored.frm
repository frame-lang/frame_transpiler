@target typescript

system S {
    machine:
        $A {
            e() {
                const t = `template line 1 with -> $B()
template line 2 with => $^
template ${`nested ${'${ignored}'} backticks`} end`;
                native();
            }
        }
}

