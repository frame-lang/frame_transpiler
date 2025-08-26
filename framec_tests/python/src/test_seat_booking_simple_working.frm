// Simple seat booking test to verify scope bug fix

fn main() {
    print("=== Simple Seat Booking Test ===")
    
    // Test the scope bug fix: obj.method() should work
    var seat_manager = SeatManager()
    seat_manager.test_operations()
    
    print("=== Test Complete ===")
}

system SeatManager {
    operations:
        test_operations() {
            print("Testing operations...")
            create_seat("A1")
            process_booking("A1", "user123")
        }
        
        create_seat(seat_id) {
            print("Creating seat: " + seat_id)
        }
        
        process_booking(seat_id, user_id) {
            print("Processing booking for seat: " + seat_id + " user: " + user_id)
        }
        
    machine:
        $Start {
            $>() {
                print("Seat manager started")
            }
        }
}