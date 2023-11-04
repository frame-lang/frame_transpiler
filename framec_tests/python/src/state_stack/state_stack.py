# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class StateStack:
    
    def __init__(self):
        
        # Create state stack.
        
        self.__state_stack = []
        
         # Create and intialize start state compartment.
        
        self.__state = self.__statestack_state_A
        self.__compartment: 'StateStackCompartment' = StateStackCompartment(self.__state)
        self.__next_compartment: 'StateStackCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def to_a(self,):
        e = FrameEvent("to_a",None)
        self.__mux(e)
    
    def to_b(self,):
        e = FrameEvent("to_b",None)
        self.__mux(e)
    
    def to_c(self,):
        e = FrameEvent("to_c",None)
        self.__mux(e)
    
    def push(self,):
        e = FrameEvent("push",None)
        self.__mux(e)
    
    def pop(self,):
        e = FrameEvent("pop",None)
        self.__mux(e)
    
    # ===================== Machine Block =================== #
    
    def __statestack_state_A(self, e):
        if e._message == ">":
            self.log_do("A:>")
            return
        elif e._message == "<":
            self.log_do("A:<")
            return
        elif e._message == "to_a":
            compartment = StateStackCompartment(self.__statestack_state_A)
            self.__transition(compartment)
            return
        elif e._message == "to_b":
            compartment = StateStackCompartment(self.__statestack_state_B)
            self.__transition(compartment)
            return
        elif e._message == "to_c":
            compartment = StateStackCompartment(self.__statestack_state_C)
            self.__transition(compartment)
            return
        elif e._message == "push":
            self.__state_stack_push(self.__compartment)
            return
        elif e._message == "pop":
            compartment = self.__state_stack_pop()
            self.__transition(compartment)
            return
    
    def __statestack_state_B(self, e):
        if e._message == ">":
            self.log_do("B:>")
            return
        elif e._message == "<":
            self.log_do("B:<")
            return
        elif e._message == "to_a":
            compartment = StateStackCompartment(self.__statestack_state_A)
            self.__transition(compartment)
            return
        elif e._message == "to_b":
            compartment = StateStackCompartment(self.__statestack_state_B)
            self.__transition(compartment)
            return
        elif e._message == "to_c":
            compartment = StateStackCompartment(self.__statestack_state_C)
            self.__transition(compartment)
            return
        elif e._message == "push":
            self.__state_stack_push(self.__compartment)
            return
        elif e._message == "pop":
            compartment = self.__state_stack_pop()
            self.__transition(compartment)
            return
    
    def __statestack_state_C(self, e):
        if e._message == ">":
            self.log_do("C:>")
            return
        elif e._message == "<":
            self.log_do("C:<")
            return
        elif e._message == "to_a":
            compartment = StateStackCompartment(self.__statestack_state_A)
            self.__transition(compartment)
            return
        elif e._message == "to_b":
            compartment = StateStackCompartment(self.__statestack_state_B)
            self.__transition(compartment)
            return
        elif e._message == "to_c":
            compartment = StateStackCompartment(self.__statestack_state_C)
            self.__transition(compartment)
            return
        elif e._message == "push":
            self.__state_stack_push(self.__compartment)
            return
        elif e._message == "pop":
            compartment = self.__state_stack_pop()
            self.__transition(compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def log_do(self,msg: str):
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
        if self.__compartment.state.__name__ == '__statestack_state_A':
            self.__statestack_state_A(e)
        elif self.__compartment.state.__name__ == '__statestack_state_B':
            self.__statestack_state_B(e)
        elif self.__compartment.state.__name__ == '__statestack_state_C':
            self.__statestack_state_C(e)
        
    def __transition(self, compartment: 'StateStackCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'StateStackCompartment'):
        self.__router(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__router(FrameEvent(">", self.__compartment.enter_args))
    
    def __state_stack_push(self, compartment: 'StateStackCompartment'):
        self.__state_stack.append(compartment)
    
    def __state_stack_pop(self):
        return self.__state_stack.pop()
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class StateStackCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    


# ********************

#class StateStackController(StateStack):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,msg: str):
        #pass

# ********************

