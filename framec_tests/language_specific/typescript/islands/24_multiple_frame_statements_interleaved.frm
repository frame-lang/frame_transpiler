 

system MultiDirectivesTS {
    operations:
    op1() {
        let x = 0;
        $$[+]
        x += 1;
        => $^
        x += 2;
        -> $Next
        x += 3;
        $$[-]
    }
    machine:
        $Init { op1() { } }
        $Parent { $Child {} }
        $Next {}
}
