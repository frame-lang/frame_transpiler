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


def testFruit():
    f: Fruit = getFruitOfTheDay()
    if f == Fruit.Peach:# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Found a Peach")
    elif f == Fruit.Pear:# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Found a Pear")
    elif f == Fruit.Banana:# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Found a Banana")
    else:# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Unknown fruit")
    return

def describeFruit(fruit_value: Fruit):
    if fruit_value == Fruit.Peach:# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Peaches")
    elif fruit_value == Fruit.Pear:# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Pears")
    elif fruit_value == Fruit.Banana:# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Bananas")
    else:# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Other Fruit")
    return
class CalendarSystem:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
class EnumValueSystem:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
class FruitSystem:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
    # ===================== Actions Block =================== #
    
    def getFruitOfTheDay_do(self):
        
        fruit_of_the_day: FruitSystem_Fruit = FruitSystem_Fruit.Pear
        return fruit_of_the_day
        return
        
