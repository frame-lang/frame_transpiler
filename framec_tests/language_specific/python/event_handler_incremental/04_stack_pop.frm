# Python incremental: stack pop

system IncEv4 {
    interface:
        back()

    machine:
        $B {
            back() { $$[-] }
        }
}

fn main() {
    t = IncEv4()
    t.back()
}

