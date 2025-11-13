@target python

system S {
    machine:
        $A {
            e() {
                -> $B(1)
                msg = f"value: {x"  # malformed f-string (missing closing brace)
            }
        }
        $B {
        }
}

