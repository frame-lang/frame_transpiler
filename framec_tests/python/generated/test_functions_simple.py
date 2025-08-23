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
    result = add(5,3)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("5 + 3 = " + str(result))
    category = categorizeNumber(42)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("42 is " + category)
    return
class Utils:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
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
        

if __name__ == '__main__':
    main()
