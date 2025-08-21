fn main() {
    var hws = HelloWorldWithDomainSystem()
    hws.sayHello()
    hws.sayWorld()
}

system HelloWorldWithDomainSystem {

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
            actionWrite(hello_txt, " ") // use domain variable
        }

        actionWriteWorld() {
            actionWrite(world_txt, "") // use domain variable
        }    

        actionWrite(msg, separator) {
            print(msg, end=separator)
        }

    domain:

        var hello_txt = "Hello"
        var world_txt = "World!"

}