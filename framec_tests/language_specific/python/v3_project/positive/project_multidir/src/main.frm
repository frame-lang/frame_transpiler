@target python_3

import from lib.helper import say_hi

system Main {
    interface:
        run()

    machine:
        $Start {
            run() {
                say_hi()
            }
        }
}
