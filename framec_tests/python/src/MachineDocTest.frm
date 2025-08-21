system HelloWorldSystem {

    interface:
    
        sayHello()
        sayWorld()

    machine:

        $Hello {
            sayHello() {
                -> $World // Transition to $World state
            }
        }
        
        $World {
            sayWorld() {
                -> $Done // Transition to $Done state
            }
        }

        $Done {
        }

}