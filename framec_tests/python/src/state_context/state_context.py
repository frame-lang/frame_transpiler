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
        
        self.__state = '__statecontextsm_state_Init'
        self.__compartment: 'StateContextSmCompartment' = StateContextSmCompartment(self.__state)
        self.__next_compartment: 'StateContextSmCompartment' = None
        self.__compartment.state_vars["w"] = 0
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def Start(self,):
        e = FrameEvent("Start",None)
        self.__kernel(e)
    
    def LogState(self,):
        e = FrameEvent("LogState",None)
        self.__kernel(e)
    
    def Inc(self,):
        e = FrameEvent("Inc",None)
        self.__kernel(e)
        return e._return
    
    def Next(self,arg: int):
        parameters = {}
        parameters["arg"] = arg
        e = FrameEvent("Next",parameters)
        self.__kernel(e)
    
    # ===================== Machine Block =================== #
      #  Change [arg:int]
    
    
    # ----------------------------------------
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
            compartment = StateContextSmCompartment('__statecontextsm_state_Foo')
            compartment.enter_args["a"] = 3
            compartment.enter_args["b"] = self.__compartment.state_vars["w"]
            compartment.state_vars["x"] = 0
            self.__transition(compartment)
            return
    
    # ----------------------------------------
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
            compartment = StateContextSmCompartment('__statecontextsm_state_Bar')
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
    
    
    # ----------------------------------------
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
        if self.__compartment.state == '__statecontextsm_state_Init':
            self.__statecontextsm_state_Init(e)
        elif self.__compartment.state == '__statecontextsm_state_Foo':
            self.__statecontextsm_state_Foo(e)
        elif self.__compartment.state == '__statecontextsm_state_Bar':
            self.__statecontextsm_state_Bar(e)
        
    def __transition(self, compartment: 'StateContextSmCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class StateContextSmCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    