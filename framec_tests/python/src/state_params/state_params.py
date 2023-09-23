# emitted from framec_v0.11.0
# get include files at https://github.com/frame-lang/frame-ancillary-files


from framelang.framelang import FrameEvent




class StateParams:
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__stateparams_state_Init
        self.__compartment: 'StateParamsCompartment' = StateParamsCompartment(self.__state)
        self.__next_compartment: 'StateParamsCompartment' = None
        
        # Initialize domain
        
        self.param_log  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def Next(self,):
        e = FrameEvent("Next",None)
        self.__mux(e)
    
    def Prev(self,):
        e = FrameEvent("Prev",None)
        self.__mux(e)
    
    def Log(self,):
        e = FrameEvent("Log",None)
        self.__mux(e)
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__stateparams_state_Init':
            self.__stateparams_state_Init(e)
        elif self.__compartment.state.__name__ == '__stateparams_state_Split':
            self.__stateparams_state_Split(e)
        elif self.__compartment.state.__name__ == '__stateparams_state_Merge':
            self.__stateparams_state_Merge(e)
        
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
    
    def __stateparams_state_Init(self, e):
        if e._message == "Next":
            compartment = StateParamsCompartment(self.__stateparams_state_Split)
            compartment.state_args["val"] = 1
            self.__transition(compartment)
            
            return
        
    def __stateparams_state_Split(self, e):
        if e._message == "Next":
            compartment = StateParamsCompartment(self.__stateparams_state_Merge)
            compartment.state_args["left"] = self.__compartment.state_args["val"]
            compartment.state_args["right"] = self.__compartment.state_args["val"] + 1
            self.__transition(compartment)
            
            return
        
        elif e._message == "Prev":
            compartment = StateParamsCompartment(self.__stateparams_state_Merge)
            compartment.state_args["left"] = self.__compartment.state_args["val"] + 1
            compartment.state_args["right"] = self.__compartment.state_args["val"]
            self.__transition(compartment)
            
            return
        
        elif e._message == "Log":
            self.got_param_do("val",(self.__compartment.state_args["val"]))
            
            return
        
    def __stateparams_state_Merge(self, e):
        if e._message == "Next":
            compartment = StateParamsCompartment(self.__stateparams_state_Split)
            compartment.state_args["val"] = self.__compartment.state_args["left"] + self.__compartment.state_args["right"]
            self.__transition(compartment)
            
            return
        
        elif e._message == "Prev":
            compartment = StateParamsCompartment(self.__stateparams_state_Split)
            compartment.state_args["val"] = self.__compartment.state_args["left"] * self.__compartment.state_args["right"]
            self.__transition(compartment)
            
            return
        
        elif e._message == "Log":
            self.got_param_do("left",(self.__compartment.state_args["left"]))
            self.got_param_do("right",(self.__compartment.state_args["right"]))
            
            return
        
    
    # ===================== Actions Block =================== #
    
    
    
    def got_param_do(self,name: str,val: int):
        raise NotImplementedError
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'StateParamsCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'StateParamsCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class StateParamsCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class StateParamsController(StateParams):
	#def __init__(self,):
	    #super().__init__()

    #def got_param_do(self,name: str,val: int):
        #pass

# ********************

