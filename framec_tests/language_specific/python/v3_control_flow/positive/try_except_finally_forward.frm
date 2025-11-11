@target python

system S {
    machine:
        $A {
            e() {
                try:
                    => $^
                    try_block()
                except Exception as e:
                    => $^
                    handle(e)
                finally:
                    => $^
                    cleanup()
            }
        }
}

