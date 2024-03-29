#Emitted from framec_v0.11.0



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
        
        self.__compartment: 'StateVarsCompartment' = StateVarsCompartment('__statevars_state_Init')
        self.__next_compartment: 'StateVarsCompartment' = None
        
        # Initialize domain
        
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def X(self,):
        __e = FrameEvent("X",None)
        self.__kernel(__e)
    
    def Y(self,):
        __e = FrameEvent("Y",None)
        self.__kernel(__e)
    
    def Z(self,):
        __e = FrameEvent("Z",None)
        self.__kernel(__e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __statevars_state_Init(self, __e):
        if __e._message == ">":
            next_compartment = StateVarsCompartment('__statevars_state_A')
            next_compartment.state_vars["x"] = 0
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $A
    
    def __statevars_state_A(self, __e):
        if __e._message == "X":
            (self.__compartment.state_vars["x"]) = self.__compartment.state_vars["x"] + 1
            return
        elif __e._message == "Y":
            next_compartment = StateVarsCompartment('__statevars_state_B')
            next_compartment.state_vars["y"] = 10
            next_compartment.state_vars["z"] = 100
            self.__transition(next_compartment)
            return
        elif __e._message == "Z":
            next_compartment = StateVarsCompartment('__statevars_state_B')
            next_compartment.state_vars["y"] = 10
            next_compartment.state_vars["z"] = 100
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $B
    
    def __statevars_state_B(self, __e):
        if __e._message == "X":
            next_compartment = StateVarsCompartment('__statevars_state_A')
            next_compartment.state_vars["x"] = 0
            self.__transition(next_compartment)
            return
        elif __e._message == "Y":
            (self.__compartment.state_vars["y"]) = self.__compartment.state_vars["y"] + 1
            return
        elif __e._message == "Z":
            (self.__compartment.state_vars["z"]) = self.__compartment.state_vars["z"] + 1
            return
    
    # ===================== Actions Block =================== #
    
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
        if self.__compartment.state == '__statevars_state_Init':
            self.__statevars_state_Init(__e)
        elif self.__compartment.state == '__statevars_state_A':
            self.__statevars_state_A(__e)
        elif self.__compartment.state == '__statevars_state_B':
            self.__statevars_state_B(__e)
        
    def __transition(self, next_compartment: 'StateVarsCompartment'):
        self.__next_compartment = next_compartment
    
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
    