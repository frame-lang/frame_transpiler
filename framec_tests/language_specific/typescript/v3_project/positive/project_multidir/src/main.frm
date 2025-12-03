@target typescript

import { sayHi } from "./../lib/helper"

system Main {
    interface:
        run()

    machine:
        $Start {
            run() {
                sayHi()
            }
        }
}
