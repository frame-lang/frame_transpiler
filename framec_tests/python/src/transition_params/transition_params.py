



from framelang.framelang import FrameEvent


# Emitted from framec_v0.11.0

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class TransitParams:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__compartment: 'TransitParamsCompartment' = TransitParamsCompartment('__transitparams_state_Init')
        self.__next_compartment: 'TransitParamsCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def Next(self,):
        e = FrameEvent("Next",None)
        self.__kernel(e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __transitparams_state_Init(self, e):
        if e._message == "Next":
            compartment = TransitParamsCompartment('__transitparams_state_A')
            compartment.enter_args["msg"] = "hi A"
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $A
    
    def __transitparams_state_A(self, e):
        if e._message == ">":
            self.log_do(e._parameters["msg"])
            return
        elif e._message == "<":
            self.log_do("bye A")
            return
        elif e._message == "Next":
            compartment = TransitParamsCompartment('__transitparams_state_B')
            compartment.enter_args["msg"] = "hi B"
            compartment.enter_args["val"] = 42
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $B
    
    def __transitparams_state_B(self, e):
        if e._message == ">":
            self.log_do(e._parameters["msg"])
            self.log_do(str(e._parameters["val"]))
            return
        elif e._message == "<":
            self.log_do(str(e._parameters["val"]))
            self.log_do(e._parameters["msg"])
            return
        elif e._message == "Next":
            self.__compartment.exit_args["val"] = True
            self.__compartment.exit_args["msg"] = "bye B"
            compartment = TransitParamsCompartment('__transitparams_state_A')
            compartment.enter_args["msg"] = "hi again A"
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
        if self.__compartment.state == '__transitparams_state_Init':
            self.__transitparams_state_Init(e)
        elif self.__compartment.state == '__transitparams_state_A':
            self.__transitparams_state_A(e)
        elif self.__compartment.state == '__transitparams_state_B':
            self.__transitparams_state_B(e)
        
    def __transition(self, compartment: 'TransitParamsCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class TransitParamsCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    