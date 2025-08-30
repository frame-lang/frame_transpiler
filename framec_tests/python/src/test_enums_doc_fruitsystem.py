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


class FruitSystem_Fruit(Enum):
    Peach = 0
    Pear = 1
    Banana = 2
class FruitSystem:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
    # ===================== Actions Block =================== #
    
    def _getFruitOfTheDay(self):
        
        fruit_of_the_day: FruitSystem_Fruit = FruitSystem_Fruit.Pear
        return fruit_of_the_day
        return
        

