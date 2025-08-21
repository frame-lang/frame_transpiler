fn main() {
    var btmd = BasicTransitionBehaviorDemo() 
    btmd.next()
}

system BasicTransitionBehaviorDemo {

    interface:
        next()

    machine:
        $Start {
            <$() {
                print("exiting $Start state")
                return
            }
            
            next() {
                print("transitioning to $End state")
                -> $End
                return
            }
        }
        
        $End {
             $>() {
                 print("entering $End state")
                 return
             }
        }
}