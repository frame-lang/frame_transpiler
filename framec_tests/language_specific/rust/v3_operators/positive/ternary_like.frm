@target rust

system Ops {
    interface:
        run()

    machine:
        $Start {
            run() {
                let cond = true;
                let v = if cond { 1 } else { 0 };
                println!("{}", v);
            }
        }
    }
}
