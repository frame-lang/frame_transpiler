# Emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class TransitionSm:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__compartment: 'TransitionSmCompartment' = TransitionSmCompartment('__transitionsm_state_S0')
        self.__next_compartment: 'TransitionSmCompartment' = None
        
        # Initialize domain
        
        self.enters  = []
        self.exits  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def transit(self,):
        __e = FrameEvent("transit",None)
        self.__kernel(__e)
    
    def change(self,):
        __e = FrameEvent("change",None)
        self.__kernel(__e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $S0
    
    def __transitionsm_state_S0(self, __e):
        if __e._message == "<":
            self.exit_do("S0")
            return
        elif __e._message == "transit":
            next_compartment = TransitionSmCompartment('__transitionsm_state_S1')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S1
    
    def __transitionsm_state_S1(self, __e):
        if __e._message == ">":
            self.enter_do("S1")
            return
        elif __e._message == "change":
            compartment = TransitionSmCompartment('__transitionsm_state_S2')
            
            self.__change_state(compartment)
            return
    
    # ----------------------------------------
    # $S2
    
    def __transitionsm_state_S2(self, __e):
        if __e._message == "<":
            self.exit_do("S2")
            return
        elif __e._message == "transit":
            next_compartment = TransitionSmCompartment('__transitionsm_state_S3')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S3
    
    def __transitionsm_state_S3(self, __e):
        if __e._message == ">":
            self.enter_do("S3")
            return
        elif __e._message == "<":
            self.exit_do("S3")
            return
        elif __e._message == "transit":
            next_compartment = TransitionSmCompartment('__transitionsm_state_S4')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S4
    
    def __transitionsm_state_S4(self, __e):
        if __e._message == ">":
            self.enter_do("S4")
            compartment = TransitionSmCompartment('__transitionsm_state_S0')
            
            self.__change_state(compartment)
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
            self.__transitionsm_state_S0(__e)
        elif self.__compartment.state == '__transitionsm_state_S1':
            self.__transitionsm_state_S1(__e)
        elif self.__compartment.state == '__transitionsm_state_S2':
            self.__transitionsm_state_S2(__e)
        elif self.__compartment.state == '__transitionsm_state_S3':
            self.__transitionsm_state_S3(__e)
        elif self.__compartment.state == '__transitionsm_state_S4':
            self.__transitionsm_state_S4(__e)
        
    def __transition(self, next_compartment: 'TransitionSmCompartment'):
        self.__next_compartment = next_compartment
    
    def __change_state(self, new_compartment: 'TransitionSmCompartment'):
        self.__compartment = new_compartment
    
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class TransitionSmCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    