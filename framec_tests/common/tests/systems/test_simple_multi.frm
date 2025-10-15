# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system SystemA {
    interface:
        next()
        
    machine:
        $Start {
            next() {
                print("SystemA next")
                system.return = true
            }
        }
}

system SystemB {
    interface:
        next()
        
    machine:
        $Start {
            next() {
                print("SystemB next")  
                system.return = false
            }
        }
}