#Emitted from framec_v0.30.0


from enum import Enum
import random

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    grocery = Grocery()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("We are selling " + grocery.getFruitOfTheDay() + " today.")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("We sold " + grocery.getFruitOfTheDay() + " yesterday.")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("We are selling " + grocery.getFruitOfTheDay() + " tomorrow.")
    return
class Grocery:
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        # Constructor implementation will be added here
    
    # Interface methods will be added here
    
    # State machine will be added here
    
    # Action methods will be added here
    
    # System runtime (__kernel, __router, __transition) will be added here
    
if __name__ == '__main__':
    main()
