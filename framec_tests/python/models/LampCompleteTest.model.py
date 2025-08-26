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


def main():
    lamp = Lamp()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    lamp.turnOn()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    lamp.turnOff()
    return
class Lamp:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__lamp_state_Off', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def turnOn(self,):
        self.return_stack.append(None)
        __e = FrameEvent("turnOn",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def turnOff(self,):
        self.return_stack.append(None)
        __e = FrameEvent("turnOff",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Off
    
    def __lamp_state_Off(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Entering $Off")
            return
        elif __e._message == "<$":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Exiting $Off")
            return
        elif __e._message == "turnOn":# DEBUG: TransitionStmt
            
            next_compartment = None
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $On
    
    def __lamp_state_On(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Entering $On")
            return
        elif __e._message == "<$":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Exiting $On")
            return
        elif __e._message == "turnOff":# DEBUG: TransitionStmt
            
            next_compartment = None
            self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sOff(self, __e):
        return self.__lamp_state_Off(__e, None)
    def _sOn(self, __e):
        return self.__lamp_state_On(__e, None)
    
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
        if target_compartment.state == '__lamp_state_Off':
            self.__lamp_state_Off(__e, target_compartment)
        elif target_compartment.state == '__lamp_state_On':
            self.__lamp_state_On(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
