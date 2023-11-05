# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class StateContextSm:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__statecontextsm_state_Init
        self.__compartment: 'StateContextSmCompartment' = StateContextSmCompartment(self.__state)
        self.__next_compartment: 'StateContextSmCompartment' = None
        self.__compartment.state_vars["w"] = 0
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def Start(self,):
        e = FrameEvent("Start",None)
        self.__mux(e)
    
    def LogState(self,):
        e = FrameEvent("LogState",None)
        self.__mux(e)
    
    def Inc(self,):
        e = FrameEvent("Inc",None)
        self.__mux(e)
        return e._return
    
    def Next(self,arg: int):
        parameters = {}
        parameters["arg"] = arg

        e = FrameEvent("Next",parameters)
        self.__mux(e)
    
    # ===================== Machine Block =================== #
      #  Change [arg:int]
    
    
    # $Init
    def __statecontextsm_state_Init(self, e):
        if e._message == ">":
            (self.__compartment.state_vars["w"]) = 3
            self.log_do("w",(self.__compartment.state_vars["w"]))
            return
        elif e._message == "Inc":
            (self.__compartment.state_vars["w"]) = self.__compartment.state_vars["w"] + 1
            self.log_do("w",(self.__compartment.state_vars["w"]))
            e._return = (self.__compartment.state_vars["w"])
            return
            
        elif e._message == "LogState":
            self.log_do("w",(self.__compartment.state_vars["w"]))
            return
        elif e._message == "Start":
            compartment = StateContextSmCompartment(self.__statecontextsm_state_Foo)
            compartment.enter_args["a"] = 3
            compartment.enter_args["b"] = self.__compartment.state_vars["w"]
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            return
    
    # $Foo
    def __statecontextsm_state_Foo(self, e):
        if e._message == ">":
            self.log_do("a",e._parameters["a"])
            self.log_do("b",e._parameters["b"])
            (self.__compartment.state_vars["x"]) = e._parameters["a"] * e._parameters["b"]
            self.log_do("x",(self.__compartment.state_vars["x"]))
            return
        elif e._message == "<":
            self.log_do("c",e._parameters["c"])
            (self.__compartment.state_vars["x"]) = self.__compartment.state_vars["x"] + e._parameters["c"]
            self.log_do("x",(self.__compartment.state_vars["x"]))
            return
        elif e._message == "LogState":
            self.log_do("x",(self.__compartment.state_vars["x"]))
            return
        elif e._message == "Inc":
            (self.__compartment.state_vars["x"]) = self.__compartment.state_vars["x"] + 1
            self.log_do("x",(self.__compartment.state_vars["x"]))
            e._return = (self.__compartment.state_vars["x"])
            return
            
        elif e._message == "Next":
            tmp  = e._parameters["arg"] * 10
            self.__compartment.exit_args["c"] = 10
            compartment = StateContextSmCompartment(self.__statecontextsm_state_Bar)
            compartment.enter_args["a"] = tmp
            compartment.state_args["y"] = self.__compartment.state_vars["x"]
            compartment.state_vars["z"] = 0
            self.__transition(compartment)
            return
      #  FIXME: Swapping this to 10 * arg causes a parse error!
      #  |Change| [arg:int]
      #      var tmp = x + arg
      #      -> $Bar(tmp)
      #      ^
    
    
    # $Bar
    def __statecontextsm_state_Bar(self, e):
        if e._message == ">":
            self.log_do("a",e._parameters["a"])
            self.log_do("y",(self.__compartment.state_args["y"]))
            (self.__compartment.state_vars["z"]) = e._parameters["a"] + self.__compartment.state_args["y"]
            self.log_do("z",(self.__compartment.state_vars["z"]))
            return
        elif e._message == "LogState":
            self.log_do("y",(self.__compartment.state_args["y"]))
            self.log_do("z",(self.__compartment.state_vars["z"]))
            return
        elif e._message == "Inc":
            (self.__compartment.state_vars["z"]) = self.__compartment.state_vars["z"] + 1
            self.log_do("z",(self.__compartment.state_vars["z"]))
            e._return = (self.__compartment.state_vars["z"])
            return
            
    
    # ===================== Actions Block =================== #
    
    def log_do(self,name: str,val: int):
        raise NotImplementedError
    
    # =============== Machinery and Mechanisms ============== #
    
    def __mux(self, e):
        
        self.__router(e)
        
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            if(next_compartment.forward_event is not None and 
               next_compartment.forward_event._message == ">"):
                self.__router(FrameEvent( "<", self.__compartment.exit_args))
                self.__compartment = next_compartment
                self.__router(next_compartment.forward_event)
            else:
                self.__do_transition(next_compartment)
                if next_compartment.forward_event is not None:
                    self.__router(next_compartment.forward_event)
            next_compartment.forward_event = None
    
    def __router(self, e):
        if self.__compartment.state.__name__ == '__statecontextsm_state_Init':
            self.__statecontextsm_state_Init(e)
        elif self.__compartment.state.__name__ == '__statecontextsm_state_Foo':
            self.__statecontextsm_state_Foo(e)
        elif self.__compartment.state.__name__ == '__statecontextsm_state_Bar':
            self.__statecontextsm_state_Bar(e)
        
    def __transition(self, compartment: 'StateContextSmCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'StateContextSmCompartment'):
        self.__router(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__router(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class StateContextSmCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    
# ********************

#class StateContextSmController(StateContextSm):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,name: str,val: int):
        #pass

# ********************

