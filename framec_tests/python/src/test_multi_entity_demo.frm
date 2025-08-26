// Frame v0.30 Multi-Entity Demo
// This demonstrates multiple functions and systems working together

// Helper function for formatting output
fn format_message(prefix, msg) {
    print("=== " + prefix + ": " + msg + " ===")
    return prefix + "_" + msg
}

// Logger function that both systems will use
fn log_event(system_name, event_name) {
    print("[LOG] System: " + system_name + ", Event: " + event_name)
}

// Main entry point
fn main() {
    print("Starting Multi-Entity Demo")
    print("---------------------------")
    
    // Test helper function
    var result = format_message("TEST", "helper_works")
    print("Result: " + result)
    
    // Create first system - a simple counter
    var counter = CounterSystem()
    counter.increment()
    counter.increment()
    var count = counter.get_count()
    print("Counter value: " + str(count))
    counter.reset()
    
    // Create second system - a toggle switch
    var toggle = ToggleSystem()
    toggle.switch()
    toggle.switch()
    toggle.switch()
    
    // Create third system - a traffic light with transitions
    var light = TrafficLight()
    light.next()
    light.next()
    light.next()
    light.emergency()
    
    print("---------------------------")
    print("Multi-Entity Demo Complete")
}

// First System: Simple Counter
system CounterSystem {
    
    interface:
        increment()
        get_count() : int
        reset()
    
    machine:
        $Counting {
            increment() {
                log_event("CounterSystem", "increment")
                count = count + 1
                print("Count incremented to: " + str(count))
                return
            }
            
            get_count() : int {
                return = count
            }
            
            reset() {
                log_event("CounterSystem", "reset")
                count = 0
                print("Counter reset to 0")
                return
            }
        }
    
    domain:
        var count : int = 0
}

// Second System: Toggle Switch
system ToggleSystem {
    
    interface:
        switch()
    
    machine:
        $Off {
            switch() {
                log_event("ToggleSystem", "switch_to_on")
                print("Toggle: OFF -> ON")
                -> $On
                return
            }
        }
        
        $On {
            switch() {
                log_event("ToggleSystem", "switch_to_off")
                print("Toggle: ON -> OFF")
                -> $Off
                return
            }
        }
}

// Third System: Traffic Light with Complex Transitions
system TrafficLight {
    
    interface:
        next()
        emergency()
    
    machine:
        $Green {
            next() {
                log_event("TrafficLight", "green_to_yellow")
                print("Traffic Light: GREEN -> YELLOW")
                -> $Yellow
                return
            }
            
            emergency() {
                print("EMERGENCY: Going to RED")
                -> $Red
                return
            }
            
            $>() {
                print("Light is now GREEN")
                return
            }
        }
        
        $Yellow {
            next() {
                log_event("TrafficLight", "yellow_to_red")
                print("Traffic Light: YELLOW -> RED")
                -> $Red
                return
            }
            
            emergency() {
                print("EMERGENCY: Going to RED")
                -> $Red
                return
            }
            
            $>() {
                print("Light is now YELLOW")
                return
            }
        }
        
        $Red {
            next() {
                log_event("TrafficLight", "red_to_green")
                print("Traffic Light: RED -> GREEN")
                -> $Green
                return
            }
            
            emergency() {
                print("Already at RED - safe state")
                return
            }
            
            $>() {
                print("Light is now RED")
                return
            }
        }
}