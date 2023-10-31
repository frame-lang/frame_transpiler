# emitted from framec_v0.11.0
# get include files at https://github.com/frame-lang/frame-ancillary-files


from framelang.framelang import FrameEvent




class SimpleHandlerCalls:
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__simplehandlercalls_state_Init
        self.__compartment: 'SimpleHandlerCallsCompartment' = SimpleHandlerCallsCompartment(self.__state)
        self.__next_compartment: 'SimpleHandlerCallsCompartment' = None
        
        # Initialize domain
        
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
    
    def C(self,):
        e = FrameEvent("C",None)
        self.__mux(e)
    
    def D(self,):
        e = FrameEvent("D",None)
        self.__mux(e)
    
    def E(self,):
        e = FrameEvent("E",None)
        self.__mux(e)
    
    # ===================== Machine Block =================== #
    
    def __simplehandlercalls_state_Init(self, e):
        if e._message == "A":
            compartment = SimpleHandlerCallsCompartment(self.__simplehandlercalls_state_A)
            self.__transition(compartment)
            
            return
        
        elif e._message == "B":
            compartment = SimpleHandlerCallsCompartment(self.__simplehandlercalls_state_B)
            self.__transition(compartment)
            
            return
        
        elif e._message == "C":
            self.A()
            return
            
            return
        
        elif e._message == "D":
            self.B()
            return
            compartment = SimpleHandlerCallsCompartment(self.__simplehandlercalls_state_A)
            self.__transition(compartment)
            
            return
        
        elif e._message == "E":
            self.D()
            return
            self.C()
            return
            
            return
        
    def __simplehandlercalls_state_A(self, e):
        pass
        
    def __simplehandlercalls_state_B(self, e):
        pass
        
    
    
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__simplehandlercalls_state_Init':
            self.__simplehandlercalls_state_Init(e)
        elif self.__compartment.state.__name__ == '__simplehandlercalls_state_A':
            self.__simplehandlercalls_state_A(e)
        elif self.__compartment.state.__name__ == '__simplehandlercalls_state_B':
            self.__simplehandlercalls_state_B(e)
        
        if self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            if(next_compartment.forward_event is not None and 
               next_compartment.forward_event._message == ">"):
                self.__mux(FrameEvent( "<", self.__compartment.exit_args))
                self.__compartment = next_compartment
                self.__mux(next_compartment.forward_event)
            else:
                self.__do_transition(next_compartment)
                if next_compartment.forward_event is not None:
                    self.__mux(next_compartment.forward_event)
            next_compartment.forward_event = None
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'SimpleHandlerCallsCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'SimpleHandlerCallsCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class SimpleHandlerCallsCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class SimpleHandlerCallsController(SimpleHandlerCalls):
	#def __init__(self,):
	    #super().__init__()

# ********************

