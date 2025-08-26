#Emitted from framec_v0.30.0


class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None):
        self.state = state
        self.forward_event = forward_event
        self.exit_args = exit_args
        self.enter_args = enter_args
        self.parent_compartment = parent_compartment


def main():
    hsm = TransitionDetectionTest()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== Testing parent transition detection ===")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.triggerParentTransition()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.checkCurrentState()
    return
class TransitionDetectionTest:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__transitiondetectiontest_state_Child', None, None, None, FrameCompartment('__transitiondetectiontest_state_Parent', None, None, None, None))
        self.__next_compartment = None
        self.return_stack = [None]
        
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
        if __e._message == "triggerParentTransition":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Child: Before parent dispatch")
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("ERROR: This line should NOT execute due to parent transition!")
            return
        elif __e._message == "checkCurrentState":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("ERROR: This should not be called - we should be in NewState")
            return
    
    
    # ----------------------------------------
    # $Parent
    
    def __transitiondetectiontest_state_Parent(self, __e, compartment):
        if __e._message == "triggerParentTransition":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Parent: Triggering transition to NewState")# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__transitiondetectiontest_state_NewState', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "checkCurrentState":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("ERROR: This should not be called - we should be in NewState")
            return
    
    
    # ----------------------------------------
    # $NewState
    
    def __transitiondetectiontest_state_NewState(self, __e, compartment):
        if __e._message == "checkCurrentState":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("SUCCESS: We are correctly in NewState after parent transition")
            return
        elif __e._message == "triggerParentTransition":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("NewState: triggerParentTransition called (no action)")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sChild(self, __e):
        return self.__transitiondetectiontest_state_Child(__e, None)
    def _sParent(self, __e):
        return self.__transitiondetectiontest_state_Parent(__e, None)
    def _sNewState(self, __e):
        return self.__transitiondetectiontest_state_NewState(__e, None)
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent("<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else:
                # forwarded event
                if next_compartment.forward_event._message == "$>":
                    self.__router(next_compartment.forward_event)
                else:
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
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

if __name__ == '__main__':
    main()