# emitted from framec_v0.10.0
# get include files at https://github.com/frame-lang/frame-ancillary-files
from framelang.framelang import FrameEvent

class VarScope:
    
    def __init__(self):
        
        # Create and intialize start state compartment.
        self.__state = self.__varscope_state_Init
        self.__compartment: 'VarScopeCompartment' = VarScopeCompartment(self.__state)
        self.__next_compartment: 'VarScopeCompartment' = None
        
        # Initialize domain
        
        self.a : str = "#.a"
        self.x : str = "#.x"
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def to_nn(self,):
        e = FrameEvent("to_nn",None)
        self.__mux(e)
    
    def to_ny(self,):
        e = FrameEvent("to_ny",None)
        self.__mux(e)
    
    def to_yn(self,):
        e = FrameEvent("to_yn",None)
        self.__mux(e)
    
    def to_yy(self,):
        e = FrameEvent("to_yy",None)
        self.__mux(e)
    
    def nn(self,d: str):
        parameters = {}
        parameters["d"] = d

        e = FrameEvent("nn",parameters)
        self.__mux(e)
    
    def ny(self,d: str):
        parameters = {}
        parameters["d"] = d

        e = FrameEvent("ny",parameters)
        self.__mux(e)
    
    def yn(self,d: str,x: str):
        parameters = {}
        parameters["d"] = d

        parameters["x"] = x

        e = FrameEvent("yn",parameters)
        self.__mux(e)
    
    def yy(self,d: str,x: str):
        parameters = {}
        parameters["d"] = d

        parameters["x"] = x

        e = FrameEvent("yy",parameters)
        self.__mux(e)
    
    def sigils(self,x: str):
        parameters = {}
        parameters["x"] = x

        e = FrameEvent("sigils",parameters)
        self.__mux(e)
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__varscope_state_Init':
            self.__varscope_state_Init(e)
        elif self.__compartment.state.__name__ == '__varscope_state_NN':
            self.__varscope_state_NN(e)
        elif self.__compartment.state.__name__ == '__varscope_state_NY':
            self.__varscope_state_NY(e)
        elif self.__compartment.state.__name__ == '__varscope_state_YN':
            self.__varscope_state_YN(e)
        elif self.__compartment.state.__name__ == '__varscope_state_YY':
            self.__varscope_state_YY(e)
        
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
    
    def __varscope_state_Init(self, e):
        if e._message == "to_nn":
            compartment = VarScopeCompartment(self.__varscope_state_NN)
            compartment.state_args["b"] = "$NN[b]"
            compartment.state_vars["c"] = "$NN.c"
            self.__transition(compartment)
            
            return
        
        elif e._message == "to_ny":
            compartment = VarScopeCompartment(self.__varscope_state_NY)
            compartment.state_args["b"] = "$NY[b]"
            compartment.state_vars["c"] = "$NY.c"
            compartment.state_vars["x"] = "$NY.x"
            self.__transition(compartment)
            
            return
        
        elif e._message == "to_yn":
            compartment = VarScopeCompartment(self.__varscope_state_YN)
            compartment.state_args["b"] = "$YN[b]"
            compartment.state_args["x"] = "$YN[x]"
            compartment.state_vars["c"] = "$YN.c"
            self.__transition(compartment)
            
            return
        
        elif e._message == "to_yy":
            compartment = VarScopeCompartment(self.__varscope_state_YY)
            compartment.state_args["b"] = "$YY[b]"
            compartment.state_args["x"] = "$YY[x]"
            compartment.state_vars["c"] = "$YY.c"
            compartment.state_vars["x"] = "$YY.x"
            self.__transition(compartment)
            
            return
        
    def __varscope_state_NN(self, e):
        if e._message == "nn":
            et : str = "|nn|.e"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(self.x)
            
            return
        
        elif e._message == "ny":
            et : str = "|ny|.e"
            x : str = "|ny|.x"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            
            return
        
        elif e._message == "yn":
            et : str = "|yn|.e"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(e._parameters["x"])
            
            return
        
        elif e._message == "yy":
            et : str = "|yy|.e"
            x : str = "|yy|.x"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            
            return
        
        elif e._message == "sigils":
            self.log_do(self.x)
            
            return
        
      #  var x:str = "|sigils|.x"
      #  log(||[x])
      #  log(||.x)
    
    def __varscope_state_NY(self, e):
        if e._message == "nn":
            et : str = "|nn|.e"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do((self.__compartment.state_vars["x"]))
            
            return
        
        elif e._message == "ny":
            et : str = "|ny|.e"
            x : str = "|ny|.x"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            
            return
        
        elif e._message == "yn":
            et : str = "|yn|.e"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(e._parameters["x"])
            
            return
        
        elif e._message == "yy":
            et : str = "|yy|.e"
            x : str = "|yy|.x"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            
            return
        
        elif e._message == "sigils":
            self.log_do(self.x)
            
            return
        
      #  var x:str = "|sigils|.x"
      #  log($.x)
      #  log(||[x])
      #  log(||.x)
    
    def __varscope_state_YN(self, e):
        if e._message == "nn":
            et : str = "|nn|.e"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do((self.__compartment.state_args["x"]))
            
            return
        
        elif e._message == "ny":
            et : str = "|ny|.e"
            x : str = "|ny|.x"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            
            return
        
        elif e._message == "yn":
            et : str = "|yn|.e"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(e._parameters["x"])
            
            return
        
        elif e._message == "yy":
            et : str = "|yy|.e"
            x : str = "|yy|.x"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            
            return
        
        elif e._message == "sigils":
            self.log_do(self.x)
            
            return
        
      #  var x:str = "|sigils|.x"
      #  log($[x])
      #  log(||[x])
      #  log(||.x)
    
    def __varscope_state_YY(self, e):
        if e._message == "nn":
            et : str = "|nn|.e"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do((self.__compartment.state_vars["x"]))
            
            return
        
        elif e._message == "ny":
            et : str = "|ny|.e"
            x : str = "|ny|.x"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            
            return
        
        elif e._message == "yn":
            et : str = "|yn|.e"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(e._parameters["x"])
            
            return
        
        elif e._message == "yy":
            et : str = "|yy|.e"
            x : str = "|yy|.x"
            self.log_do(self.a)
            self.log_do((self.__compartment.state_args["b"]))
            self.log_do((self.__compartment.state_vars["c"]))
            self.log_do(e._parameters["d"])
            self.log_do(et)
            self.log_do(x)
            
            return
        
        elif e._message == "sigils":
            self.log_do(self.x)
            
            return
        
    
    # ===================== Actions Block =================== #
    
    
    # Unimplemented Actions
    
    def log_do(self,s: str):
        raise NotImplementedError
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'VarScopeCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'VarScopeCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class VarScopeCompartment:

    def __init__(self, state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class VarScopeController(VarScope):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,s: str):
        #pass

# ********************
