#Emitted from framec_v0.11.2



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class StateParams:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = StateParamsCompartment('__stateparams_state_Init', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        self.param_log  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def Next(self,):
        self.return_stack.append(None)
        __e = FrameEvent("Next",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def Prev(self,):
        self.return_stack.append(None)
        __e = FrameEvent("Prev",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def Log(self,):
        self.return_stack.append(None)
        __e = FrameEvent("Log",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __stateparams_state_Init(self, __e, compartment):
        if __e._message == "Next":
            next_compartment = None
            next_compartment = StateParamsCompartment('__stateparams_state_Split', next_compartment)
            next_compartment.state_args["val"] = 1
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $Split
    
    def __stateparams_state_Split(self, __e, compartment):
        if __e._message == "Next":
            next_compartment = None
            next_compartment = StateParamsCompartment('__stateparams_state_Merge', next_compartment)
            next_compartment.state_args["left"] = compartment.state_args["val"]
            next_compartment.state_args["right"] = compartment.state_args["val"] + 1
            self.__transition(next_compartment)
            return
        elif __e._message == "Prev":
            next_compartment = None
            next_compartment = StateParamsCompartment('__stateparams_state_Merge', next_compartment)
            next_compartment.state_args["left"] = compartment.state_args["val"] + 1
            next_compartment.state_args["right"] = compartment.state_args["val"]
            self.__transition(next_compartment)
            return
        elif __e._message == "Log":
            self.got_param_do("val",(compartment.state_args["val"]))
            return
    
    # ----------------------------------------
    # $Merge
    
    def __stateparams_state_Merge(self, __e, compartment):
        if __e._message == "Next":
            next_compartment = None
            next_compartment = StateParamsCompartment('__stateparams_state_Split', next_compartment)
            next_compartment.state_args["val"] = compartment.state_args["left"] + compartment.state_args["right"]
            self.__transition(next_compartment)
            return
        elif __e._message == "Prev":
            next_compartment = None
            next_compartment = StateParamsCompartment('__stateparams_state_Split', next_compartment)
            next_compartment.state_args["val"] = compartment.state_args["left"] * compartment.state_args["right"]
            self.__transition(next_compartment)
            return
        elif __e._message == "Log":
            self.got_param_do("left",(compartment.state_args["left"]))
            self.got_param_do("right",(compartment.state_args["right"]))
            return
    
    # ===================== Actions Block =================== #
    
    def got_param_do(self,name: str,val: int):
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
        if self.__compartment.state == '__stateparams_state_Init':
            self.__stateparams_state_Init(__e, self.__compartment)
        elif self.__compartment.state == '__stateparams_state_Split':
            self.__stateparams_state_Split(__e, self.__compartment)
        elif self.__compartment.state == '__stateparams_state_Merge':
            self.__stateparams_state_Merge(__e, self.__compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class StateParamsCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    