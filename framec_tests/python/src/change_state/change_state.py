# emitted from framec_v0.11.0


class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class ChangeStateSm:
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__changestatesm_state_S0
        self.__compartment: 'ChangeStateSmCompartment' = ChangeStateSmCompartment(self.__state)
        self.__next_compartment: 'ChangeStateSmCompartment' = None
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def change(self,):
        e = FrameEvent("change",None)
        self.__mux(e)
    
    # ===================== Machine Block =================== #
    
    def __changestatesm_state_S0(self, e):
        if e._message == "change":
            compartment = ChangeStateSmCompartment(self.__changestatesm_state_S1)
            
            self.__change_state(compartment)
            return
    
    def __changestatesm_state_S1(self, e):
        if e._message == "change":
            compartment = ChangeStateSmCompartment(self.__changestatesm_state_S2)
            
            self.__change_state(compartment)
            return
    
    def __changestatesm_state_S2(self, e):
        if e._message == "change":
            compartment = ChangeStateSmCompartment(self.__changestatesm_state_S3)
            
            self.__change_state(compartment)
            return
    
    def __changestatesm_state_S3(self, e):
        if e._message == "change":
            compartment = ChangeStateSmCompartment(self.__changestatesm_state_S4)
            
            self.__change_state(compartment)
            return
    
    def __changestatesm_state_S4(self, e):
        if e._message == "change":
            compartment = ChangeStateSmCompartment(self.__changestatesm_state_S0)
            
            self.__change_state(compartment)
            return
    
    # =============== Machinery and Mechanisms ============== #
    
    def __mux(self, e):
        
        self.__router(e)
        
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            if(next_compartment.forward_event is not None and 
               next_compartment.forward_event._message == ">"):
                self.__router(FrameEvent( "<", self.__compartment.exit_args))
                self.__compartment = next_compartment
                self.__router(next_compartment.forward_event)
            else:
                self.__do_transition(next_compartment)
                if next_compartment.forward_event is not None:
                    self.__router(next_compartment.forward_event)
            next_compartment.forward_event = None
    
    def __router(self, e):
        if self.__compartment.state.__name__ == '__changestatesm_state_S0':
            self.__changestatesm_state_S0(e)
        elif self.__compartment.state.__name__ == '__changestatesm_state_S1':
            self.__changestatesm_state_S1(e)
        elif self.__compartment.state.__name__ == '__changestatesm_state_S2':
            self.__changestatesm_state_S2(e)
        elif self.__compartment.state.__name__ == '__changestatesm_state_S3':
            self.__changestatesm_state_S3(e)
        elif self.__compartment.state.__name__ == '__changestatesm_state_S4':
            self.__changestatesm_state_S4(e)
        
    def __transition(self, compartment: 'ChangeStateSmCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'ChangeStateSmCompartment'):
        self.__router(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__router(FrameEvent(">", self.__compartment.enter_args))
    
    def __change_state(self, new_compartment: 'ChangeStateSmCompartment'):
        self.__compartment = new_compartment
    
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class ChangeStateSmCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    


# ********************

#class ChangeStateSmController(ChangeStateSm):
	#def __init__(self,):
	    #super().__init__()

# ********************

