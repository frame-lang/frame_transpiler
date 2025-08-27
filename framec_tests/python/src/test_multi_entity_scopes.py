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
    
    print("=== Multi-Entity Scope Test ===")
    shared_module_var = "MODULE_SHARED"# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Module var: " + shared_module_var)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    function_one()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    function_two()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    function_three()
    s1 = FirstSystem()
    s2 = SecondSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    s1.test_scope()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    s2.test_scope()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    test_cross_entity_isolation()
    return

def function_one():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("\n=== Function One ===")
    local_one = "F1_LOCAL"# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(local_one)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    function_helper()
    return

def function_two():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("\n=== Function Two ===")
    local_two = "F2_LOCAL"# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(local_two)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    function_helper()
    return

def function_three():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("\n=== Function Three ===")
    local_three = "F3_LOCAL"
    if True:
        nested = "F3_NESTED"# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(nested)# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(local_three)
    return

def function_helper():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Helper called")
    return

def test_cross_entity_isolation():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("\n=== Cross-Entity Isolation Test ===")
    sys = FirstSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys.test_scope()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Cross-entity isolation verified")
    return
class FirstSystem:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
    
    # ==================== Operations Block ================== #

if __name__ == '__main__':
    main()
