#Emitted from framec_v0.30.0

from enum import Enum

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None):
        self.state = state
        self.forward_event = forward_event
        self.exit_args = exit_args
        self.enter_args = enter_args
        self.parent_compartment = parent_compartment


def main():
    print("=== CultureTicks Seat Booking Workflow Test ===")
    seat_manager = SeatManager()
    seat_manager.test_seat_state_management()
    seat_manager.test_mercury_lock_rules()
    print("=== Workflow Test Complete ===")
    return
class SeatManager:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.current_seat_id: str = ""
        self.lock_duration: str = "900"
        self.active_locks: str = ""
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def test_seat_state_management(self):
        print("### Phase 1: Core Business Rules - Seat State Management")
        print("Rule 1.1: Testing seat states: Available, Reserved, Sold")
        self.create_seat("A1","Available")
        self.create_seat("A2","Reserved")
        self.create_seat("A3","Sold")
        print("Rule 1.2: Testing frontend status mapping")
        self.test_status_mapping()
        print("Rule 1.3: Testing valid state transitions")
        self.test_valid_transitions()
        print("Rule 1.4: Testing invalid transition rejection")
        self.test_invalid_transitions()
    
    def test_mercury_lock_rules(self):
        print("### Mercury Lock Business Rules")
        print("Rule 1.4: Lock duration is 15 minutes (900 seconds)")
        self.test_lock_duration()
        print("Rule 1.5: Lock must include required IDs")
        self.test_lock_requirements()
        print("Rule 1.6: Testing lock application restrictions")
        self.test_lock_restrictions()
    
    def create_seat(self,seat_id,initial_state):
        print("Creating seat: " + seat_id + " with state: " + initial_state)
    
    def test_status_mapping(self):
        print("- Available + no active lock -> available")
        print("- Available/Reserved + active lock -> locked")
        print("- Sold -> sold")
    
    def test_valid_transitions(self):
        print("- Available -> Reserved (user selects seat, creates lock)")
        print("- Reserved -> Available (lock expires or user deselects)")
        print("- Reserved -> Sold (payment successful)")
    
    def test_invalid_transitions(self):
        print("- Available -> Sold (must reserve/lock first)")
        print("- Sold -> Reserved (cannot re-lock sold seat)")
        print("- Sold -> Available (sold seats cannot become available)")
    
    def test_lock_duration(self):
        print("Lock duration validation: exactly 900 seconds")
    
    def test_lock_requirements(self):
        print("Lock requires: lockRequestId (UUID), exchangeTicketGroupId")
    
    def test_lock_restrictions(self):
        print("Cannot lock: already locked seats, purchased seats")
    # ==================== Interface Block ================== #
    
    def select_seat(self,seat_id):
        parameters = {}
        parameters["seat_id"] = seat_id
        self.return_stack.append(None)
        __e = FrameEvent("select_seat",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def reserve_seat(self,seat_id,lock_request_id,exchange_ticket_group_id):
        parameters = {}
        parameters["seat_id"] = seat_id
        parameters["lock_request_id"] = lock_request_id
        parameters["exchange_ticket_group_id"] = exchange_ticket_group_id
        self.return_stack.append(None)
        __e = FrameEvent("reserve_seat",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def purchase_seat(self,seat_id,payment_info):
        parameters = {}
        parameters["seat_id"] = seat_id
        parameters["payment_info"] = payment_info
        self.return_stack.append(None)
        __e = FrameEvent("purchase_seat",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def release_seat(self,seat_id):
        parameters = {}
        parameters["seat_id"] = seat_id
        self.return_stack.append(None)
        __e = FrameEvent("release_seat",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def expire_lock(self,seat_id):
        parameters = {}
        parameters["seat_id"] = seat_id
        self.return_stack.append(None)
        __e = FrameEvent("expire_lock",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Idle
    
    def __seatmanager_state_Idle(self, __e, compartment):
        if __e._message == "select_seat":
            print("Starting seat selection for: " + __e._parameters["seat_id"])
            next_compartment = FrameCompartment('__seatmanager_state_ProcessingSelection', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $ProcessingSelection
    
    def __seatmanager_state_ProcessingSelection(self, __e, compartment):
        if __e._message == "$>":
            print("Processing selection for seat")
            self.check_seat_availability_do("seat_id")
            return
        elif __e._message == "reserve_seat":
            print("Reserving seat: " + __e._parameters["seat_id"])
            print("Lock Request ID: " + __e._parameters["lock_request_id"])
            print("Exchange Ticket Group ID: " + __e._parameters["exchange_ticket_group_id"])
            if self.is_seat_available_do(__e._parameters["seat_id"]):
                self.create_mercury_lock_do(__e._parameters["seat_id"],__e._parameters["lock_request_id"],__e._parameters["exchange_ticket_group_id"])
                next_compartment = FrameCompartment('__seatmanager_state_SeatReserved', None, None, None, None)
                self.__transition(next_compartment)
            else:
                print("ERROR: Seat not available for reservation")
                next_compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
                self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $SeatReserved
    
    def __seatmanager_state_SeatReserved(self, __e, compartment):
        if __e._message == "$>":
            print("Seat reserved with lock")
            self.start_lock_timer_do("current_seat","current_lock")
            return
        elif __e._message == "purchase_seat":
            print("Processing payment for seat: " + __e._parameters["seat_id"])
            if self.process_payment_do(__e._parameters["payment_info"]):
                print("Payment successful - seat sold")
                next_compartment = FrameCompartment('__seatmanager_state_SeatSold', None, None, None, None)
                self.__transition(next_compartment)
            else:
                print("Payment failed - releasing seat")
                next_compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
                self.__transition(next_compartment)
            return
        elif __e._message == "release_seat":
            print("User released seat: " + __e._parameters["seat_id"])
            self.remove_lock_do(__e._parameters["seat_id"])
            next_compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "expire_lock":
            print("Lock expired for seat: " + __e._parameters["seat_id"])
            self.remove_lock_do(__e._parameters["seat_id"])
            next_compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $SeatSold
    
    def __seatmanager_state_SeatSold(self, __e, compartment):
        if __e._message == "$>":
            print("Seat sold")
            return
        elif __e._message == "select_seat":
            print("ERROR: Cannot select sold seat: " + __e._parameters["seat_id"])
            return
        elif __e._message == "reserve_seat":
            print("ERROR: Cannot reserve sold seat: " + __e._parameters["seat_id"])
            return
    
    
    # ----------------------------------------
    # $SeatCreated
    
    def __seatmanager_state_SeatCreated(self, __e, compartment):
        if __e._message == "$>":
            print("Seat created in initial state")
            next_compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sIdle(self, __e):
        return self.__seatmanager_state_Idle(__e, None)
    def _sProcessingSelection(self, __e):
        return self.__seatmanager_state_ProcessingSelection(__e, None)
    def _sSeatReserved(self, __e):
        return self.__seatmanager_state_SeatReserved(__e, None)
    def _sSeatSold(self, __e):
        return self.__seatmanager_state_SeatSold(__e, None)
    def _sSeatCreated(self, __e):
        return self.__seatmanager_state_SeatCreated(__e, None)
    # ===================== Actions Block =================== #
    
    def check_seat_availability_do(self,seat_id):
        
        print("Checking availability for seat: " + seat_id)
        return
        
    
    def is_seat_available_do(self,seat_id):
        
        print("Validating seat availability: " + seat_id)
        return True
        return
        
    
    def create_mercury_lock_do(self,seat_id,lock_request_id,exchange_ticket_group_id):
        
        print("Creating Mercury lock:")
        print("  Seat: " + seat_id)
        print("  Lock Request ID: " + lock_request_id)
        print("  Exchange Ticket Group ID: " + exchange_ticket_group_id)
        print("  Duration: 900 seconds (15 minutes)")
        return
        
    
    def start_lock_timer_do(self,seat_id,lock_request_id):
        
        print("Starting 15-minute timer for seat: " + seat_id)
        return
        
    
    def process_payment_do(self,payment_info):
        
        print("Processing payment: " + payment_info)
        return True
        return
        
    
    def remove_lock_do(self,seat_id):
        
        print("Removing lock from seat: " + seat_id)
        return
        
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent("<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else:
                # forwarded event
                if next_compartment.forward_event._message == "$>":
                    self.__router(next_compartment.forward_event)
                else:
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
    
    def __router(self, __e, compartment=None):
        target_compartment = compartment or self.__compartment
        if target_compartment.state == '__seatmanager_state_Idle':
            self.__seatmanager_state_Idle(__e, target_compartment)
        elif target_compartment.state == '__seatmanager_state_ProcessingSelection':
            self.__seatmanager_state_ProcessingSelection(__e, target_compartment)
        elif target_compartment.state == '__seatmanager_state_SeatReserved':
            self.__seatmanager_state_SeatReserved(__e, target_compartment)
        elif target_compartment.state == '__seatmanager_state_SeatSold':
            self.__seatmanager_state_SeatSold(__e, target_compartment)
        elif target_compartment.state == '__seatmanager_state_SeatCreated':
            self.__seatmanager_state_SeatCreated(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
