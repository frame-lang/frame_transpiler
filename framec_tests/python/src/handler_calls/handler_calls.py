#Emitted from framec_v0.11.0



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
        
        self.__compartment: 'HandlerCallsCompartment' = HandlerCallsCompartment('__handlercalls_state_Init')
        self.__next_compartment: 'HandlerCallsCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def NonRec(self,):
        __e = FrameEvent("NonRec",None)
        self.__kernel(__e)
    
    def SelfRec(self,):
        __e = FrameEvent("SelfRec",None)
        self.__kernel(__e)
    
    def MutRec(self,):
        __e = FrameEvent("MutRec",None)
        self.__kernel(__e)
    
    def Call(self,event: str,arg: int):
        parameters = {}
        parameters["event"] = event
        parameters["arg"] = arg
        __e = FrameEvent("Call",parameters)
        self.__kernel(__e)
    
    def Foo(self,arg: int):
        parameters = {}
        parameters["arg"] = arg
        __e = FrameEvent("Foo",parameters)
        self.__kernel(__e)
    
    def Bar(self,arg: int):
        parameters = {}
        parameters["arg"] = arg
        __e = FrameEvent("Bar",parameters)
        self.__kernel(__e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __handlercalls_state_Init(self, __e):
        if __e._message == "NonRec":
            next_compartment = HandlerCallsCompartment('__handlercalls_state_NonRecursive')
            next_compartment.state_vars["counter"] = 0
            self.__transition(next_compartment)
            return
        elif __e._message == "SelfRec":
            next_compartment = HandlerCallsCompartment('__handlercalls_state_SelfRecursive')
            next_compartment.state_vars["counter"] = 0
            self.__transition(next_compartment)
            return
        elif __e._message == "MutRec":
            next_compartment = HandlerCallsCompartment('__handlercalls_state_MutuallyRecursive')
            next_compartment.state_vars["counter"] = 0
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $NonRecursive
    
    def __handlercalls_state_NonRecursive(self, __e):
        if __e._message == "Foo":
            self.log_do("Foo",__e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + __e._parameters["arg"]
            self.Bar(__e._parameters["arg"] * 2)
            return
            self.log_do("Unreachable",0)
            return
          #  the front-end should report the next line as a static error
        elif __e._message == "Bar":
            self.log_do("Bar",__e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + __e._parameters["arg"]
            next_compartment = HandlerCallsCompartment('__handlercalls_state_Final')
            next_compartment.state_args["counter"] = self.__compartment.state_vars["counter"]
            self.__transition(next_compartment)
            return
        elif __e._message == "Call":
            if ((__e._parameters["event"] == "Foo")):
                self.Foo(__e._parameters["arg"])
                return
            elif ((__e._parameters["event"] == "Bar")):
                self.Bar(__e._parameters["arg"])
                return
            
            else:
                self.Call("Foo",1000)
                return
            
            return
    
    # ----------------------------------------
    # $SelfRecursive
    
    def __handlercalls_state_SelfRecursive(self, __e):
        if __e._message == "Foo":
            self.log_do("Foo",__e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + __e._parameters["arg"]
            if  (self.__compartment.state_vars["counter"]) < 100:
                self.Foo(__e._parameters["arg"] * 2)
                return
            else:
                next_compartment = HandlerCallsCompartment('__handlercalls_state_Final')
                next_compartment.state_args["counter"] = self.__compartment.state_vars["counter"]
                self.__transition(next_compartment)
                return
            
            return
        elif __e._message == "Bar":
            self.log_do("Bar",__e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + __e._parameters["arg"]
            next_compartment = HandlerCallsCompartment('__handlercalls_state_Final')
            next_compartment.state_args["counter"] = self.__compartment.state_vars["counter"]
            self.__transition(next_compartment)
            return
        elif __e._message == "Call":
            if ((__e._parameters["event"] == "Foo")):
                self.Foo(__e._parameters["arg"])
                return
            elif ((__e._parameters["event"] == "Bar")):
                self.Bar(__e._parameters["arg"])
                return
            
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $MutuallyRecursive
    
    def __handlercalls_state_MutuallyRecursive(self, __e):
        if __e._message == "Foo":
            self.log_do("Foo",__e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + __e._parameters["arg"]
            if  (self.__compartment.state_vars["counter"]) > 100:
                next_compartment = HandlerCallsCompartment('__handlercalls_state_Final')
                next_compartment.state_args["counter"] = self.__compartment.state_vars["counter"]
                self.__transition(next_compartment)
                return
            else:
                self.Bar(__e._parameters["arg"] * 2)
                return
            
            return
        elif __e._message == "Bar":
            self.log_do("Bar",__e._parameters["arg"])
            (self.__compartment.state_vars["counter"]) = self.__compartment.state_vars["counter"] + __e._parameters["arg"]
            if (__e._parameters["arg"] == 4):
                self.Foo(__e._parameters["arg"])
                return
            elif (__e._parameters["arg"] == 8):
                self.Foo(__e._parameters["arg"] * 2)
                return
            
            else:
                self.Foo(__e._parameters["arg"] * 3)
                return
            
            return
        elif __e._message == "Call":
            if ((__e._parameters["event"] == "Foo")):
                self.Foo(__e._parameters["arg"])
                return
            elif ((__e._parameters["event"] == "Bar")):
                self.Bar(__e._parameters["arg"])
                return
            
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $Final
    
    def __handlercalls_state_Final(self, __e):
        if __e._message == ">":
            self.log_do("Final",(self.__compartment.state_args["counter"]))
            next_compartment = HandlerCallsCompartment('__handlercalls_state_Init')
            self.__transition(next_compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def log_do(self,through: str,val: int):
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
        if self.__compartment.state == '__handlercalls_state_Init':
            self.__handlercalls_state_Init(__e)
        elif self.__compartment.state == '__handlercalls_state_NonRecursive':
            self.__handlercalls_state_NonRecursive(__e)
        elif self.__compartment.state == '__handlercalls_state_SelfRecursive':
            self.__handlercalls_state_SelfRecursive(__e)
        elif self.__compartment.state == '__handlercalls_state_MutuallyRecursive':
            self.__handlercalls_state_MutuallyRecursive(__e)
        elif self.__compartment.state == '__handlercalls_state_Final':
            self.__handlercalls_state_Final(__e)
        
    def __transition(self, next_compartment: 'HandlerCallsCompartment'):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class HandlerCallsCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    