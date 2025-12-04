@target rust

system Ops {
    interface:
        run()

    machine:
        $Start {
            run() {
                let t = true && (false || true);
                println!("{}", t);
            }
        }
    }
}
