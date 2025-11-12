@target rust

system S {
    machine:
        $A {
            e() {
                /* outer /* inner with -> $B() and => $^ */ still comment */
                native();
            }
        }
}

