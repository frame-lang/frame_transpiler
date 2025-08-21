#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


class A:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
        self.__compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
    
    # ===================== Actions Block =================== #
    
    def a_do(self):
        
        doSomething()
        if x:
            doY()
        doOtherThing()
        return
        
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        pass
    

# ===================== Compartment =================== #

class ACompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    