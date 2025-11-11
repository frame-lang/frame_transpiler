@target rust

system S {
    machine:
        $A {
            e() {
                => $^
                let x = ; // malformed native statement
            }
        }
}

