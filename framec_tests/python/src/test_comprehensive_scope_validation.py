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
    
    print("=== Module Level (main function) ===")
    module_var = "module_variable"
    sys1 = TestSystem()
    sys2 = ComplexSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    test_function_scope()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys1.test_operations()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys2.test_interface()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(module_var)
    return

def test_function_scope():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== Function Scope ===")
    func_var = "function_variable"
    local_counter = 42
    if True:
        if_var = "if_block_variable"# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(func_var)# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(if_var)# DEBUG_EXPR_TYPE: Discriminant(5)
        
        local_counter = local_counter + 1
        if local_counter > 40:
            nested_if_var = "nested_if_variable"# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(nested_if_var)
    for i in [1,2,3]:
        loop_var = "loop_variable"# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(loop_var)# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Loop iteration")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(func_var)
    return

def test_operation_calls():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== Testing Operations Calls ===")
    ops_test = TestSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    ops_test.run_operation()
    return
class TestSystem:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
    
    # ==================== Operations Block ================== #

if __name__ == '__main__':
    main()
