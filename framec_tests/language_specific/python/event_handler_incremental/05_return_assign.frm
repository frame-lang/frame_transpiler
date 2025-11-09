# Python incremental: system.return assignment

system IncEv5 {
    interface:
        get() : int = 0

    machine:
        $S {
            get() : int = 5 {
                system.return = 42
                return
            }
        }
}

fn main() {
    t = IncEv5()
    v = t.get()
    print(v)
}

