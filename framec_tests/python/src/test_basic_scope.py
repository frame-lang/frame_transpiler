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
    
    print("=== Basic Scope Test ===")
    module_var = "MODULE"# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(module_var)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    test_function()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Module var after function:")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(module_var)
    return

def test_function():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== Function Scope ===")
    func_var = "FUNCTION"# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(func_var)
    if True:
        block_var = "BLOCK"# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(block_var)# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(func_var)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(func_var)
    return

if __name__ == '__main__':
    main()
