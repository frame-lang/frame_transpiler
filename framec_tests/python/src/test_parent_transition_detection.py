#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    hsm = TransitionDetectionTest()
    print("=== Testing parent transition detection ===")
    hsm.triggerParentTransition()
    hsm.checkCurrentState()
    return

class TransitionDetectionTest:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = TransitionDetectionTestCompartment('__transitiondetectiontest_state_Parent', next_compartment)
        next_compartment = TransitionDetectionTestCompartment('__transitiondetectiontest_state_Child', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def triggerParentTransition(self,):
        self.return_stack.append(None)
        __e = FrameEvent("triggerParentTransition",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def checkCurrentState(self,):
        self.return_stack.append(None)
        __e = FrameEvent("checkCurrentState",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Child
    
    def __transitiondetectiontest_state_Child(self, __e, compartment):
        if __e._message == "triggerParentTransition":
            print("Child: Before parent dispatch")
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            print("ERROR: This line should NOT execute due to parent transition!")
            return
        elif __e._message == "checkCurrentState":
            print("ERROR: This should not be called - we should be in NewState")
            return
        
        self.__router(__e, compartment.parent_compartment)
        
    
    
    # ----------------------------------------
    # $Parent
    
    def __transitiondetectiontest_state_Parent(self, __e, compartment):
        if __e._message == "triggerParentTransition":
            print("Parent: Triggering transition to NewState")
            next_compartment = None
            next_compartment = TransitionDetectionTestCompartment('__transitiondetectiontest_state_NewState', next_compartment)
            self.__transition(next_compartment)
            return
        elif __e._message == "checkCurrentState":
            print("ERROR: This should not be called - we should be in NewState")
            return
    
    
    # ----------------------------------------
    # $NewState
    
    def __transitiondetectiontest_state_NewState(self, __e, compartment):
        if __e._message == "checkCurrentState":
            print("SUCCESS: We are correctly in NewState after parent transition")
            return
        elif __e._message == "triggerParentTransition":
            print("NewState: triggerParentTransition called (no action)")
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
        if target_compartment.state == '__transitiondetectiontest_state_Child':
            self.__transitiondetectiontest_state_Child(__e, target_compartment)
        elif target_compartment.state == '__transitiondetectiontest_state_Parent':
            self.__transitiondetectiontest_state_Parent(__e, target_compartment)
        elif target_compartment.state == '__transitiondetectiontest_state_NewState':
            self.__transitiondetectiontest_state_NewState(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class TransitionDetectionTestCompartment:

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
