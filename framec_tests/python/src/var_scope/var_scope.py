#Emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class VarScope:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = VarScopeCompartment('__varscope_state_Init', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        
        # Initialize domain
        
        self.a : str = "#.a"
        self.x : str = "#.x"
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def to_nn(self,):
        __e = FrameEvent("to_nn",None)
        self.__kernel(__e)
    
    def to_ny(self,):
        __e = FrameEvent("to_ny",None)
        self.__kernel(__e)
    
    def to_yn(self,):
        __e = FrameEvent("to_yn",None)
        self.__kernel(__e)
    
    def to_yy(self,):
        __e = FrameEvent("to_yy",None)
        self.__kernel(__e)
    
    def nn(self,d: str):
        parameters = {}
        parameters["d"] = d
        __e = FrameEvent("nn",parameters)
        self.__kernel(__e)
    
    def ny(self,d: str):
        parameters = {}
        parameters["d"] = d
        __e = FrameEvent("ny",parameters)
        self.__kernel(__e)
    
    def yn(self,d: str,x: str):
        parameters = {}
        parameters["d"] = d
        parameters["x"] = x
        __e = FrameEvent("yn",parameters)
        self.__kernel(__e)
    
    def yy(self,d: str,x: str):
        parameters = {}
        parameters["d"] = d
        parameters["x"] = x
        __e = FrameEvent("yy",parameters)
        self.__kernel(__e)
    
    def sigils(self,x: str):
        parameters = {}
        parameters["x"] = x
        __e = FrameEvent("sigils",parameters)
        self.__kernel(__e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __varscope_state_Init(self, __e, compartment):
        if __e._message == "to_nn":
            next_compartment = None
            next_compartment = VarScopeCompartment('__varscope_state_NN', next_compartment)
            next_compartment.state_args["b"] = "$NN[b]"
            next_compartment.state_vars["c"] = "$NN.c"
            self.__transition(next_compartment)
            return
        elif __e._message == "to_ny":
            next_compartment = None
            next_compartment = VarScopeCompartment('__varscope_state_NY', next_compartment)
            next_compartment.state_args["b"] = "$NY[b]"
            next_compartment.state_vars["c"] = "$NY.c"
            next_compartment.state_vars["x"] = "$NY.x"
            self.__transition(next_compartment)
            return
        elif __e._message == "to_yn":
            next_compartment = None
            next_compartment = VarScopeCompartment('__varscope_state_YN', next_compartment)
            next_compartment.state_args["b"] = "$YN[b]"
            next_compartment.state_args["x"] = "$YN[x]"
            next_compartment.state_vars["c"] = "$YN.c"
            self.__transition(next_compartment)
            return
        elif __e._message == "to_yy":
            next_compartment = None
            next_compartment = VarScopeCompartment('__varscope_state_YY', next_compartment)
            next_compartment.state_args["b"] = "$YY[b]"
            next_compartment.state_args["x"] = "$YY[x]"
            next_compartment.state_vars["c"] = "$YY.c"
            next_compartment.state_vars["x"] = "$YY.x"
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $NN
    
    def __varscope_state_NN(self, __e, compartment):
        if __e._message == "nn":
            et: str = "|nn|.e"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(self.x)
            return
        elif __e._message == "ny":
            et: str = "|ny|.e"
            x: str = "|ny|.x"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            return
        elif __e._message == "yn":
            et: str = "|yn|.e"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(__e._parameters["x"])
            return
        elif __e._message == "yy":
            et: str = "|yy|.e"
            x: str = "|yy|.x"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            return
        elif __e._message == "sigils":
            self.log_do(self.x)
            return
      #  var x:str = "|sigils|.x"
      #  log(||[x])
      #  log(||.x)
    
    
    # ----------------------------------------
    # $NY
    
    def __varscope_state_NY(self, __e, compartment):
        if __e._message == "nn":
            et: str = "|nn|.e"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do((compartment.state_vars["x"]))
            return
        elif __e._message == "ny":
            et: str = "|ny|.e"
            x: str = "|ny|.x"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            return
        elif __e._message == "yn":
            et: str = "|yn|.e"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(__e._parameters["x"])
            return
        elif __e._message == "yy":
            et: str = "|yy|.e"
            x: str = "|yy|.x"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            return
        elif __e._message == "sigils":
            self.log_do(self.x)
            return
      #  var x:str = "|sigils|.x"
      #  log($.x)
      #  log(||[x])
      #  log(||.x)
    
    
    # ----------------------------------------
    # $YN
    
    def __varscope_state_YN(self, __e, compartment):
        if __e._message == "nn":
            et: str = "|nn|.e"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do((compartment.state_args["x"]))
            return
        elif __e._message == "ny":
            et: str = "|ny|.e"
            x: str = "|ny|.x"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            return
        elif __e._message == "yn":
            et: str = "|yn|.e"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(__e._parameters["x"])
            return
        elif __e._message == "yy":
            et: str = "|yy|.e"
            x: str = "|yy|.x"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            return
        elif __e._message == "sigils":
            self.log_do(self.x)
            return
      #  var x:str = "|sigils|.x"
      #  log($[x])
      #  log(||[x])
      #  log(||.x)
    
    
    # ----------------------------------------
    # $YY
    
    def __varscope_state_YY(self, __e, compartment):
        if __e._message == "nn":
            et: str = "|nn|.e"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do((compartment.state_vars["x"]))
            return
        elif __e._message == "ny":
            et: str = "|ny|.e"
            x: str = "|ny|.x"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            return
        elif __e._message == "yn":
            et: str = "|yn|.e"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(__e._parameters["x"])
            return
        elif __e._message == "yy":
            et: str = "|yy|.e"
            x: str = "|yy|.x"
            self.log_do(self.a)
            self.log_do((compartment.state_args["b"]))
            self.log_do((compartment.state_vars["c"]))
            self.log_do(__e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            return
        elif __e._message == "sigils":
            self.log_do(self.x)
            return
    
    # ===================== Actions Block =================== #
    
    def log_do(self,s: str):
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
        if self.__compartment.state == '__varscope_state_Init':
            self.__varscope_state_Init(__e, self.__compartment)
        elif self.__compartment.state == '__varscope_state_NN':
            self.__varscope_state_NN(__e, self.__compartment)
        elif self.__compartment.state == '__varscope_state_NY':
            self.__varscope_state_NY(__e, self.__compartment)
        elif self.__compartment.state == '__varscope_state_YN':
            self.__varscope_state_YN(__e, self.__compartment)
        elif self.__compartment.state == '__varscope_state_YY':
            self.__varscope_state_YY(__e, self.__compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class VarScopeCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    