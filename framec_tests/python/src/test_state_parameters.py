#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    FibonacciSystemParamsDemo(0,1)
    return

class FibonacciSystemParamsDemo:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self,start_state_state_param_zero,start_state_enter_param_one):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = FibonacciSystemParamsDemoCompartment('__fibonaccisystemparamsdemo_state_Setup', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        self.__compartment.state_args["zero"] = start_state_state_param_zero
        self.__compartment.enter_args["one"] = start_state_enter_param_one
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent("$>", self.__compartment.enter_args)
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
        if __e._message == "$>":
            print((compartment.state_args["zero"]))
            print(__e._parameters["one"])
            next_compartment = None
            next_compartment = FibonacciSystemParamsDemoCompartment('__fibonaccisystemparamsdemo_state_PrintNextFibonacciNumber', next_compartment)
            next_compartment.state_args["a"] = compartment.state_args["zero"]
            next_compartment.state_args["b"] = __e._parameters["one"]
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $PrintNextFibonacciNumber
    
    def __fibonaccisystemparamsdemo_state_PrintNextFibonacciNumber(self, __e, compartment):
        if __e._message == "next":
            sum = compartment.state_args["a"] + compartment.state_args["b"]
            print(sum)
            (compartment.state_args["a"]) = compartment.state_args["b"]
            (compartment.state_args["b"]) = sum
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
        if target_compartment.state == '__fibonaccisystemparamsdemo_state_Setup':
            self.__fibonaccisystemparamsdemo_state_Setup(__e, target_compartment)
        elif target_compartment.state == '__fibonaccisystemparamsdemo_state_PrintNextFibonacciNumber':
            self.__fibonaccisystemparamsdemo_state_PrintNextFibonacciNumber(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class FibonacciSystemParamsDemoCompartment:

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
