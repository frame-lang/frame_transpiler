fn main() {
    var hws = HelloWorldSystem()
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
                actionWriteHello() // call action
                -> $World
                return
            }
        }
        
        $World {
            sayWorld() {
                actionWriteWorld() // call action
                -> $Done
                return
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