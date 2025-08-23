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
    grocery = Grocery()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("We are selling " + grocery.getFruitOfTheDay() + " today.")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("We sold " + grocery.getFruitOfTheDay() + " yesterday.")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("We are selling " + grocery.getFruitOfTheDay() + " tomorrow.")
    return
class Grocery:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__grocery_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def getFruitOfTheDay(self,):
        self.return_stack.append(None)
        __e = FrameEvent("getFruitOfTheDay",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __grocery_state_Start(self, __e, compartment):
        if __e._message == "getFruitOfTheDay":
            f: Grocery_Fruit = getRandomFruit()
            if f == Grocery_Fruit.Peach:# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("Found a Peach.")
                return "Peaches"
            elif f == Grocery_Fruit.Pear:# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("Found a Pear.")
                return "Pears"
            elif f == Grocery_Fruit.Banana:# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("Found a Banana.")
                return "Bananas"
            return "None"
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__grocery_state_Start(__e, None)
    # ===================== Actions Block =================== #
    
    def getRandomFruit_do(self):
        
        val = random.randint(1,3)
        if val == 1:
            return Grocery_Fruit.Peach
        elif val == 2:
            return Grocery_Fruit.Pear
        elif val == 3:
            return Grocery_Fruit.Banana
        else:
            return Grocery_Fruit.Peach
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
        if target_compartment.state == '__grocery_state_Start':
            self.__grocery_state_Start(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
