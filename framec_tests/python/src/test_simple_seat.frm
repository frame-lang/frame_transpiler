// Simple seat booking test

fn main() {
    print("=== Simple Seat Test ===")
    var seat = SeatSystem()
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
        var current_seat:string = ""
}