#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


class FruitSystem_Fruit(Enum):
    Peach = 0
    Pear = 1
    Banana = 2


class FruitSystem:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
        self.__compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        
    
    # ===================== Actions Block =================== #
    
    def getFruitOfTheDay_do(self):
        
        fruit_of_the_day: FruitSystem_Fruit = FruitSystem_Fruit.Pear
        return fruit_of_the_day
        return
        
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        pass
    

# ===================== Compartment =================== #

class FruitSystemCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    