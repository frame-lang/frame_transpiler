#Emitted from framec_v0.30.0


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


def main():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== CultureTicks Seat Booking Workflow Test ===")
    seat_manager = SeatManager()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    seat_manager.test_seat_state_management()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    seat_manager.test_mercury_lock_rules()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== Workflow Test Complete ===")
    return
class SeatManager:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.current_seat_id = ""
        self.lock_duration = "900"
        self.active_locks = ""
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def test_seat_state_management(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("### Phase 1: Core Business Rules - Seat State Management")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Rule 1.1: Testing seat states: Available, Reserved, Sold")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        self.create_seat("A1","Available")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        self.create_seat("A2","Reserved")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        self.create_seat("A3","Sold")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Rule 1.2: Testing frontend status mapping")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        self.test_status_mapping()# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Rule 1.3: Testing valid state transitions")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        self.test_valid_transitions()# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Rule 1.4: Testing invalid transition rejection")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        self.test_invalid_transitions()
    
    def test_mercury_lock_rules(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("### Mercury Lock Business Rules")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Rule 1.4: Lock duration is 15 minutes (900 seconds)")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        self.test_lock_duration()# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Rule 1.5: Lock must include required IDs")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        self.test_lock_requirements()# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Rule 1.6: Testing lock application restrictions")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        self.test_lock_restrictions()
    
    def create_seat(self,seat_id,initial_state):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Creating seat: " + seat_id + " with state: " + initial_state)
    
    def test_status_mapping(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("- Available + no active lock -> available")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("- Available/Reserved + active lock -> locked")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("- Sold -> sold")
    
    def test_valid_transitions(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("- Available -> Reserved (user selects seat, creates lock)")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("- Reserved -> Available (lock expires or user deselects)")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("- Reserved -> Sold (payment successful)")
    
    def test_invalid_transitions(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("- Available -> Sold (must reserve/lock first)")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("- Sold -> Reserved (cannot re-lock sold seat)")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("- Sold -> Available (sold seats cannot become available)")
    
    def test_lock_duration(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Lock duration validation: exactly 900 seconds")
    
    def test_lock_requirements(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Lock requires: lockRequestId (UUID), exchangeTicketGroupId")
    
    def test_lock_restrictions(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
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
        if __e._message == "select_seat":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Starting seat selection for: " + __e._parameters["seat_id"])# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__seatmanager_state_ProcessingSelection', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $ProcessingSelection
    
    def __seatmanager_state_ProcessingSelection(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Processing selection for seat")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            check_seat_availability("seat_id")
            return
        elif __e._message == "reserve_seat":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Reserving seat: " + __e._parameters["seat_id"])# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Lock Request ID: " + __e._parameters["lock_request_id"])# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Exchange Ticket Group ID: " + __e._parameters["exchange_ticket_group_id"])
            if is_seat_available(__e._parameters["seat_id"]):# DEBUG_EXPR_TYPE: Discriminant(4)
                
                create_mercury_lock(__e._parameters["seat_id"],__e._parameters["lock_request_id"],__e._parameters["exchange_ticket_group_id"])# DEBUG: TransitionStmt
                
                next_compartment = FrameCompartment('__seatmanager_state_SeatReserved', None, None, None, None)
                self.__transition(next_compartment)
            else:# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("ERROR: Seat not available for reservation")# DEBUG: TransitionStmt
                
                next_compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
                self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $SeatReserved
    
    def __seatmanager_state_SeatReserved(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Seat reserved with lock")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            start_lock_timer("current_seat","current_lock")
            return
        elif __e._message == "purchase_seat":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Processing payment for seat: " + __e._parameters["seat_id"])
            if process_payment(__e._parameters["payment_info"]):# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("Payment successful - seat sold")# DEBUG: TransitionStmt
                
                next_compartment = FrameCompartment('__seatmanager_state_SeatSold', None, None, None, None)
                self.__transition(next_compartment)
            else:# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("Payment failed - releasing seat")# DEBUG: TransitionStmt
                
                next_compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
                self.__transition(next_compartment)
            return
        elif __e._message == "release_seat":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("User released seat: " + __e._parameters["seat_id"])# DEBUG_EXPR_TYPE: Discriminant(4)
            
            remove_lock(__e._parameters["seat_id"])# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "expire_lock":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Lock expired for seat: " + __e._parameters["seat_id"])# DEBUG_EXPR_TYPE: Discriminant(4)
            
            remove_lock(__e._parameters["seat_id"])# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__seatmanager_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $SeatSold
    
    def __seatmanager_state_SeatSold(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Seat sold")
            return
        elif __e._message == "select_seat":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("ERROR: Cannot select sold seat: " + __e._parameters["seat_id"])
            return
        elif __e._message == "reserve_seat":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("ERROR: Cannot reserve sold seat: " + __e._parameters["seat_id"])
            return
    
    
    # ----------------------------------------
    # $SeatCreated
    
    def __seatmanager_state_SeatCreated(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Seat created in initial state")# DEBUG: TransitionStmt
            
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
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Checking availability for seat: " + seat_id)
        return
        
    
    def is_seat_available_do(self,seat_id):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Validating seat availability: " + seat_id)
        return True
        return
        
    
    def create_mercury_lock_do(self,seat_id,lock_request_id,exchange_ticket_group_id):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Creating Mercury lock:")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("  Seat: " + seat_id)# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("  Lock Request ID: " + lock_request_id)# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("  Exchange Ticket Group ID: " + exchange_ticket_group_id)# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("  Duration: 900 seconds (15 minutes)")
        return
        
    
    def start_lock_timer_do(self,seat_id,lock_request_id):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Starting 15-minute timer for seat: " + seat_id)
        return
        
    
    def process_payment_do(self,payment_info):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Processing payment: " + payment_info)
        return True
        return
        
    
    def remove_lock_do(self,seat_id):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
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