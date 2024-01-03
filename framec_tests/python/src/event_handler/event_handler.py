



from framelang.framelang import FrameEvent


# Emitted from framec_v0.11.0

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class EventHandler:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__compartment: 'EventHandlerCompartment' = EventHandlerCompartment('__eventhandler_state_S1')
        self.__next_compartment: 'EventHandlerCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def LogIt(self,x: int):
        parameters = {}
        parameters["x"] = x
        __e = FrameEvent("LogIt",parameters)
        self.__kernel(__e)
    
    def LogAdd(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a
        parameters["b"] = b
        __e = FrameEvent("LogAdd",parameters)
        self.__kernel(__e)
    
    def LogReturn(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a
        parameters["b"] = b
        __e = FrameEvent("LogReturn",parameters)
        self.__kernel(__e)
        return __e._return
    
    def PassAdd(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a
        parameters["b"] = b
        __e = FrameEvent("PassAdd",parameters)
        self.__kernel(__e)
    
    def PassReturn(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a
        parameters["b"] = b
        __e = FrameEvent("PassReturn",parameters)
        self.__kernel(__e)
        return __e._return
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $S1
    
    def __eventhandler_state_S1(self, __e):
        if __e._message == "LogIt":
            self.log_do("x",__e._parameters["x"])
            return
        elif __e._message == "LogAdd":
            self.log_do("a",__e._parameters["a"])
            self.log_do("b",__e._parameters["b"])
            self.log_do("a+b",__e._parameters["a"] + __e._parameters["b"])
            return
        elif __e._message == "LogReturn":
            self.log_do("a",__e._parameters["a"])
            self.log_do("b",__e._parameters["b"])
            r = __e._parameters["a"] + __e._parameters["b"]
            self.log_do("r",r)
            __e._return = r
            return
            
        elif __e._message == "PassAdd":
            compartment = EventHandlerCompartment('__eventhandler_state_S2')
            compartment.state_args["p"] = __e._parameters["a"] + __e._parameters["b"]
            self.__transition(compartment)
            return
        elif __e._message == "PassReturn":
            r = __e._parameters["a"] + __e._parameters["b"]
            self.log_do("r",r)
            compartment = EventHandlerCompartment('__eventhandler_state_S2')
            compartment.state_args["p"] = r
            self.__transition(compartment)
            __e._return = r
            return
            
    
    # ----------------------------------------
    # $S2
    
    def __eventhandler_state_S2(self, __e):
        if __e._message == ">":
            self.log_do("p",(self.__compartment.state_args["p"]))
            return
    
    # ===================== Actions Block =================== #
    
    def log_do(self,msg: str,val: int):
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
        if self.__compartment.state == '__eventhandler_state_S1':
            self.__eventhandler_state_S1(__e)
        elif self.__compartment.state == '__eventhandler_state_S2':
            self.__eventhandler_state_S2(__e)
        
    def __transition(self, compartment: 'EventHandlerCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class EventHandlerCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    