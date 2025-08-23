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

class HelloWorldSystem:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
    # ==================== Interface Block ================== #
    
    def sayHello(self,):
        self.return_stack.append(None)
        __e = FrameEvent("sayHello",None)
        return self.return_stack.pop(-1)
    
    def sayWorld(self,):
        self.return_stack.append(None)
        __e = FrameEvent("sayWorld",None)
        return self.return_stack.pop(-1)
    

