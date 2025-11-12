@target rust
# @expect: E200

system S {
    machine:
        $A {
            e() {
                => $B  // invalid forward head; only => $^ is allowed
            }
        }
        $B { }
}

