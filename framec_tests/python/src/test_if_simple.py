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

class A:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
    # ===================== Actions Block =================== #
    
    def a_do(self):
        
        if True:
            doY()
        return
        

