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
    service = CleanExitService()
    return
class CleanExitService:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__cleanexitservice_state_Init', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def signal_handler(self,sig,frame):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        sys.exit(0)
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Init
    
    def __cleanexitservice_state_Init(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            signal.signal(signal.SIGINT,self.signal_handler)# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__cleanexitservice_state_A', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $A
    
    def __cleanexitservice_state_A(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("$A")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            time.sleep(.2)# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__cleanexitservice_state_B', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $B
    
    def __cleanexitservice_state_B(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("$B")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            time.sleep(.2)# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__cleanexitservice_state_A', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sInit(self, __e):
        return self.__cleanexitservice_state_Init(__e, None)
    def _sA(self, __e):
        return self.__cleanexitservice_state_A(__e, None)
    def _sB(self, __e):
        return self.__cleanexitservice_state_B(__e, None)
    
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
        if target_compartment.state == '__cleanexitservice_state_Init':
            self.__cleanexitservice_state_Init(__e, target_compartment)
        elif target_compartment.state == '__cleanexitservice_state_A':
            self.__cleanexitservice_state_A(__e, target_compartment)
        elif target_compartment.state == '__cleanexitservice_state_B':
            self.__cleanexitservice_state_B(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()