@@target python_3

# Native imports and setup before system
import json

def log_transition(from_state, to_state):
    """Native helper function"""
    print(f"Transition: {from_state} -> {to_state}")

@@persist
@@system TrafficLight {
    interface:
        start()
        tick() 
        pedestrian()
        emergency()
        
    machine:
        $Red {
            pedestrian() {
                print("Pedestrian button pressed in Red - staying Red")
            }
            
            tick() {
                log_transition("Red", "Green")
                -> $Green
            }
            
            emergency() {
                -> $Emergency
            }
        
        }
        
        $Green {
            tick() {
                log_transition("Green", "Yellow")  
                -> $Yellow
            }
            
            pedestrian() {
                print("Pedestrian button pressed - going to Yellow")
                -> $Yellow
            }
            
            emergency() {
                -> $Emergency
            }
            
        }
        
        $Yellow {
            tick() {
                log_transition("Yellow", "Red")
                -> $Red
            }
            
            emergency() {
                -> $Emergency
            }
            
        }
        
        $Emergency {
            tick() {
                print("Emergency resolved - returning to Red")
                -> $Red
            }
        }
            
    actions:
        start() {
            print("Traffic light system started")
        }
}

# Native test code after system
def test_traffic_light():
    """Test the traffic light system"""
    light = TrafficLight()
    light.start()
    
    # Should start in Red (first state)
    light.tick()  # Red -> Green
    light.tick()  # Green -> Yellow  
    light.tick()  # Yellow -> Red
    
    # Test emergency
    light.emergency()  # -> Emergency
    light.tick()       # Emergency -> Red
    
    print("SUCCESS: Traffic light v4 test complete")
    
if __name__ == "__main__":
    test_traffic_light()