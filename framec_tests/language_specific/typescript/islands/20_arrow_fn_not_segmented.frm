 

system ArrowFnNotSegmented {
    operations:
    op1() {
        const f = () => { return 1; };
        -> $Next
    }
    machine:
        $Init { op1() { } }
        $Next {}
}
