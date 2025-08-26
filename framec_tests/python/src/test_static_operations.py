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

class StaticTest:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__statictest_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    @staticmethod
    def static_helper():# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Static: static_helper called")
    
    def instance_helper(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Instance: instance_helper called")
    
    @staticmethod
    def static_with_param(msg):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Static: " + msg)
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __statictest_state_Start(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Machine: testing static vs instance calls")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            self.instance_helper()# DEBUG_EXPR_TYPE: Discriminant(4)
            
            StaticTest# DEBUG_EXPR_TYPE: Discriminant(4)
            
            StaticTest# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Machine: all operation tests complete")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__statictest_state_Start(__e, None)
    
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
        if target_compartment.state == '__statictest_state_Start':
            self.__statictest_state_Start(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

