@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system HelloWorldSystem {

    interface:
    
        sayHello()
        sayWorld()

    machine:

        $Hello {
            sayHello() {
                -> $World # Transition to $World state
            }
        }
        
        $World {
            sayWorld() {
                -> $Done # Transition to $Done state
            }
        }

        $Done {
        }

}
