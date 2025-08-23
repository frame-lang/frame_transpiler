system BlocksTest {
    operations:
        work() {
            print("ops: work called")
        }

    interface:
        go()

    machine:
        $Start {
            $>() {
                print("machine: start entered")  
                self.work()
                self.finish()
            }
            
            go() {
                print("machine: go interface called")
            }
        }

    actions:
        finish() {
            print("actions: finish called")
            print("domain: value=" + str(self.value))
        }

    domain:
        var value : int = 123
}