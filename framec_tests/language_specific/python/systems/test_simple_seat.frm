@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Simple seat booking test

fn main() {
    print("=== Simple Seat Test ===")
    seat = SeatSystem()
    seat.select_seat("A1")
}

system SeatSystem {
    interface:
        select_seat(seat_id)
        
    machine:
        $Idle {
            select_seat(seat_id) {
                print("Selected seat: " + seat_id)
                -> $SeatSelected
            }
        }
        
        $SeatSelected {
            $>() {
                print("Seat is now selected")
            }
        }
        
    domain:
        current_seat:string = ""
}
