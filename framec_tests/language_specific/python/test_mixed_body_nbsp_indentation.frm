
system NBSPIndentPy {
    machine:
        $Init {
            start() {
                text = "NBSP indent"
                -> $Next
            }
        }
        $Next {}
}
