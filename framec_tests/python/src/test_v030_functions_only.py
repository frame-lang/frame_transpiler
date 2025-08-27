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


def main():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Main function")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    helper("test")
    result = calculate(10,20)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(result)
    return

def helper(msg):# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Helper: " + msg)
    return

def calculate(a,b):
    return a + b
    return

if __name__ == '__main__':
    main()
