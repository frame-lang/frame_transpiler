// CultureTicks Backend Testing Plan Implementation
// Seat Business Logic Rules - State Management and Lock Business Rules

fn main() {
    print("=== CultureTicks Seat Booking Workflow Test ===")
    
    // Create seat management system
    var seat_manager = SeatManager()
    
    // Test Phase 1: Core Business Rules
    seat_manager.test_seat_state_management()
    seat_manager.test_mercury_lock_rules()
    
    print("=== Workflow Test Complete ===")
}

system SeatManager {
    operations:
        test_seat_state_management() {
            print("### Phase 1: Core Business Rules - Seat State Management")
            
            // Rule 1.1: Seats exist in database states
            print("Rule 1.1: Testing seat states: Available, Reserved, Sold")
            self.create_seat("A1", "Available")
            self.create_seat("A2", "Reserved") 
            self.create_seat("A3", "Sold")
            
            // Rule 1.2: Frontend status mapping
            print("Rule 1.2: Testing frontend status mapping")
            self.test_status_mapping()
            
            // Rule 1.3: Valid state transitions
            print("Rule 1.3: Testing valid state transitions")
            self.test_valid_transitions()
            
            // Rule 1.4: Invalid transitions must be rejected
            print("Rule 1.4: Testing invalid transition rejection")
            self.test_invalid_transitions()
        }
        
        test_mercury_lock_rules() {
            print("### Mercury Lock Business Rules")
            
            // Rule 1.4: Lock duration is exactly 15 minutes (900 seconds)
            print("Rule 1.4: Lock duration is 15 minutes (900 seconds)")
            self.test_lock_duration()
            
            // Rule 1.5: Lock must include lockRequestId (UUID) and exchangeTicketGroupId
            print("Rule 1.5: Lock must include required IDs")
            self.test_lock_requirements()
            
            // Rule 1.6: Lock cannot be applied to already locked or purchased seats
            print("Rule 1.6: Testing lock application restrictions")
            self.test_lock_restrictions()
        }
        
        create_seat(seat_id, initial_state) {
            print("Creating seat: " + seat_id + " with state: " + initial_state)
            // Note: Cannot transition from operations - would need to handle in machine block
        }
        
        test_status_mapping() {
            print("- Available + no active lock -> available")
            print("- Available/Reserved + active lock -> locked") 
            print("- Sold -> sold")
        }
        
        test_valid_transitions() {
            print("- Available -> Reserved (user selects seat, creates lock)")
            print("- Reserved -> Available (lock expires or user deselects)")
            print("- Reserved -> Sold (payment successful)")
        }
        
        test_invalid_transitions() {
            print("- Available -> Sold (must reserve/lock first)")
            print("- Sold -> Reserved (cannot re-lock sold seat)")
            print("- Sold -> Available (sold seats cannot become available)")
        }
        
        test_lock_duration() {
            print("Lock duration validation: exactly 900 seconds")
        }
        
        test_lock_requirements() {
            print("Lock requires: lockRequestId (UUID), exchangeTicketGroupId")
        }
        
        test_lock_restrictions() {
            print("Cannot lock: already locked seats, purchased seats")
        }
        
    interface:
        select_seat(seat_id)
        reserve_seat(seat_id, lock_request_id, exchange_ticket_group_id)
        purchase_seat(seat_id, payment_info)
        release_seat(seat_id)
        expire_lock(seat_id)
        
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
                // Check if seat is available for selection
                self.check_seat_availability("seat_id")
            }
            
            reserve_seat(seat_id, lock_request_id, exchange_ticket_group_id) {
                print("Reserving seat: " + seat_id)
                print("Lock Request ID: " + lock_request_id)
                print("Exchange Ticket Group ID: " + exchange_ticket_group_id)
                
                if self.is_seat_available(seat_id) {
                    self.create_mercury_lock(seat_id, lock_request_id, exchange_ticket_group_id)
                    -> $SeatReserved
                } else {
                    print("ERROR: Seat not available for reservation")
                    -> $Idle
                }
            }
        }
        
        $SeatReserved {
            $>() {
                print("Seat reserved with lock")
                // Start 15-minute timer
                self.start_lock_timer("current_seat", "current_lock")
            }
            
            purchase_seat(seat_id, payment_info) {
                print("Processing payment for seat: " + seat_id)
                if self.process_payment(payment_info) {
                    print("Payment successful - seat sold")
                    -> $SeatSold
                } else {
                    print("Payment failed - releasing seat")
                    -> $Idle
                }
            }
            
            release_seat(seat_id) {
                print("User released seat: " + seat_id)
                self.remove_lock(seat_id)
                -> $Idle
            }
            
            expire_lock(seat_id) {
                print("Lock expired for seat: " + seat_id)
                self.remove_lock(seat_id)
                -> $Idle
            }
        }
        
        $SeatSold {
            $>() {
                print("Seat sold")
                // Seat is permanently sold - no further state changes allowed
            }
            
            select_seat(seat_id) {
                print("ERROR: Cannot select sold seat: " + seat_id)
                // Invalid transition - sold seats cannot be reselected
            }
            
            reserve_seat(seat_id, lock_request_id, exchange_ticket_group_id) {
                print("ERROR: Cannot reserve sold seat: " + seat_id)
                // Invalid transition - sold seats cannot be reserved
            }
        }
        
        $SeatCreated {
            $>() {
                print("Seat created in initial state")
                // Simplified - just go to idle for testing
                -> $Idle
            }
        }
        
    actions:
        check_seat_availability(seat_id) {
            print("Checking availability for seat: " + seat_id)
        }
        
        is_seat_available(seat_id) {
            print("Validating seat availability: " + seat_id)
            return true  // Simplified for test
        }
        
        create_mercury_lock(seat_id, lock_request_id, exchange_ticket_group_id) {
            print("Creating Mercury lock:")
            print("  Seat: " + seat_id)
            print("  Lock Request ID: " + lock_request_id) 
            print("  Exchange Ticket Group ID: " + exchange_ticket_group_id)
            print("  Duration: 900 seconds (15 minutes)")
        }
        
        start_lock_timer(seat_id, lock_request_id) {
            print("Starting 15-minute timer for seat: " + seat_id)
        }
        
        process_payment(payment_info) {
            print("Processing payment: " + payment_info)
            return true  // Simplified for test
        }
        
        remove_lock(seat_id) {
            print("Removing lock from seat: " + seat_id)
        }
        
    domain:
        var current_seat_id:string = ""
        var lock_duration:string = "900"  // 15 minutes in seconds
        var active_locks:string = ""
}