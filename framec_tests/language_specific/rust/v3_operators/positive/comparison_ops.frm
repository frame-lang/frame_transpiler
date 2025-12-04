@target rust

system Ops {
    interface:
        run()

    machine:
        $Start {
            run() {
                let x = 3;
                let y = 5;
                println!("{} {} {}", x < y, x > y, x == y);
            }
        }
    }
}
