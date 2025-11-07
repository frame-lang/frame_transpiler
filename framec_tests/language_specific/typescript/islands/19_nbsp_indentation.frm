 

system NBSPIndentTS {
    operations:
    op1() {
        const msg = "indented with NBSP";
        -> $Next
    }
    machine:
        $Init { op1() { } }
        $Next {}
}
