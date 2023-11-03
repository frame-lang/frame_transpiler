# emitted from framec_v0.11.0
# get include files at https://github.com/frame-lang/frame-ancillary-files


from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class TransitParams:
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__transitparams_state_Init
        self.__compartment: 'TransitParamsCompartment' = TransitParamsCompartment(self.__state)
        self.__next_compartment: 'TransitParamsCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def Next(self,):
        e = FrameEvent("Next",None)
        self.__mux(e)
    
    # ===================== Machine Block =================== #
    
    def __transitparams_state_Init(self, e):
        if e._message == "Next":
            compartment = TransitParamsCompartment(self.__transitparams_state_A)
            compartment.enter_args["msg"] = "hi A"
            self.__transition(compartment)
            
            return
        
    def __transitparams_state_A(self, e):
        if e._message == ">":
            self.log_do(e._parameters["msg"])
            
            return
        
        elif e._message == "<":
            self.log_do("bye A")
            
            return
        
        elif e._message == "Next":
            compartment = TransitParamsCompartment(self.__transitparams_state_B)
            compartment.enter_args["msg"] = "hi B"
            compartment.enter_args["val"] = 42
            self.__transition(compartment)
            
            return
        
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
            compartment = TransitParamsCompartment(self.__transitparams_state_A)
            compartment.enter_args["msg"] = "hi again A"
            self.__transition(compartment)
            
            return
        
    
    # ===================== Actions Block =================== #
    
    
    
    def log_do(self,msg: str):
        raise NotImplementedError
    
    
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__transitparams_state_Init':
            self.__transitparams_state_Init(e)
        elif self.__compartment.state.__name__ == '__transitparams_state_A':
            self.__transitparams_state_A(e)
        elif self.__compartment.state.__name__ == '__transitparams_state_B':
            self.__transitparams_state_B(e)
        
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
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'TransitParamsCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'TransitParamsCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class TransitParamsCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class TransitParamsController(TransitParams):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,msg: str):
        #pass

# ********************

