#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    result = self.add_do(5,3)
    print("5 + 3 = " + str(result))
    category = self.categorizeNumber_do(42)
    print("42 is " + category)
    return

class Utils:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
        self.__compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
    
    # ===================== Actions Block =================== #
    
    def add_do(self,x: int,y: int):
        
        return x + y
        return
        
    
    def categorizeNumber_do(self,num: int):
        
        if num < 0:
            return "negative"
        elif num == 0:
            return "zero"
        elif num < 10:
            return "single digit"
        elif num < 100:
            return "double digit"
        else:
            return "large number"
        return
        
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        pass
    

# ===================== Compartment =================== #

class UtilsCompartment:

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
