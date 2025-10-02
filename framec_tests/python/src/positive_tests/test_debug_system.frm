# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test file for debugging Frame systems (state machines)
# Tests state transitions, enter/exit events, interface calls, and event handlers

fn main() {
    print("=== Starting System Debug Test ===")
    
    # Create and test the traffic light system
    var light = TrafficLight()
    
    # Test initial state
    print("Initial state: should be Red")
    
    # Cycle through states
    light.next()  # Red -> Green
    light.next()  # Green -> Yellow
    light.next()  # Yellow -> Red
    light.next()  # Red -> Green again
    
    # Test interface methods
    var status = light.get_status()
    print("Current status: " + status)
    
    # Test emergency - goes to Offline state
    print("\n=== Testing Emergency Mode ===")
    light.emergency()  # Should go to Offline
    
    # Try operations while offline
    status = light.get_status()
    print("Status while offline: " + status)
    light.next()  # Should not work
    light.set_timer(10)  # Should not work
    
    # Restart the system
    print("\n=== Restarting System ===")
    light.restart()  # Should go back to Red
    
    # Test with timer after restart
    light.set_timer(3)
    light.auto_cycle()  # Should cycle automatically
    
    print("=== System Debug Test Complete ===")
    return
}

# Traffic Light System - demonstrates state machine concepts
system TrafficLight {
    
    # Interface - public methods that can be called
    interface:
        next()
        emergency()
        restart()
        get_status()
        set_timer(seconds)
        auto_cycle()
    
    # State machine definition
    machine:
    
        # Initial state
        $Red {
            # Enter event - called when entering this state
            $>() {
                print("  [Red.$enter] Light is now RED - STOP")
                return
            }
            
            # Exit event - called when leaving this state  
            <$() {
                print("  [Red.$exit] Leaving Red state")
                return
            }
            
            # Event handlers
            next() {
                self.log_transition("Red", "Green")
                self.wait()
                -> $Green
            }
            
            get_status() {
                return "Cycle #" + str(self.cycle_count) + " - Timer: " + str(self.timer_seconds) + "s - State: RED"
            }
            
            set_timer(seconds) {
                self.timer_seconds = seconds
                print("    [Action] Timer set to " + str(seconds) + " seconds")
                return
            }
            
            auto_cycle() {
                print("  [Red.auto_cycle] Starting auto-cycle")
                for i in range(3) {
                    print("  Auto-cycle step " + str(i + 1))
                    self.next()
                }
                return
            }
            
            emergency() {
                self.stop_traffic()
                -> $Offline
            }
            
            restart() {
                print("  [Red.restart] System is running - no restart needed")
                return
            }
        }
        
        $Green {
            $>() {
                print("  [Green.$enter] Light is now GREEN - GO")
                return
            }
            
            <$() {
                print("  [Green.$exit] Leaving Green state")
                return
            }
            
            next() {
                self.log_transition("Green", "Yellow")
                self.wait()
                -> $Yellow
            }
            
            # Can always go to emergency
            emergency() {
                self.stop_traffic()
                -> $Offline
            }
            
            get_status() {
                return "Cycle #" + str(self.cycle_count) + " - Timer: " + str(self.timer_seconds) + "s - State: GREEN"
            }
            
            set_timer(seconds) {
                self.timer_seconds = seconds
                print("    [Action] Timer set to " + str(seconds) + " seconds")
                return
            }
            
            auto_cycle() {
                print("  [Green.auto_cycle] Starting auto-cycle")
                for i in range(3) {
                    print("  Auto-cycle step " + str(i + 1))
                    self.next()
                }
                return
            }
            
            restart() {
                print("  [Green.restart] System is running - no restart needed")
                return
            }
        }
        
        $Yellow {
            $>() {
                print("  [Yellow.$enter] Light is now YELLOW - CAUTION")
                return
            }
            
            <$() {
                print("  [Yellow.$exit] Leaving Yellow state")
                return
            }
            
            next() {
                self.log_transition("Yellow", "Red")
                self.wait()
                -> $Red
            }
            
            # Can always go to emergency
            emergency() {
                self.stop_traffic()
                -> $Offline
            }
            
            get_status() {
                return "Cycle #" + str(self.cycle_count) + " - Timer: " + str(self.timer_seconds) + "s - State: YELLOW"
            }
            
            set_timer(seconds) {
                self.timer_seconds = seconds
                print("    [Action] Timer set to " + str(seconds) + " seconds")
                return
            }
            
            auto_cycle() {
                print("  [Yellow.auto_cycle] Starting auto-cycle")
                for i in range(3) {
                    print("  Auto-cycle step " + str(i + 1))
                    self.next()
                }
                return
            }
            
            restart() {
                print("  [Yellow.restart] System is running - no restart needed")
                return
            }
        }
        
        $Offline {
            $>() {
                print("  [Offline.$enter] Light is now OFFLINE - ALL LIGHTS FLASHING")
                return
            }
            
            <$() {
                print("  [Offline.$exit] Leaving Offline state, resuming normal operation")
                return
            }
            
            # Can only restart from offline
            restart() {
                print("  [Offline.restart] Restarting traffic light system")
                self.cycle_count = 0  # Reset cycle count on restart
                -> $Red
            }
            
            # Status still works in offline
            get_status() {
                return "SYSTEM OFFLINE - Emergency mode active. Call restart() to resume."
            }
            
            # These don't work in offline state
            next() {
                print("  [Offline.next] Cannot advance - system is offline")
                return
            }
            
            set_timer(seconds) {
                print("  [Offline.set_timer] Cannot set timer - system is offline")
                return
            }
            
            auto_cycle() {
                print("  [Offline.auto_cycle] Cannot auto-cycle - system is offline")
                return
            }
            
            emergency() {
                print("  [Offline.emergency] Already in emergency offline state")
                return
            }
        }
    
    # Actions - private helper methods
    actions:
        log_transition(from_state, to_state) {
            print("    [Action] Transitioning: " + from_state + " -> " + to_state)
            self.cycle_count = self.cycle_count + 1
        }
        
        wait() {
            print("    [Action] Waiting " + str(self.timer_seconds) + " seconds...")
            # In real system, would actually wait
        }
        
        stop_traffic() {
            print("    [Action] EMERGENCY - Stopping all traffic!")
        }
    
    # System variables (persist across states)
    domain:
        var cycle_count = 0
        var timer_seconds = 5
}