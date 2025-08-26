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

class DomainTypedTest:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__domaintypedtest_state_Ready', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Ready
    
    def __domaintypedtest_state_Ready(self, __e, compartment):
        if __e._message == "displayName":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("My name is " + self.name)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sReady(self, __e):
        return self.__domaintypedtest_state_Ready(__e, None)
    # ===================== Actions Block =================== #
    
    def printName_do(self):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("My name is " + self.name)
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
        if target_compartment.state == '__domaintypedtest_state_Ready':
            self.__domaintypedtest_state_Ready(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

