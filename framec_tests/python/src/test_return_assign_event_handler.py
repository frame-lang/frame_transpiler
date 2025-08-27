#Emitted from framec_v0.30.0

from enum import Enum

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
    calculator = Calculator()
    result1 = calculator.getDefault()
    print("Default value: " + result1)
    result2 = calculator.calculate(10,5)
    print("10 + 5 = " + str(result2))
    result3 = calculator.divide(10,0)
    print("10 / 0 = " + result3)
    result4 = calculator.divide(10,2)
    print("10 / 2 = " + str(result4))
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
    
    def getDefault(self,):
        self.return_stack.append("default_value")
        __e = FrameEvent("getDefault",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def calculate(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a
        parameters["b"] = b
        self.return_stack.append(None)
        __e = FrameEvent("calculate",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def divide(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a
        parameters["b"] = b
        self.return_stack.append(None)
        __e = FrameEvent("divide",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Ready
    
    def __calculator_state_Ready(self, __e, compartment):
        if __e._message == "getDefault":
            return
        elif __e._message == "calculate":
            self.return_stack[-1] = __e._parameters["a"] + __e._parameters["b"]
            print("Calculated sum: " + str(__e._parameters["a"] + __e._parameters["b"]))
            return
        elif __e._message == "divide":
            if __e._parameters["b"] == 0:
                self.return_stack[-1] = "error: division by zero"
                return
            self.return_stack[-1] = __e._parameters["a"] / __e._parameters["b"]
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
