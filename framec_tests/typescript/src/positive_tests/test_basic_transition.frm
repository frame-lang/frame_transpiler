# Test basic state transitions in TypeScript
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

system TrafficLight {
    interface:
        start()
        stop()
        tick()
    
    machine:
        $Red {
            start() {
                print("Starting from Red")
                -> $Green
            }
            
            stop() {
                print("Stopping from Red")
                -> $Off
            }
        }
        
        $Green {
            tick() {
                print("Green timeout")
                -> $Yellow
            }
            
            stop() {
                print("Emergency stop from Green")
                -> $Off
            }
        }
        
        $Yellow {
            tick() {
                print("Yellow timeout")
                -> $Red
            }
            
            stop() {
                print("Stopping from Yellow")
                -> $Off
            }
        }
        
        $Off {
            start() {
                print("Restarting")
                -> $Red
            }
        }
}