# emitted from framec_v0.11.0
# get include files at https://github.com/frame-lang/frame-ancillary-files
from framelang.framelang import FrameEvent

class ForwardEvents:
    
    def __init__(self):
        
        # Create state stack.
        
        self.__state_stack = []
        
         # Create and intialize start state compartment.
        self.__state = self.__forwardevents_state_S0
        self.__compartment: 'ForwardEventsCompartment' = ForwardEventsCompartment(self.__state)
        self.__next_compartment: 'ForwardEventsCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def GotoS1(self,):
        e = FrameEvent("GotoS1",None)
        self.__mux(e)
    
    def GotoS2(self,):
        e = FrameEvent("GotoS2",None)
        self.__mux(e)
    
    def ReturnFromS1(self,):
        e = FrameEvent("ReturnFromS1",None)
        self.__mux(e)
    
    def ReturnFromS2(self,):
        e = FrameEvent("ReturnFromS2",None)
        self.__mux(e)
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__forwardevents_state_S0':
            self.__forwardevents_state_S0(e)
        elif self.__compartment.state.__name__ == '__forwardevents_state_S1':
            self.__forwardevents_state_S1(e)
        elif self.__compartment.state.__name__ == '__forwardevents_state_S2':
            self.__forwardevents_state_S2(e)
        
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
    
    # ===================== Machine Block =================== #
    
    def __forwardevents_state_S0(self, e):
        if e._message == ">":
            self.log_do("Enter $S0")
            
            return
        
        elif e._message == "<":
            self.log_do("Exit $S0")
            
            return
        
        elif e._message == "GotoS1":
            self.log_do("Recieved |GotoS1|")
            compartment = ForwardEventsCompartment(self.__forwardevents_state_S1)
            self.__transition(compartment)
            
            return
        
        elif e._message == "GotoS2":
            self.log_do("Recieved |GotoS2|")
            self.__state_stack_push(self.__compartment)
            compartment = ForwardEventsCompartment(self.__forwardevents_state_S2)
            self.__transition(compartment)
            
            return
        
        elif e._message == "ReturnFromS1":
            self.log_do("|ReturnFromS1| Forwarded")
            
            return
        
        elif e._message == "ReturnFromS2":
            self.log_do("|ReturnFromS2| Forwarded")
            
            return
        
    def __forwardevents_state_S1(self, e):
        if e._message == ">":
            self.log_do("Enter $S1")
            
            return
        
        elif e._message == "<":
            self.log_do("Exit $S1")
            
            return
        
        elif e._message == "ReturnFromS1":
            compartment = ForwardEventsCompartment(self.__forwardevents_state_S0)
            compartment.forward_event = e
            self.__transition(compartment)
            
            return
        
    def __forwardevents_state_S2(self, e):
        if e._message == ">":
            self.log_do("Enter $S2")
            
            return
        
        elif e._message == "<":
            self.log_do("Exit $S2")
            
            return
        
        elif e._message == "ReturnFromS2":
            compartment = self.__state_stack_pop()
            compartment.forward_event = e
            self.__transition(compartment)
            
            return
        
    
    # ===================== Actions Block =================== #
    
    
    
    def log_do(self,msg: str):
        self.tape.append(f'{msg}')
        
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'ForwardEventsCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'ForwardEventsCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def __state_stack_push(self, compartment: 'ForwardEventsCompartment'):
        self.__state_stack.append(compartment)
    
    def __state_stack_pop(self):
        return self.__state_stack.pop()
    
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class ForwardEventsCompartment:

    def __init__(self, state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class ForwardEventsController(ForwardEvents):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,msg: str):

# ********************

