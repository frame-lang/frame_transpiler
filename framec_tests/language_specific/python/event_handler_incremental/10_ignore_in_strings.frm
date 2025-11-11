@target python

# Python incremental: Frame-statement tokens inside strings must be ignored

system IncEv10 {
    interface:
        go()

    machine:
        $S {
            go() {
                print("not a Frame statement: -> $Nowhere")
                print('ignore $$[+] and => $^ inside strings')
                # real Frame statement follows
                -> $T
            }
        }
        $T { $>() { return } }
}

fn main() {
    t = IncEv10()
    t.go()
}
