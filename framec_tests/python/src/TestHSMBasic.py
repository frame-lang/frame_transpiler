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
    sys = HSM1()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys.a()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys.b()
    return
class HSM1:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__hsm1_state_S0', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def a(self,):
        self.return_stack.append(None)
        __e = FrameEvent("a",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def b(self,):
        self.return_stack.append(None)
        __e = FrameEvent("b",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $S0
    
    def __hsm1_state_S0(self, __e, compartment):
        if __e._message == "b":# DEBUG: TransitionStmt
            
            # b
            next_compartment = FrameCompartment('__hsm1_state_S3', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $S1
    
    def __hsm1_state_S1(self, __e, compartment):
        if __e._message == "a":# DEBUG: TransitionStmt
            
            # a
            next_compartment = FrameCompartment('__hsm1_state_S2', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $S2
    
    def __hsm1_state_S2(self, __e, compartment):
        if __e._message == "a":# DEBUG: TransitionStmt
            
            # a
            next_compartment = FrameCompartment('__hsm1_state_S1', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $S3
    
    def __hsm1_state_S3(self, __e, compartment):
        pass
        
    
    # ===================== State Dispatchers =================== #
    
    def _sS0(self, __e):
        return self.__hsm1_state_S0(__e, None)
    def _sS1(self, __e):
        return self.__hsm1_state_S1(__e, None)
    def _sS2(self, __e):
        return self.__hsm1_state_S2(__e, None)
    def _sS3(self, __e):
        return self.__hsm1_state_S3(__e, None)
    
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
        if target_compartment.state == '__hsm1_state_S0':
            self.__hsm1_state_S0(__e, target_compartment)
        elif target_compartment.state == '__hsm1_state_S1':
            self.__hsm1_state_S1(__e, target_compartment)
        elif target_compartment.state == '__hsm1_state_S2':
            self.__hsm1_state_S2(__e, target_compartment)
        elif target_compartment.state == '__hsm1_state_S3':
            self.__hsm1_state_S3(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()