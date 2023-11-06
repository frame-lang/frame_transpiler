# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class EventHandler:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__eventhandler_state_S1
        self.__compartment: 'EventHandlerCompartment' = EventHandlerCompartment(self.__state)
        self.__next_compartment: 'EventHandlerCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ===================== Interface Block =================== #
    
    def LogIt(self,x: int):
        parameters = {}
        parameters["x"] = x

        e = FrameEvent("LogIt",parameters)
        self.__kernel(e)
    
    def LogAdd(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a

        parameters["b"] = b

        e = FrameEvent("LogAdd",parameters)
        self.__kernel(e)
    
    def LogReturn(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a

        parameters["b"] = b

        e = FrameEvent("LogReturn",parameters)
        self.__kernel(e)
        return e._return
    
    def PassAdd(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a

        parameters["b"] = b

        e = FrameEvent("PassAdd",parameters)
        self.__kernel(e)
    
    def PassReturn(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a

        parameters["b"] = b

        e = FrameEvent("PassReturn",parameters)
        self.__kernel(e)
        return e._return
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $S1
    
    def __eventhandler_state_S1(self, e):
        if e._message == "LogIt":
            self.log_do("x",e._parameters["x"])
            return
        elif e._message == "LogAdd":
            self.log_do("a",e._parameters["a"])
            self.log_do("b",e._parameters["b"])
            self.log_do("a+b",e._parameters["a"] + e._parameters["b"])
            return
        elif e._message == "LogReturn":
            self.log_do("a",e._parameters["a"])
            self.log_do("b",e._parameters["b"])
            r  = e._parameters["a"] + e._parameters["b"]
            self.log_do("r",r)
            e._return = r
            return
            
        elif e._message == "PassAdd":
            compartment = EventHandlerCompartment(self.__eventhandler_state_S2)
            compartment.state_args["p"] = e._parameters["a"] + e._parameters["b"]
            self.__transition(compartment)
            return
        elif e._message == "PassReturn":
            r  = e._parameters["a"] + e._parameters["b"]
            self.log_do("r",r)
            compartment = EventHandlerCompartment(self.__eventhandler_state_S2)
            compartment.state_args["p"] = r
            self.__transition(compartment)
            e._return = r
            return
            
    
    # ----------------------------------------
    # $S2
    
    def __eventhandler_state_S2(self, e):
        if e._message == ">":
            self.log_do("p",(self.__compartment.state_args["p"]))
            return
    
    # ===================== Actions Block =================== #
    
    def log_do(self,msg: str,val: int):
        raise NotImplementedError
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, e):
        
        self.__router(e)
        
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
        if self.__compartment.state.__name__ == '__eventhandler_state_S1':
            self.__eventhandler_state_S1(e)
        elif self.__compartment.state.__name__ == '__eventhandler_state_S2':
            self.__eventhandler_state_S2(e)
        
    def __transition(self, compartment: 'EventHandlerCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'EventHandlerCompartment'):
        self.__router(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__router(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class EventHandlerCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    
# ********************

#class EventHandlerController(EventHandler):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,msg: str,val: int):
        #pass

# ********************

