@target rust

system Ops {
    interface:
        run()

    machine:
        $Start {
            run() {
                let a = 2 + 3 * 4;
                let b = (10 - 2) / 4;
                let c = 7 % 3;
                println!("{} {} {}", a, b, c);
            }
        }
    }
}
