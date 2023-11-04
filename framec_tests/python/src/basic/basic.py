# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class Basic:
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__basic_state_S0
        self.__compartment: 'BasicCompartment' = BasicCompartment(self.__state)
        self.__next_compartment: 'BasicCompartment' = None
        
        # Initialize domain
        
        self.entry_log  = []
        self.exit_log  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def A(self,):
        e = FrameEvent("A",None)
        self.__mux(e)
    
    def B(self,):
        e = FrameEvent("B",None)
        self.__mux(e)
    
    # ===================== Machine Block =================== #
    
    def __basic_state_S0(self, e):
        if e._message == ">":
            self.entered_do("S0")
            return
        elif e._message == "<":
            self.left_do("S0")
            return
        elif e._message == "A":
            # ooh
            compartment = BasicCompartment(self.__basic_state_S1)
            self.__transition(compartment)
            return
    
    def __basic_state_S1(self, e):
        if e._message == ">":
            self.entered_do("S1")
            return
        elif e._message == "<":
            self.left_do("S1")
            return
        elif e._message == "B":
            # aah
            compartment = BasicCompartment(self.__basic_state_S0)
            self.__transition(compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def entered_do(self,msg: str):
        raise NotImplementedError
    def left_do(self,msg: str):
        raise NotImplementedError
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
        if self.__compartment.state.__name__ == '__basic_state_S0':
            self.__basic_state_S0(e)
        elif self.__compartment.state.__name__ == '__basic_state_S1':
            self.__basic_state_S1(e)
        
    def __transition(self, compartment: 'BasicCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'BasicCompartment'):
        self.__router(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__router(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class BasicCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    


# ********************

#class BasicController(Basic):
	#def __init__(self,):
	    #super().__init__()

    #def entered_do(self,msg: str):
        #pass

    #def left_do(self,msg: str):
        #pass

# ********************

