# Simple state variable test - counter that increments on each trigger
fn main() {
    var counter = SimpleCounter()
    counter.trigger()
    counter.trigger() 
    counter.trigger()
    counter.getCount()
}

system SimpleCounter {
    
    interface:
        trigger()
        getCount()
    
    machine:
        $Counting {
            var count = 0
            
            trigger() {
                count = count + 1
                print("Count incremented to: " + str(count))
                return
            }
            
            getCount() {
                print("Current count is: " + str(count))
                return
            }
        }
}