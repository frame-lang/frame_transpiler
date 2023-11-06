# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class HandlerCalls:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__handlercalls_state_Init
        self.__compartment: 'HandlerCallsCompartment' = HandlerCallsCompartment(self.__state)
        self.__next_compartment: 'HandlerCallsCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ===================== Interface Block =================== #
    
    def NonRec(self,):
        e = FrameEvent("NonRec",None)
        self.__kernel(e)
    
    def SelfRec(self,):
        e = FrameEvent("SelfRec",None)
        self.__kernel(e)
    
    def MutRec(self,):
        e = FrameEvent("MutRec",None)
        self.__kernel(e)
    
    def Call(self,event: str,arg: int):
        parameters = {}
        parameters["event"] = event

        parameters["arg"] = arg

        e = FrameEvent("Call",parameters)
        self.__kernel(e)
    
    def Foo(self,arg: int):
        parameters = {}
        parameters["arg"] = arg

        e = FrameEvent("Foo",parameters)
        self.__kernel(e)
    
    def Bar(self,arg: int):
        parameters = {}
        parameters["arg"] = arg

        e = FrameEvent("Bar",parameters)
        self.__kernel(e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __handlercalls_state_Init(self, e):
        if e._message == "NonRec":
            compartment = HandlerCallsCompartment(self.__handlercalls_state_NonRecursive)
            compartment.state_vars["counter"] = 0
            self.__transition(compartment)
            return
        elif e._message == "SelfRec":
            compartment = HandlerCallsCompartment(self.__handlercalls_state_SelfRecursive)
            compartment.state_vars["counter"] = 0
            self.__transition(compartment)
            return
        elif e._message == "MutRec":
            compartment = HandlerCallsCompartment(self.__handlercalls_state_MutuallyRecursive)
            compartment.state_vars["counter"] = 0
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $NonRecursive
    
    def __handlercalls_state_NonRecursive(self, e):
        if e._message == "Foo":
            self.log_do("Foo",e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + e._parameters["arg"]
            self.Bar(e._parameters["arg"] * 2)
            return
            self.log_do("Unreachable",0)
            return
          #  the front-end should report the next line as a static error
        elif e._message == "Bar":
            self.log_do("Bar",e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + e._parameters["arg"]
            compartment = HandlerCallsCompartment(self.__handlercalls_state_Final)
            compartment.state_args["counter"] = self.__compartment.state_vars["counter"]
            self.__transition(compartment)
            return
        elif e._message == "Call":
            if ((e._parameters["event"] == "Foo")):
                self.Foo(e._parameters["arg"])
                return
            elif ((e._parameters["event"] == "Bar")):
                self.Bar(e._parameters["arg"])
                return
            else:
                self.Call("Foo",1000)
                return
            
            return
    
    # ----------------------------------------
    # $SelfRecursive
    
    def __handlercalls_state_SelfRecursive(self, e):
        if e._message == "Foo":
            self.log_do("Foo",e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + e._parameters["arg"]
            if  (self.__compartment.state_vars["counter"]) < 100:
                self.Foo(e._parameters["arg"] * 2)
                return
            else:
                compartment = HandlerCallsCompartment(self.__handlercalls_state_Final)
                compartment.state_args["counter"] = self.__compartment.state_vars["counter"]
                self.__transition(compartment)
                return
            
            return
        elif e._message == "Bar":
            self.log_do("Bar",e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + e._parameters["arg"]
            compartment = HandlerCallsCompartment(self.__handlercalls_state_Final)
            compartment.state_args["counter"] = self.__compartment.state_vars["counter"]
            self.__transition(compartment)
            return
        elif e._message == "Call":
            if ((e._parameters["event"] == "Foo")):
                self.Foo(e._parameters["arg"])
                return
            elif ((e._parameters["event"] == "Bar")):
                self.Bar(e._parameters["arg"])
                return
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $MutuallyRecursive
    
    def __handlercalls_state_MutuallyRecursive(self, e):
        if e._message == "Foo":
            self.log_do("Foo",e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + e._parameters["arg"]
            if  (self.__compartment.state_vars["counter"]) > 100:
                compartment = HandlerCallsCompartment(self.__handlercalls_state_Final)
                compartment.state_args["counter"] = self.__compartment.state_vars["counter"]
                self.__transition(compartment)
                return
            else:
                self.Bar(e._parameters["arg"] * 2)
                return
            
            return
        elif e._message == "Bar":
            self.log_do("Bar",e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + e._parameters["arg"]
            if (e._parameters["arg"] == 4):
                self.Foo(e._parameters["arg"])
                return
            elif (e._parameters["arg"] == 8):
                self.Foo(e._parameters["arg"] * 2)
                return
            else:
                self.Foo(e._parameters["arg"] * 3)
                return
            
            return
        elif e._message == "Call":
            if ((e._parameters["event"] == "Foo")):
                self.Foo(e._parameters["arg"])
                return
            elif ((e._parameters["event"] == "Bar")):
                self.Bar(e._parameters["arg"])
                return
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $Final
    
    def __handlercalls_state_Final(self, e):
        if e._message == ">":
            self.log_do("Final",(self.__compartment.state_args["counter"]))
            compartment = HandlerCallsCompartment(self.__handlercalls_state_Init)
            self.__transition(compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def log_do(self,through: str,val: int):
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
        if self.__compartment.state.__name__ == '__handlercalls_state_Init':
            self.__handlercalls_state_Init(e)
        elif self.__compartment.state.__name__ == '__handlercalls_state_NonRecursive':
            self.__handlercalls_state_NonRecursive(e)
        elif self.__compartment.state.__name__ == '__handlercalls_state_SelfRecursive':
            self.__handlercalls_state_SelfRecursive(e)
        elif self.__compartment.state.__name__ == '__handlercalls_state_MutuallyRecursive':
            self.__handlercalls_state_MutuallyRecursive(e)
        elif self.__compartment.state.__name__ == '__handlercalls_state_Final':
            self.__handlercalls_state_Final(e)
        
    def __transition(self, compartment: 'HandlerCallsCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class HandlerCallsCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    
# ********************

#class HandlerCallsController(HandlerCalls):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,through: str,val: int):
        #pass

# ********************

