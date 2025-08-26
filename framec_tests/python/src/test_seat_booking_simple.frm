// Simplified seat booking test

fn main() {
    print("=== CultureTicks Seat Booking Workflow Test ===")
    
    // Create seat management system  
    var seat_manager = SeatManager()
    
    // Test the scope fix
    seat_manager.test_seat_state_management()
    
    print("=== Workflow Test Complete ===")
}

system SeatManager {
    operations:
        test_seat_state_management() {
            print("### Phase 1: Core Business Rules - Seat State Management")
            print("Testing seat state transitions and business logic")
        }
        
    machine:
        $Idle {
            select_seat(seat_id) {
                print("Starting seat selection for: " + seat_id)
                -> $ProcessingSelection
            }
        }
        
        $ProcessingSelection {
            $>() {
                print("Processing selection for seat")
            }
        }
        
    domain:
        var current_seat_id:string = ""
}