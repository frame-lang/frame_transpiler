# emitted from framec_v0.11.0
# get include files at https://github.com/frame-lang/frame-ancillary-files


from framelang.framelang import FrameEvent




class StateContextStack:
    
    def __init__(self):
        
        # Create state stack.
        
        self.__state_stack = []
        
         # Create and intialize start state compartment.
        
        self.__state = self.__statecontextstack_state_A
        self.__compartment: 'StateContextStackCompartment' = StateContextStackCompartment(self.__state)
        self.__next_compartment: 'StateContextStackCompartment' = None
        self.__compartment.state_vars["x"] = 0
        
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
    
    def inc(self,):
        e = FrameEvent("inc",None)
        self.__mux(e)
    
    def value(self,):
        e = FrameEvent("value",None)
        self.__mux(e)
        return e._return
    
    def push(self,):
        e = FrameEvent("push",None)
        self.__mux(e)
    
    def pop(self,):
        e = FrameEvent("pop",None)
        self.__mux(e)
    
    def pop_change(self,):
        e = FrameEvent("pop_change",None)
        self.__mux(e)
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__statecontextstack_state_A':
            self.__statecontextstack_state_A(e)
        elif self.__compartment.state.__name__ == '__statecontextstack_state_B':
            self.__statecontextstack_state_B(e)
        elif self.__compartment.state.__name__ == '__statecontextstack_state_C':
            self.__statecontextstack_state_C(e)
        
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
    
    def __statecontextstack_state_A(self, e):
        if e._message == ">":
            self.log_do("A:>")
            
            return
        
        elif e._message == "<":
            self.log_do("A:<")
            
            return
        
        elif e._message == "inc":
            (self.__compartment.state_vars["x"]) = (self.__compartment.state_vars["x"]) + 1
            
            return
        
        elif e._message == "value":
            e._return = (self.__compartment.state_vars["x"])
            return
            
        
        elif e._message == "to_a":
            compartment = StateContextStackCompartment(self.__statecontextstack_state_A)
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            
            return
        
        elif e._message == "to_b":
            compartment = StateContextStackCompartment(self.__statecontextstack_state_B)
            compartment.state_vars["y"] = 0
            self.__transition(compartment)
            
            return
        
        elif e._message == "to_c":
            compartment = StateContextStackCompartment(self.__statecontextstack_state_C)
            compartment.state_vars["z"] = 0
            self.__transition(compartment)
            
            return
        
        elif e._message == "push":
            self.__state_stack_push(self.__compartment)
            
            return
        
        elif e._message == "pop":
            compartment = self.__state_stack_pop()
            self.__transition(compartment)
            
            return
        
        elif e._message == "pop_change":
            compartment = self.__state_stack_pop()
            self.__change_state(compartment)
            
            return
        
    def __statecontextstack_state_B(self, e):
        if e._message == ">":
            self.log_do("B:>")
            
            return
        
        elif e._message == "<":
            self.log_do("B:<")
            
            return
        
        elif e._message == "inc":
            (self.__compartment.state_vars["y"]) = (self.__compartment.state_vars["y"]) + 5
            
            return
        
        elif e._message == "value":
            e._return = (self.__compartment.state_vars["y"])
            return
            
        
        elif e._message == "to_a":
            compartment = StateContextStackCompartment(self.__statecontextstack_state_A)
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            
            return
        
        elif e._message == "to_b":
            compartment = StateContextStackCompartment(self.__statecontextstack_state_B)
            compartment.state_vars["y"] = 0
            self.__transition(compartment)
            
            return
        
        elif e._message == "to_c":
            compartment = StateContextStackCompartment(self.__statecontextstack_state_C)
            compartment.state_vars["z"] = 0
            self.__transition(compartment)
            
            return
        
        elif e._message == "push":
            self.__state_stack_push(self.__compartment)
            
            return
        
        elif e._message == "pop":
            compartment = self.__state_stack_pop()
            self.__transition(compartment)
            
            return
        
        elif e._message == "pop_change":
            compartment = self.__state_stack_pop()
            self.__change_state(compartment)
            
            return
        
    def __statecontextstack_state_C(self, e):
        if e._message == ">":
            self.log_do("C:>")
            
            return
        
        elif e._message == "<":
            self.log_do("C:<")
            
            return
        
        elif e._message == "inc":
            (self.__compartment.state_vars["z"]) = (self.__compartment.state_vars["z"]) + 10
            
            return
        
        elif e._message == "value":
            e._return = (self.__compartment.state_vars["z"])
            return
            
        
        elif e._message == "to_a":
            compartment = StateContextStackCompartment(self.__statecontextstack_state_A)
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            
            return
        
        elif e._message == "to_b":
            compartment = StateContextStackCompartment(self.__statecontextstack_state_B)
            compartment.state_vars["y"] = 0
            self.__transition(compartment)
            
            return
        
        elif e._message == "to_c":
            compartment = StateContextStackCompartment(self.__statecontextstack_state_C)
            compartment.state_vars["z"] = 0
            self.__transition(compartment)
            
            return
        
        elif e._message == "push":
            self.__state_stack_push(self.__compartment)
            
            return
        
        elif e._message == "pop":
            compartment = self.__state_stack_pop()
            self.__transition(compartment)
            
            return
        
        elif e._message == "pop_change":
            compartment = self.__state_stack_pop()
            self.__change_state(compartment)
            
            return
        
    
    # ===================== Actions Block =================== #
    
    
    
    def log_do(self,msg: str):
        raise NotImplementedError
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'StateContextStackCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'StateContextStackCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def __state_stack_push(self, compartment: 'StateContextStackCompartment'):
        self.__state_stack.append(compartment)
    
    def __state_stack_pop(self):
        return self.__state_stack.pop()
    
    
    def __change_state(self, new_compartment: 'StateContextStackCompartment'):
        self.__compartment = new_compartment
    
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class StateContextStackCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class StateContextStackController(StateContextStack):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,msg: str):
        #pass

# ********************

