# Emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class StateStack:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
        # Create state stack.
        
        self.__state_stack = []
        
         # Create and intialize start state compartment.
        
        self.__compartment: 'StateStackCompartment' = StateStackCompartment('__statestack_state_A')
        self.__next_compartment: 'StateStackCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def to_a(self,):
        __e = FrameEvent("to_a",None)
        self.__kernel(__e)
    
    def to_b(self,):
        __e = FrameEvent("to_b",None)
        self.__kernel(__e)
    
    def to_c(self,):
        __e = FrameEvent("to_c",None)
        self.__kernel(__e)
    
    def push(self,):
        __e = FrameEvent("push",None)
        self.__kernel(__e)
    
    def pop(self,):
        __e = FrameEvent("pop",None)
        self.__kernel(__e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $A
    
    def __statestack_state_A(self, __e):
        if __e._message == ">":
            self.log_do("A:>")
            return
        elif __e._message == "<":
            self.log_do("A:<")
            return
        elif __e._message == "to_a":
            next_compartment = StateStackCompartment('__statestack_state_A')
            self.__transition(next_compartment)
            return
        elif __e._message == "to_b":
            next_compartment = StateStackCompartment('__statestack_state_B')
            self.__transition(next_compartment)
            return
        elif __e._message == "to_c":
            next_compartment = StateStackCompartment('__statestack_state_C')
            self.__transition(next_compartment)
            return
        elif __e._message == "push":
            self.__state_stack_push(self.__compartment)
            return
        elif __e._message == "pop":
            next_compartment = self.__state_stack_pop()
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $B
    
    def __statestack_state_B(self, __e):
        if __e._message == ">":
            self.log_do("B:>")
            return
        elif __e._message == "<":
            self.log_do("B:<")
            return
        elif __e._message == "to_a":
            next_compartment = StateStackCompartment('__statestack_state_A')
            self.__transition(next_compartment)
            return
        elif __e._message == "to_b":
            next_compartment = StateStackCompartment('__statestack_state_B')
            self.__transition(next_compartment)
            return
        elif __e._message == "to_c":
            next_compartment = StateStackCompartment('__statestack_state_C')
            self.__transition(next_compartment)
            return
        elif __e._message == "push":
            self.__state_stack_push(self.__compartment)
            return
        elif __e._message == "pop":
            next_compartment = self.__state_stack_pop()
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $C
    
    def __statestack_state_C(self, __e):
        if __e._message == ">":
            self.log_do("C:>")
            return
        elif __e._message == "<":
            self.log_do("C:<")
            return
        elif __e._message == "to_a":
            next_compartment = StateStackCompartment('__statestack_state_A')
            self.__transition(next_compartment)
            return
        elif __e._message == "to_b":
            next_compartment = StateStackCompartment('__statestack_state_B')
            self.__transition(next_compartment)
            return
        elif __e._message == "to_c":
            next_compartment = StateStackCompartment('__statestack_state_C')
            self.__transition(next_compartment)
            return
        elif __e._message == "push":
            self.__state_stack_push(self.__compartment)
            return
        elif __e._message == "pop":
            next_compartment = self.__state_stack_pop()
            self.__transition(next_compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def log_do(self,msg: str):
        raise NotImplementedError
    
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
        if self.__compartment.state == '__statestack_state_A':
            self.__statestack_state_A(__e)
        elif self.__compartment.state == '__statestack_state_B':
            self.__statestack_state_B(__e)
        elif self.__compartment.state == '__statestack_state_C':
            self.__statestack_state_C(__e)
        
    def __transition(self, next_compartment: 'StateStackCompartment'):
        self.__next_compartment = next_compartment
    
    def __state_stack_push(self, compartment: 'StateStackCompartment'):
        self.__state_stack.append(compartment)
    
    def __state_stack_pop(self):
        return self.__state_stack.pop()
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class StateStackCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    