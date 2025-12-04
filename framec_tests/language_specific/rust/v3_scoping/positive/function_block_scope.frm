@target rust

fn helper() {
    let y = 5;
    println!("{}", y);
}

system Scope {
    interface:
        run()

    machine:
        $Start {
            run() {
                helper();
                let y = 10;
                println!("{}", y);
            }
        }
    }
}
