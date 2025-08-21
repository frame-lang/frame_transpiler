#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


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
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
        self.__compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        
    
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
        
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        pass
    

# ===================== Compartment =================== #

class TestSystemCompartment:

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
