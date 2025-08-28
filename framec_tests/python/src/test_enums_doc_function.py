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
        self.__compartment = None
        self.return_stack = [None]
    # ===================== Actions Block =================== #
    
    def testFruit_do(self):
        
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
        
    
    def describeFruit_do(self,fruit_value: TestSystem_Fruit):
        
        if fruit_value == TestSystem_Fruit.Peach:
            print("Peaches")
        elif fruit_value == TestSystem_Fruit.Pear:
            print("Pears")
        elif fruit_value == TestSystem_Fruit.Banana:
            print("Bananas")
        else:
            print("Other Fruit")
        return
        

if __name__ == '__main__':
    main()
