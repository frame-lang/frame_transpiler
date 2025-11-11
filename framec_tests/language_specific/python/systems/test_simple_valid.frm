@target python

# Simple valid Frame file for testing validation
system SimpleSystem {
    interface:
        doSomething()
        
    machine:
        $Start {
            doSomething() {
                print("Hello from Frame!")
            }
        }
}
