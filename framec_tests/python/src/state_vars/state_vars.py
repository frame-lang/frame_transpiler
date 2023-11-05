# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class StateVars:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__statevars_state_Init
        self.__compartment: 'StateVarsCompartment' = StateVarsCompartment(self.__state)
        self.__next_compartment: 'StateVarsCompartment' = None
        
        # Initialize domain
        
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def X(self,):
        e = FrameEvent("X",None)
        self.__mux(e)
    
    def Y(self,):
        e = FrameEvent("Y",None)
        self.__mux(e)
    
    def Z(self,):
        e = FrameEvent("Z",None)
        self.__mux(e)
    
    # ===================== Machine Block =================== #
    
    def __statevars_state_Init(self, e):
        if e._message == ">":
            compartment = StateVarsCompartment(self.__statevars_state_A)
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            return
    
    def __statevars_state_A(self, e):
        if e._message == "X":
            (self.__compartment.state_vars["x"]) = self.__compartment.state_vars["x"] + 1
            return
        elif e._message == "Y":
            compartment = StateVarsCompartment(self.__statevars_state_B)
            compartment.state_vars["y"] = 10
            compartment.state_vars["z"] = 100
            self.__transition(compartment)
            return
        elif e._message == "Z":
            compartment = StateVarsCompartment(self.__statevars_state_B)
            compartment.state_vars["y"] = 10
            compartment.state_vars["z"] = 100
            self.__transition(compartment)
            return
    
    def __statevars_state_B(self, e):
        if e._message == "X":
            compartment = StateVarsCompartment(self.__statevars_state_A)
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            return
        elif e._message == "Y":
            (self.__compartment.state_vars["y"]) = self.__compartment.state_vars["y"] + 1
            return
        elif e._message == "Z":
            (self.__compartment.state_vars["z"]) = self.__compartment.state_vars["z"] + 1
            return
    
    # ===================== Actions Block =================== #
    
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
        if self.__compartment.state.__name__ == '__statevars_state_Init':
            self.__statevars_state_Init(e)
        elif self.__compartment.state.__name__ == '__statevars_state_A':
            self.__statevars_state_A(e)
        elif self.__compartment.state.__name__ == '__statevars_state_B':
            self.__statevars_state_B(e)
        
    def __transition(self, compartment: 'StateVarsCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'StateVarsCompartment'):
        self.__router(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__router(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        
    def compartment_info(self):
        return self.__compartment
        

# ===================== Compartment =================== #

class StateVarsCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    


# ********************

#class StateVarsController(StateVars):
	#def __init__(self,):
	    #super().__init__()

# ********************

