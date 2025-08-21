#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    sys = SystemInitializationDemo("a","b","c","d","e","f")
    return

class SystemInitializationDemo:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self,start_state_state_param_A,start_state_state_param_B,start_state_enter_param_C,start_state_enter_param_D,domain_param_E,domain_param_F):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = SystemInitializationDemoCompartment('__systeminitializationdemo_state_Start', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        self.__compartment.state_args["A"] = start_state_state_param_A
        self.__compartment.state_args["B"] = start_state_state_param_B
        self.__compartment.enter_args["C"] = start_state_enter_param_C
        self.__compartment.enter_args["D"] = start_state_enter_param_D
        
        # Initialize domain
        
        self.E  = domain_param_E
        self.F  = domain_param_F
        
        # Send system start event
        frame_event = FrameEvent("$>", self.__compartment.enter_args)
        self.__kernel(frame_event)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __systeminitializationdemo_state_Start(self, __e, compartment):
        if __e._message == "$>":
            print((compartment.state_args["A"]) + (compartment.state_args["B"]) + __e._parameters["C"] + __e._parameters["D"] + self.E + self.F)
            return
    
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent( "<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else: # there is a forwarded event
                if next_compartment.forward_event._message == "$>":
                    # forwarded event is enter event
                    self.__router(next_compartment.forward_event)
                else:
                    # forwarded event is not enter event
                    # send normal enter event
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
                    # and now forward event to new, intialized state
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
                
    
    def __router(self, __e, compartment=None):
        target_compartment = compartment or self.__compartment
        if target_compartment.state == '__systeminitializationdemo_state_Start':
            self.__systeminitializationdemo_state_Start(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class SystemInitializationDemoCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    
if __name__ == '__main__':
    main()
