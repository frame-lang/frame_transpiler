@target python_3

# Native imports and setup before system
import json

def log_transition(from_state, to_state):
    """Native helper function"""
    print(f"Transition: {from_state} -> {to_state}")

system TrafficLight {
    interface:
        start()
        tick() 
        
    machine:
        $Red {
            tick() {
                log_transition("Red", "Green")
                -> $Green
            }
        }
        
        $Green {
            tick() {
                log_transition("Green", "Red")  
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
    light.tick()  # Red -> Green
    light.tick()  # Green -> Red
    print("SUCCESS: V3 native code preservation test")
    
if __name__ == "__main__":
    test_traffic_light()