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
    
    print("=== Simple Seat Test ===")
    seat = SeatSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    seat.select_seat("A1")
    return
class SeatSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__seatsystem_state_Idle', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.current_seat = ""
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def select_seat(self,seat_id):
        parameters = {}
        parameters["seat_id"] = seat_id
        self.return_stack.append(None)
        __e = FrameEvent("select_seat",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Idle
    
    def __seatsystem_state_Idle(self, __e, compartment):
        if __e._message == "select_seat":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Selected seat: " + __e._parameters["seat_id"])# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__seatsystem_state_SeatSelected', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $SeatSelected
    
    def __seatsystem_state_SeatSelected(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Seat is now selected")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sIdle(self, __e):
        return self.__seatsystem_state_Idle(__e, None)
    def _sSeatSelected(self, __e):
        return self.__seatsystem_state_SeatSelected(__e, None)
    
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
        if target_compartment.state == '__seatsystem_state_Idle':
            self.__seatsystem_state_Idle(__e, target_compartment)
        elif target_compartment.state == '__seatsystem_state_SeatSelected':
            self.__seatsystem_state_SeatSelected(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()