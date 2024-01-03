



from framelang.framelang import FrameEvent


# Emitted from framec_v0.11.0

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class StateVars:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__compartment: 'StateVarsCompartment' = StateVarsCompartment('__statevars_state_Init')
        self.__next_compartment: 'StateVarsCompartment' = None
        
        # Initialize domain
        
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def X(self,):
        e = FrameEvent("X",None)
        self.__kernel(e)
    
    def Y(self,):
        e = FrameEvent("Y",None)
        self.__kernel(e)
    
    def Z(self,):
        e = FrameEvent("Z",None)
        self.__kernel(e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __statevars_state_Init(self, e):
        if e._message == ">":
            compartment = StateVarsCompartment('__statevars_state_A')
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $A
    
    def __statevars_state_A(self, e):
        if e._message == "X":
            (self.__compartment.state_vars["x"]) = self.__compartment.state_vars["x"] + 1
            return
        elif e._message == "Y":
            compartment = StateVarsCompartment('__statevars_state_B')
            compartment.state_vars["y"] = 10
            compartment.state_vars["z"] = 100
            self.__transition(compartment)
            return
        elif e._message == "Z":
            compartment = StateVarsCompartment('__statevars_state_B')
            compartment.state_vars["y"] = 10
            compartment.state_vars["z"] = 100
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $B
    
    def __statevars_state_B(self, e):
        if e._message == "X":
            compartment = StateVarsCompartment('__statevars_state_A')
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
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, e):
        
        # send event to current state
        self.__router(e)
        
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
                
    
    def __router(self, e):
        if self.__compartment.state == '__statevars_state_Init':
            self.__statevars_state_Init(e)
        elif self.__compartment.state == '__statevars_state_A':
            self.__statevars_state_A(e)
        elif self.__compartment.state == '__statevars_state_B':
            self.__statevars_state_B(e)
        
    def __transition(self, compartment: 'StateVarsCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state
        
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
    