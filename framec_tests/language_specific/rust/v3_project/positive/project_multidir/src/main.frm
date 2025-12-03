@target rust

import crate::helper::say_hi

system Main {
    interface:
        run()

    machine:
        $Start {
            run() {
                say_hi();
            }
        }
}
