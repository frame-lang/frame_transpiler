system SystemA {
    interface:
        next()
        
    machine:
        $Start {
            next() {
                print("SystemA next")
                return = true
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
                return = false
            }
        }
}