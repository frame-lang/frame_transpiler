@target rust

system Scope {
    interface:
        run()

    machine:
        $Start {
            run() {
                let x = 1;
                {
                    let x = 2;
                    println!("{}", x);
                }
                println!("{}", x);
            }
        }
    }
}
