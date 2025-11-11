@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    hws = HelloWorldSystem()
    hws.sayHello()
    hws.sayWorld()
}

system HelloWorldSystem {

    interface:
    
        sayHello()
        sayWorld()

    machine:

        $Hello {
            sayHello() {
                actionWriteHello() # call action
                -> $World
            }
        }
        
        $World {
            sayWorld() {
                actionWriteWorld() # call action
                -> $Done
            }
        }

        $Done {
        }

    actions: 

        actionWriteHello() {
            actionWrite("Hello", " ")
        }

        actionWriteWorld() {
            actionWrite("World!", "")
        }    

        actionWrite(msg, separator) {
            print(msg, end=separator)
        }
}
