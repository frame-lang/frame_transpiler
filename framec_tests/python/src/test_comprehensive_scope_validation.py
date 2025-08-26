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
    
    sys1# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys2# DEBUG_EXPR_TYPE: Discriminant(4)
    
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
    
    ops_test
    return
class TestSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__testsystem_state_Idle', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.domain_var = "domain_variable"
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def test_operations(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("=== Operations Block Scope ===")
        ops_var = "operations_variable"# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(ops_var)
    
    def run_operation(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Operation called correctly (no self.self bug)")
    # ==================== Interface Block ================== #
    
    def test_interface(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test_interface",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def process(self,data: string):
        parameters = {}
        parameters["data"] = data
        self.return_stack.append(None)
        __e = FrameEvent("process",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Idle
    
    def __testsystem_state_Idle(self, __e, compartment):
        if __e._message == "test_interface":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("=== Machine Block - Event Handler Scope ===")
            handler_var = "event_handler_variable"# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(handler_var)# DEBUG_EXPR_TYPE: Discriminant(4)
            
            process("test_data")
            return
            return
        elif __e._message == "process":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("=== Event Handler with Parameters ===")
            param_local = "param_handler_variable"# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(__e._parameters["data"])# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(param_local)
            if __e._parameters["data"] == "test_data":
                nested_handler_var = "nested_in_handler"# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print(nested_handler_var)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sIdle(self, __e):
        return self.__testsystem_state_Idle(__e, None)
    # ===================== Actions Block =================== #
    
    def internal_action_do(self):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("=== Actions Block Scope ===")
        action_var = "action_variable"# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(action_var)
        return
        
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent("<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else:
                # forwarded event
                if next_compartment.forward_event._message == "$>":
                    self.__router(next_compartment.forward_event)
                else:
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
    
    def __router(self, __e, compartment=None):
        target_compartment = compartment or self.__compartment
        if target_compartment.state == '__testsystem_state_Idle':
            self.__testsystem_state_Idle(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class ComplexSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__complexsystem_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.domain_var = "complex_domain_variable"
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def test_interface(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test_interface",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __complexsystem_state_Start(self, __e, compartment):
        if __e._message == "test_interface":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("=== Complex System Scope Test ===")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(domain_var)
            state_local = "state_local_variable"# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(state_local)
            for item in ["a","b","c"]:
                loop_in_handler = "loop_in_event_handler"# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print(loop_in_handler)
                if item == "b":
                    deep_nested = "deeply_nested_variable"# DEBUG_EXPR_TYPE: Discriminant(4)
                    
                    print(deep_nested)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__complexsystem_state_Start(__e, None)
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent("<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else:
                # forwarded event
                if next_compartment.forward_event._message == "$>":
                    self.__router(next_compartment.forward_event)
                else:
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
    
    def __router(self, __e, compartment=None):
        target_compartment = compartment or self.__compartment
        if target_compartment.state == '__complexsystem_state_Start':
            self.__complexsystem_state_Start(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()