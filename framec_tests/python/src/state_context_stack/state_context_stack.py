



from framelang.framelang import FrameEvent


# Emitted from framec_v0.11.0

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class StateContextStack:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
        # Create state stack.
        
        self.__state_stack = []
        
         # Create and intialize start state compartment.
        
        self.__compartment: 'StateContextStackCompartment' = StateContextStackCompartment('__statecontextstack_state_A')
        self.__next_compartment: 'StateContextStackCompartment' = None
        self.__compartment.state_vars["x"] = 0
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def to_a(self,):
        e = FrameEvent("to_a",None)
        self.__kernel(e)
    
    def to_b(self,):
        e = FrameEvent("to_b",None)
        self.__kernel(e)
    
    def to_c(self,):
        e = FrameEvent("to_c",None)
        self.__kernel(e)
    
    def inc(self,):
        e = FrameEvent("inc",None)
        self.__kernel(e)
    
    def value(self,):
        e = FrameEvent("value",None)
        self.__kernel(e)
        return e._return
    
    def push(self,):
        e = FrameEvent("push",None)
        self.__kernel(e)
    
    def pop(self,):
        e = FrameEvent("pop",None)
        self.__kernel(e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $A
    
    def __statecontextstack_state_A(self, e):
        if e._message == ">":
            self.log_do("A:>")
            return
        elif e._message == "<":
            self.log_do("A:<")
            return
        elif e._message == "inc":
            (self.__compartment.state_vars["x"]) = self.__compartment.state_vars["x"] + 1
            return
        elif e._message == "value":
            e._return = (self.__compartment.state_vars["x"])
            return
            
        elif e._message == "to_a":
            compartment = StateContextStackCompartment('__statecontextstack_state_A')
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            return
        elif e._message == "to_b":
            compartment = StateContextStackCompartment('__statecontextstack_state_B')
            compartment.state_vars["y"] = 0
            self.__transition(compartment)
            return
        elif e._message == "to_c":
            compartment = StateContextStackCompartment('__statecontextstack_state_C')
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
    
    # ----------------------------------------
    # $B
    
    def __statecontextstack_state_B(self, e):
        if e._message == ">":
            self.log_do("B:>")
            return
        elif e._message == "<":
            self.log_do("B:<")
            return
        elif e._message == "inc":
            (self.__compartment.state_vars["y"]) = self.__compartment.state_vars["y"] + 5
            return
        elif e._message == "value":
            e._return = (self.__compartment.state_vars["y"])
            return
            
        elif e._message == "to_a":
            compartment = StateContextStackCompartment('__statecontextstack_state_A')
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            return
        elif e._message == "to_b":
            compartment = StateContextStackCompartment('__statecontextstack_state_B')
            compartment.state_vars["y"] = 0
            self.__transition(compartment)
            return
        elif e._message == "to_c":
            compartment = StateContextStackCompartment('__statecontextstack_state_C')
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
    
    # ----------------------------------------
    # $C
    
    def __statecontextstack_state_C(self, e):
        if e._message == ">":
            self.log_do("C:>")
            return
        elif e._message == "<":
            self.log_do("C:<")
            return
        elif e._message == "inc":
            (self.__compartment.state_vars["z"]) = self.__compartment.state_vars["z"] + 10
            return
        elif e._message == "value":
            e._return = (self.__compartment.state_vars["z"])
            return
            
        elif e._message == "to_a":
            compartment = StateContextStackCompartment('__statecontextstack_state_A')
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            return
        elif e._message == "to_b":
            compartment = StateContextStackCompartment('__statecontextstack_state_B')
            compartment.state_vars["y"] = 0
            self.__transition(compartment)
            return
        elif e._message == "to_c":
            compartment = StateContextStackCompartment('__statecontextstack_state_C')
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
    
    # ===================== Actions Block =================== #
    
    def log_do(self,msg: str):
        raise NotImplementedError
    
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
        if self.__compartment.state == '__statecontextstack_state_A':
            self.__statecontextstack_state_A(e)
        elif self.__compartment.state == '__statecontextstack_state_B':
            self.__statecontextstack_state_B(e)
        elif self.__compartment.state == '__statecontextstack_state_C':
            self.__statecontextstack_state_C(e)
        
    def __transition(self, compartment: 'StateContextStackCompartment'):
        self.__next_compartment = compartment
    
    def __state_stack_push(self, compartment: 'StateContextStackCompartment'):
        self.__state_stack.append(compartment)
    
    def __state_stack_pop(self):
        return self.__state_stack.pop()
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class StateContextStackCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    