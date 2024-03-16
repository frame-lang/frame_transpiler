#Emitted from framec_v0.11.2



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class TransitionSm:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = TransitionSmCompartment('__transitionsm_state_S0', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = []
        
        # Initialize domain
        
        self.enters  = []
        self.exits  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def transit(self,):
        self.return_stack.append(None)
        __e = FrameEvent("transit",None)
        self.__kernel(__e)
        self.return_stack.pop(-1)
    
    def change(self,):
        self.return_stack.append(None)
        __e = FrameEvent("change",None)
        self.__kernel(__e)
        self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $S0
    
    def __transitionsm_state_S0(self, __e, compartment):
        if __e._message == "<":
            self.exit_do("S0")
            return
        elif __e._message == "transit":
            next_compartment = None
            next_compartment = TransitionSmCompartment('__transitionsm_state_S1', next_compartment)
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S1
    
    def __transitionsm_state_S1(self, __e, compartment):
        if __e._message == ">":
            self.enter_do("S1")
            return
        elif __e._message == "change":
            next_compartment = None
            next_compartment = TransitionSmCompartment('__transitionsm_state_S2', next_compartment)
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S2
    
    def __transitionsm_state_S2(self, __e, compartment):
        if __e._message == "<":
            self.exit_do("S2")
            return
        elif __e._message == "transit":
            next_compartment = None
            next_compartment = TransitionSmCompartment('__transitionsm_state_S3', next_compartment)
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S3
    
    def __transitionsm_state_S3(self, __e, compartment):
        if __e._message == ">":
            self.enter_do("S3")
            return
        elif __e._message == "<":
            self.exit_do("S3")
            return
        elif __e._message == "transit":
            next_compartment = None
            next_compartment = TransitionSmCompartment('__transitionsm_state_S4', next_compartment)
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S4
    
    def __transitionsm_state_S4(self, __e, compartment):
        if __e._message == ">":
            self.enter_do("S4")
            next_compartment = None
            next_compartment = TransitionSmCompartment('__transitionsm_state_S0', next_compartment)
            self.__transition(next_compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def enter_do(self,state: str):
        raise NotImplementedError
    
    def exit_do(self,state: str):
        raise NotImplementedError
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent( "<", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent(">", self.__compartment.enter_args))
            else: # there is a forwarded event
                if next_compartment.forward_event._message == ">":
                    # forwarded event is enter event
                    self.__router(next_compartment.forward_event)
                else:
                    # forwarded event is not enter event
                    # send normal enter event
                    self.__router(FrameEvent(">", self.__compartment.enter_args))
                    # and now forward event to new, intialized state
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
                
    
    def __router(self, __e):
        if self.__compartment.state == '__transitionsm_state_S0':
            self.__transitionsm_state_S0(__e, self.__compartment)
        elif self.__compartment.state == '__transitionsm_state_S1':
            self.__transitionsm_state_S1(__e, self.__compartment)
        elif self.__compartment.state == '__transitionsm_state_S2':
            self.__transitionsm_state_S2(__e, self.__compartment)
        elif self.__compartment.state == '__transitionsm_state_S3':
            self.__transitionsm_state_S3(__e, self.__compartment)
        elif self.__compartment.state == '__transitionsm_state_S4':
            self.__transitionsm_state_S4(__e, self.__compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class TransitionSmCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    