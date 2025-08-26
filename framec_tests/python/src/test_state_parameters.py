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


def main():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    FibonacciSystemParamsDemo(0,1)
    return
class FibonacciSystemParamsDemo:
    def __init__(self, arg0, arg1):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__fibonaccisystemparamsdemo_state_Setup', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        self.__compartment.state_args = {"zero": arg0}
        
        # Send system start event
        enter_params = {"one": arg1}
        frame_event = FrameEvent("$>", enter_params)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def next(self,):
        self.return_stack.append(None)
        __e = FrameEvent("next",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Setup
    
    def __fibonaccisystemparamsdemo_state_Setup(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print((compartment.state_args["zero"]))# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(__e._parameters["one"])# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__fibonaccisystemparamsdemo_state_PrintNextFibonacciNumber', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $PrintNextFibonacciNumber
    
    def __fibonaccisystemparamsdemo_state_PrintNextFibonacciNumber(self, __e, compartment):
        if __e._message == "next":
            sum = compartment.state_args["a"] + compartment.state_args["b"]# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(sum)# DEBUG_EXPR_TYPE: Discriminant(5)
            
            (compartment.state_args["a"]) = compartment.state_args["b"]# DEBUG_EXPR_TYPE: Discriminant(5)
            
            (compartment.state_args["b"]) = sum
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sSetup(self, __e):
        return self.__fibonaccisystemparamsdemo_state_Setup(__e, None)
    def _sPrintNextFibonacciNumber(self, __e):
        return self.__fibonaccisystemparamsdemo_state_PrintNextFibonacciNumber(__e, None)
    
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
        if target_compartment.state == '__fibonaccisystemparamsdemo_state_Setup':
            self.__fibonaccisystemparamsdemo_state_Setup(__e, target_compartment)
        elif target_compartment.state == '__fibonaccisystemparamsdemo_state_PrintNextFibonacciNumber':
            self.__fibonaccisystemparamsdemo_state_PrintNextFibonacciNumber(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()