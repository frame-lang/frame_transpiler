@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    btmd = BasicTransitionBehaviorDemo() 
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
            }
        }
        
        $End {
             $>() {
                 print("entering $End state")
                 return
             }
        }
}
