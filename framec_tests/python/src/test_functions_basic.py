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
    calc = Calculator()
    sum = calc.add(5,3)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("5 + 3 = " + str(sum))
    category = calc.categorizeNumber(42)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("42 is: " + category)
    return
class Calculator:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__calculator_state_Ready', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def add(self,x: int,y: int):
        parameters = {}
        parameters["x"] = x
        parameters["y"] = y
        self.return_stack.append(None)
        __e = FrameEvent("add",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def categorizeNumber(self,num: int):
        parameters = {}
        parameters["num"] = num
        self.return_stack.append(None)
        __e = FrameEvent("categorizeNumber",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Ready
    
    def __calculator_state_Ready(self, __e, compartment):
        if __e._message == "add":
            self.return_stack[-1] = __e._parameters["x"] + __e._parameters["y"]
            return
        elif __e._message == "categorizeNumber":
            if __e._parameters["num"] < 0:
                self.return_stack[-1] = "negative"
            elif __e._parameters["num"] == 0:
                self.return_stack[-1] = "zero"
            elif __e._parameters["num"] < 10:
                self.return_stack[-1] = "single digit"
            elif __e._parameters["num"] < 100:
                self.return_stack[-1] = "double digit"
            else:
                self.return_stack[-1] = "large number"
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sReady(self, __e):
        return self.__calculator_state_Ready(__e, None)
    
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
        if target_compartment.state == '__calculator_state_Ready':
            self.__calculator_state_Ready(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()