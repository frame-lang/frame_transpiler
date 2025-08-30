#Emitted from framec_v0.30.0

from enum import Enum

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None, state_vars=None, state_args=None):
        self.state = state
        self.forward_event = forward_event
        self.exit_args = exit_args
        self.enter_args = enter_args
        self.parent_compartment = parent_compartment
        self.state_vars = state_vars or {}
        self.state_args = state_args or {}


class TestSystem_Fruit(Enum):
    Peach = 0
    Pear = 1
    Banana = 2

def main():
    sys = TestSystem()
    sys.testFruit()
    sys.describeFruit(TestSystem_Fruit.Banana)
    return
class TestSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__testsystem_state_Ready', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def testFruit(self,):
        self.return_stack.append(None)
        __e = FrameEvent("testFruit",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def describeFruit(self,fruit_value: TestSystem_Fruit):
        parameters = {}
        parameters["fruit_value"] = fruit_value
        self.return_stack.append(None)
        __e = FrameEvent("describeFruit",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Ready
    
    def __testsystem_state_Ready(self, __e, compartment):
        if __e._message == "testFruit":
            _testFruit()
            return
        elif __e._message == "describeFruit":
            _describeFruit(__e._parameters["fruit_value"])
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sReady(self, __e):
        return self.__testsystem_state_Ready(__e, None)
    # ===================== Actions Block =================== #
    
    def _testFruit(self):
        
        f: TestSystem_Fruit = TestSystem_Fruit.Pear
        if f == TestSystem_Fruit.Peach:
            print("Found a Peach")
        elif f == TestSystem_Fruit.Pear:
            print("Found a Pear")
        elif f == TestSystem_Fruit.Banana:
            print("Found a Banana")
        else:
            print("Unknown fruit")
        return
        
    
    def _describeFruit(self,fruit_value: TestSystem_Fruit):
        
        if fruit_value == TestSystem_Fruit.Peach:
            print("Peaches")
        elif fruit_value == TestSystem_Fruit.Pear:
            print("Pears")
        elif fruit_value == TestSystem_Fruit.Banana:
            print("Bananas")
        else:
            print("Other Fruit")
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
        if target_compartment.state == '__testsystem_state_Ready':
            self.__testsystem_state_Ready(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
