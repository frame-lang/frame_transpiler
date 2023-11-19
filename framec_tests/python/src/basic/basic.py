# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class Basic:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = '__basic_state_S0'
        self.__compartment: 'BasicCompartment' = BasicCompartment(self.__state)
        self.__next_compartment: 'BasicCompartment' = None
        
        # Initialize domain
        
        self.entry_log  = []
        self.exit_log  = []
        
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
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $S0
    
    def __basic_state_S0(self, e):
        if e._message == ">":
            self.entered_do("S0")
            return
        elif e._message == "<":
            self.left_do("S0")
            return
        elif e._message == "A":
            # ooh
            compartment = BasicCompartment('__basic_state_S1')
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $S1
    
    def __basic_state_S1(self, e):
        if e._message == ">":
            self.entered_do("S1")
            return
        elif e._message == "<":
            self.left_do("S1")
            return
        elif e._message == "B":
            # aah
            compartment = BasicCompartment('__basic_state_S0')
            self.__transition(compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def entered_do(self,msg: str):
        raise NotImplementedError
    
    def left_do(self,msg: str):
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
        if self.__compartment.state == '__basic_state_S0':
            self.__basic_state_S0(e)
        elif self.__compartment.state == '__basic_state_S1':
            self.__basic_state_S1(e)
        
    def __transition(self, compartment: 'BasicCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class BasicCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    