#Emitted from framec_v0.11.2



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class StateContextSm:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = StateContextSmCompartment('__statecontextsm_state_Init', next_compartment)
        next_compartment.state_vars["w"] = 0
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = []
        next_compartment.state_vars["w"] = 0
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def Start(self,):
        self.return_stack.append(None)
        __e = FrameEvent("Start",None)
        self.__kernel(__e)
        self.return_stack.pop(-1)
    
    def LogState(self,):
        self.return_stack.append(None)
        __e = FrameEvent("LogState",None)
        self.__kernel(__e)
        self.return_stack.pop(-1)
    
    def Inc(self,):
        self.return_stack.append(None)
        __e = FrameEvent("Inc",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def Next(self,arg: int):
        parameters = {}
        parameters["arg"] = arg
        self.return_stack.append(None)
        __e = FrameEvent("Next",parameters)
        self.__kernel(__e)
        self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
      #  Change [arg:int]
    
    
    # ----------------------------------------
    # $Init
    
    def __statecontextsm_state_Init(self, __e, compartment):
        if __e._message == ">":
            (compartment.state_vars["w"]) = 3
            self.log_do("w",(compartment.state_vars["w"]))
            return
        elif __e._message == "Inc":
            (compartment.state_vars["w"]) = compartment.state_vars["w"] + 1
            self.log_do("w",(compartment.state_vars["w"]))
            self.return_stack[-1] = (compartment.state_vars["w"])
            return
            
        elif __e._message == "LogState":
            self.log_do("w",(compartment.state_vars["w"]))
            return
        elif __e._message == "Start":
            next_compartment = None
            next_compartment = StateContextSmCompartment('__statecontextsm_state_Foo', next_compartment)
            next_compartment.enter_args["a"] = 3
            next_compartment.enter_args["b"] = compartment.state_vars["w"]
            next_compartment.state_vars["x"] = 0
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $Foo
    
    def __statecontextsm_state_Foo(self, __e, compartment):
        if __e._message == ">":
            self.log_do("a",__e._parameters["a"])
            self.log_do("b",__e._parameters["b"])
            (compartment.state_vars["x"]) = __e._parameters["a"] * __e._parameters["b"]
            self.log_do("x",(compartment.state_vars["x"]))
            return
        elif __e._message == "<":
            self.log_do("c",__e._parameters["c"])
            (compartment.state_vars["x"]) = compartment.state_vars["x"] + __e._parameters["c"]
            self.log_do("x",(compartment.state_vars["x"]))
            return
        elif __e._message == "LogState":
            self.log_do("x",(compartment.state_vars["x"]))
            return
        elif __e._message == "Inc":
            (compartment.state_vars["x"]) = compartment.state_vars["x"] + 1
            self.log_do("x",(compartment.state_vars["x"]))
            self.return_stack[-1] = (compartment.state_vars["x"])
            return
            
        elif __e._message == "Next":
            tmp = __e._parameters["arg"] * 10
            self.__compartment.exit_args["c"] = 10
            next_compartment = None
            next_compartment = StateContextSmCompartment('__statecontextsm_state_Bar', next_compartment)
            next_compartment.enter_args["a"] = tmp
            next_compartment.state_args["y"] = compartment.state_vars["x"]
            next_compartment.state_vars["z"] = 0
            self.__transition(next_compartment)
            return
      #  FIXME: Swapping this to 10 * arg causes a parse error!
      #  |Change| [arg:int]
      #      var tmp = x + arg
      #      -> $Bar(tmp)
      #      ^
    
    
    # ----------------------------------------
    # $Bar
    
    def __statecontextsm_state_Bar(self, __e, compartment):
        if __e._message == ">":
            self.log_do("a",__e._parameters["a"])
            self.log_do("y",(compartment.state_args["y"]))
            (compartment.state_vars["z"]) = __e._parameters["a"] + compartment.state_args["y"]
            self.log_do("z",(compartment.state_vars["z"]))
            return
        elif __e._message == "LogState":
            self.log_do("y",(compartment.state_args["y"]))
            self.log_do("z",(compartment.state_vars["z"]))
            return
        elif __e._message == "Inc":
            (compartment.state_vars["z"]) = compartment.state_vars["z"] + 1
            self.log_do("z",(compartment.state_vars["z"]))
            self.return_stack[-1] = (compartment.state_vars["z"])
            return
            
    
    # ===================== Actions Block =================== #
    
    def log_do(self,name: str,val: int):
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
        if self.__compartment.state == '__statecontextsm_state_Init':
            self.__statecontextsm_state_Init(__e, self.__compartment)
        elif self.__compartment.state == '__statecontextsm_state_Foo':
            self.__statecontextsm_state_Foo(__e, self.__compartment)
        elif self.__compartment.state == '__statecontextsm_state_Bar':
            self.__statecontextsm_state_Bar(__e, self.__compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class StateContextSmCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    