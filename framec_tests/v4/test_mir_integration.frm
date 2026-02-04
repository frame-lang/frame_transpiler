@@target python

@@system TrafficLight {
    interface:
        start()
        stop()
    
    machine:
        $Red {
            start() {
                print("Starting from red")
                -> $Green  # Transition within native code
            }
            
            stop() {
                print("Already stopped")
            }
        }
        
        $Green {
            start() {
                print("Already running")
            }
            
            stop() {
                print("Stopping at green")
                -> $Yellow
            }
        }
        
        $Yellow {
            start() {
                print("Starting from yellow")
                -> $Green
            }
            
            stop() {
                print("Transitioning to red")
                -> $Red
            }
        }
}