#Emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class SimpleHandlerCalls:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = SimpleHandlerCallsCompartment('__simplehandlercalls_state_Init', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def A(self,):
        __e = FrameEvent("A",None)
        self.__kernel(__e)
    
    def B(self,):
        __e = FrameEvent("B",None)
        self.__kernel(__e)
    
    def C(self,):
        __e = FrameEvent("C",None)
        self.__kernel(__e)
    
    def D(self,):
        __e = FrameEvent("D",None)
        self.__kernel(__e)
    
    def E(self,):
        __e = FrameEvent("E",None)
        self.__kernel(__e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __simplehandlercalls_state_Init(self, __e, compartment):
        if __e._message == "A":
            next_compartment = None
            next_compartment = SimpleHandlerCallsCompartment('__simplehandlercalls_state_A', next_compartment)
            
            self.__transition(next_compartment)
            return
        elif __e._message == "B":
            next_compartment = None
            next_compartment = SimpleHandlerCallsCompartment('__simplehandlercalls_state_B', next_compartment)
            
            self.__transition(next_compartment)
            return
        elif __e._message == "C":
            self.A()
            return
            return
        elif __e._message == "D":
            self.B()
            return
            next_compartment = None
            next_compartment = SimpleHandlerCallsCompartment('__simplehandlercalls_state_A', next_compartment)
            
            self.__transition(next_compartment)
            return
        elif __e._message == "E":
            self.D()
            return
            self.C()
            return
            return
    
    # ----------------------------------------
    # $A
    
    def __simplehandlercalls_state_A(self, __e, compartment):
        pass
        
    
    # ----------------------------------------
    # $B
    
    def __simplehandlercalls_state_B(self, __e, compartment):
        pass
        
    
    
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
        if self.__compartment.state == '__simplehandlercalls_state_Init':
            self.__simplehandlercalls_state_Init(__e, self.__compartment)
        elif self.__compartment.state == '__simplehandlercalls_state_A':
            self.__simplehandlercalls_state_A(__e, self.__compartment)
        elif self.__compartment.state == '__simplehandlercalls_state_B':
            self.__simplehandlercalls_state_B(__e, self.__compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class SimpleHandlerCallsCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    