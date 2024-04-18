#Emitted from framec_v0.11.2



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


class TransitParams:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = TransitParamsCompartment('__transitparams_state_Init', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def Next(self,):
        self.return_stack.append(None)
        __e = FrameEvent("Next",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __transitparams_state_Init(self, __e, compartment):
        if __e._message == "Next":
            next_compartment = None
            next_compartment = TransitParamsCompartment('__transitparams_state_A', next_compartment)
            next_compartment.enter_args["msg"] = "hi A"
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $A
    
    def __transitparams_state_A(self, __e, compartment):
        if __e._message == ">":
            self.log_do(__e._parameters["msg"])
            return
        elif __e._message == "<":
            self.log_do("bye A")
            return
        elif __e._message == "Next":
            next_compartment = None
            next_compartment = TransitParamsCompartment('__transitparams_state_B', next_compartment)
            next_compartment.enter_args["msg"] = "hi B"
            next_compartment.enter_args["val"] = 42
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $B
    
    def __transitparams_state_B(self, __e, compartment):
        if __e._message == ">":
            self.log_do(__e._parameters["msg"])
            self.log_do(str(__e._parameters["val"]))
            return
        elif __e._message == "<":
            self.log_do(str(__e._parameters["val"]))
            self.log_do(__e._parameters["msg"])
            return
        elif __e._message == "Next":
            self.__compartment.exit_args["val"] = True
            self.__compartment.exit_args["msg"] = "bye B"
            next_compartment = None
            next_compartment = TransitParamsCompartment('__transitparams_state_A', next_compartment)
            next_compartment.enter_args["msg"] = "hi again A"
            self.__transition(next_compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def log_do(self,msg: str):
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
        if self.__compartment.state == '__transitparams_state_Init':
            self.__transitparams_state_Init(__e, self.__compartment)
        elif self.__compartment.state == '__transitparams_state_A':
            self.__transitparams_state_A(__e, self.__compartment)
        elif self.__compartment.state == '__transitparams_state_B':
            self.__transitparams_state_B(__e, self.__compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class TransitParamsCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    