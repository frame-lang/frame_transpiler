# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class SimpleHandlerCalls:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = '__simplehandlercalls_state_Init'
        self.__compartment: 'SimpleHandlerCallsCompartment' = SimpleHandlerCallsCompartment(self.__state)
        self.__next_compartment: 'SimpleHandlerCallsCompartment' = None
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def A(self,):
        e = FrameEvent("A",None)
        self.__kernel(e)
    
    def B(self,):
        e = FrameEvent("B",None)
        self.__kernel(e)
    
    def C(self,):
        e = FrameEvent("C",None)
        self.__kernel(e)
    
    def D(self,):
        e = FrameEvent("D",None)
        self.__kernel(e)
    
    def E(self,):
        e = FrameEvent("E",None)
        self.__kernel(e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __simplehandlercalls_state_Init(self, e):
        if e._message == "A":
            compartment = SimpleHandlerCallsCompartment('__simplehandlercalls_state_A')
            self.__transition(compartment)
            return
        elif e._message == "B":
            compartment = SimpleHandlerCallsCompartment('__simplehandlercalls_state_B')
            self.__transition(compartment)
            return
        elif e._message == "C":
            self.A()
            return
            return
        elif e._message == "D":
            self.B()
            return
            compartment = SimpleHandlerCallsCompartment('__simplehandlercalls_state_A')
            self.__transition(compartment)
            return
        elif e._message == "E":
            self.D()
            return
            self.C()
            return
            return
    
    # ----------------------------------------
    # $A
    
    def __simplehandlercalls_state_A(self, e):
        pass
        
    
    # ----------------------------------------
    # $B
    
    def __simplehandlercalls_state_B(self, e):
        pass
        
    
    
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
        if self.__compartment.state == '__simplehandlercalls_state_Init':
            self.__simplehandlercalls_state_Init(e)
        elif self.__compartment.state == '__simplehandlercalls_state_A':
            self.__simplehandlercalls_state_A(e)
        elif self.__compartment.state == '__simplehandlercalls_state_B':
            self.__simplehandlercalls_state_B(e)
        
    def __transition(self, compartment: 'SimpleHandlerCallsCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class SimpleHandlerCallsCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    