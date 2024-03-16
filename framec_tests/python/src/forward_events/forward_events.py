#Emitted from framec_v0.11.2



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class ForwardEvents:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
        # Create state stack.
        
        self.__state_stack = []
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = ForwardEventsCompartment('__forwardevents_state_S0', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = []
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def GotoS1(self,):
        self.return_stack.append(None)
        __e = FrameEvent("GotoS1",None)
        self.__kernel(__e)
        self.return_stack.pop(-1)
    
    def GotoS2(self,):
        self.return_stack.append(None)
        __e = FrameEvent("GotoS2",None)
        self.__kernel(__e)
        self.return_stack.pop(-1)
    
    def ReturnFromS1(self,):
        self.return_stack.append(None)
        __e = FrameEvent("ReturnFromS1",None)
        self.__kernel(__e)
        self.return_stack.pop(-1)
    
    def ReturnFromS2(self,):
        self.return_stack.append(None)
        __e = FrameEvent("ReturnFromS2",None)
        self.__kernel(__e)
        self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $S0
    
    def __forwardevents_state_S0(self, __e, compartment):
        if __e._message == ">":
            self.log_do("Enter $S0")
            return
        elif __e._message == "<":
            self.log_do("Exit $S0")
            return
        elif __e._message == "GotoS1":
            self.log_do("Recieved |GotoS1|")
            next_compartment = None
            next_compartment = ForwardEventsCompartment('__forwardevents_state_S1', next_compartment)
            self.__transition(next_compartment)
            return
        elif __e._message == "GotoS2":
            self.log_do("Recieved |GotoS2|")
            self.__state_stack_push(self.__compartment)
            next_compartment = None
            next_compartment = ForwardEventsCompartment('__forwardevents_state_S2', next_compartment)
            self.__transition(next_compartment)
            return
        elif __e._message == "ReturnFromS1":
            self.log_do("|ReturnFromS1| Forwarded")
            return
        elif __e._message == "ReturnFromS2":
            self.log_do("|ReturnFromS2| Forwarded")
            return
    
    # ----------------------------------------
    # $S1
    
    def __forwardevents_state_S1(self, __e, compartment):
        if __e._message == ">":
            self.log_do("Enter $S1")
            return
        elif __e._message == "<":
            self.log_do("Exit $S1")
            return
        elif __e._message == "ReturnFromS1":
            next_compartment = None
            next_compartment = ForwardEventsCompartment('__forwardevents_state_S0', next_compartment)
            next_compartment.forward_event = __e
            next_compartment.forward_event = __e
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S2
    
    def __forwardevents_state_S2(self, __e, compartment):
        if __e._message == ">":
            self.log_do("Enter $S2")
            return
        elif __e._message == "<":
            self.log_do("Exit $S2")
            return
        elif __e._message == "ReturnFromS2":
            next_compartment = self.__state_stack_pop()
            next_compartment.forward_event = __e
            self.__transition(next_compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def log_do(self,msg: str):
        self.tape.append(f'{msg}')
        
    
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
        if self.__compartment.state == '__forwardevents_state_S0':
            self.__forwardevents_state_S0(__e, self.__compartment)
        elif self.__compartment.state == '__forwardevents_state_S1':
            self.__forwardevents_state_S1(__e, self.__compartment)
        elif self.__compartment.state == '__forwardevents_state_S2':
            self.__forwardevents_state_S2(__e, self.__compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    
    def __state_stack_push(self, compartment):
        self.__state_stack.append(compartment)
    
    def __state_stack_pop(self):
        return self.__state_stack.pop()
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class ForwardEventsCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    