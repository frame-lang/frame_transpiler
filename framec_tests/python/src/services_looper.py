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
    looper = Looper(1000000)
    return
class Looper:
    def __init__(self, arg0):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__looper_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        enter_params = {"loops": arg0}
        frame_event = FrameEvent("$>", enter_params)
        self.__kernel(frame_event)
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __looper_state_Start(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Starting")# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__looper_state_A', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $A
    
    def __looper_state_A(self, __e, compartment):
        if __e._message == "$>":
            if __e._parameters["loops_left"] == 0:# DEBUG: TransitionStmt
                
                next_compartment = FrameCompartment('__looper_state_Done', None, None, None, None)
                self.__transition(next_compartment)# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__looper_state_B', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $B
    
    def __looper_state_B(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(5)
            
            __e._parameters["loops_left"] = __e._parameters["loops_left"] - 1# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__looper_state_A', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Done
    
    def __looper_state_Done(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Done. Looped " + str(__e._parameters["total_loops"]) + " times in ",end = " ")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(str(time.time() - __e._parameters["start"]) + " seconds.")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__looper_state_Start(__e, None)
    def _sA(self, __e):
        return self.__looper_state_A(__e, None)
    def _sB(self, __e):
        return self.__looper_state_B(__e, None)
    def _sDone(self, __e):
        return self.__looper_state_Done(__e, None)
    
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
        if target_compartment.state == '__looper_state_Start':
            self.__looper_state_Start(__e, target_compartment)
        elif target_compartment.state == '__looper_state_A':
            self.__looper_state_A(__e, target_compartment)
        elif target_compartment.state == '__looper_state_B':
            self.__looper_state_B(__e, target_compartment)
        elif target_compartment.state == '__looper_state_Done':
            self.__looper_state_Done(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()