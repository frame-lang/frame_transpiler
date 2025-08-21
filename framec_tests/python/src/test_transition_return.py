#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    sys = TransitionTest()
    sys.test()
    return

class TransitionTest:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = TransitionTestCompartment('__transitiontest_state_Start', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def test(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __transitiontest_state_Start(self, __e, compartment):
        if __e._message == "test":
            condition = "error"
            if condition == "error":
                next_compartment = None
                next_compartment = TransitionTestCompartment('__transitiontest_state_Error', next_compartment)
                self.__transition(next_compartment)
            elif condition == "success":
                next_compartment = None
                next_compartment = TransitionTestCompartment('__transitiontest_state_Success', next_compartment)
                self.__transition(next_compartment)
            else:
                next_compartment = None
                next_compartment = TransitionTestCompartment('__transitiontest_state_Default', next_compartment)
                self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Error
    
    def __transitiontest_state_Error(self, __e, compartment):
        if __e._message == "$>":
            print("In error state")
            return
    
    
    # ----------------------------------------
    # $Success
    
    def __transitiontest_state_Success(self, __e, compartment):
        if __e._message == "$>":
            print("In success state")
            return
    
    
    # ----------------------------------------
    # $Default
    
    def __transitiontest_state_Default(self, __e, compartment):
        if __e._message == "$>":
            print("In default state")
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
        if target_compartment.state == '__transitiontest_state_Start':
            self.__transitiontest_state_Start(__e, target_compartment)
        elif target_compartment.state == '__transitiontest_state_Error':
            self.__transitiontest_state_Error(__e, target_compartment)
        elif target_compartment.state == '__transitiontest_state_Success':
            self.__transitiontest_state_Success(__e, target_compartment)
        elif target_compartment.state == '__transitiontest_state_Default':
            self.__transitiontest_state_Default(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class TransitionTestCompartment:

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
