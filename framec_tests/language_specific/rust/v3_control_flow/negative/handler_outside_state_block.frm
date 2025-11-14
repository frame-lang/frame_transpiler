@target rust
# @expect: E404

system S {
    machine:
        // Handler outside of any state block
        e() { $$[+] }
}

