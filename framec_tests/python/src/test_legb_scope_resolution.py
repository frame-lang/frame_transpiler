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
    print("=== LEGB Scope Resolution Test ===")
    name = "MODULE"
    value = 100
    print("Module scope: name=" + name)
    test_function_scope()
    print("After function: name=" + name)
    test_nested_scopes()
    test_builtin_access()
    return

def test_function_scope():
    print("\n=== Function Scope Test ===")
    name = "FUNCTION"
    local_only = "LOCAL_VAR"
    print("Function scope: name=" + name)
    print("Function local: " + local_only)
    if True:
        name = "BLOCK"
        block_only = "BLOCK_VAR"
        print("Block scope: name=" + name)
        print("Block local: " + block_only)
        if True:
            name = "NESTED"
            print("Nested block: name=" + name)
        print("After nested: name=" + name)
    print("After block: name=" + name)
    return

def test_nested_scopes():
    print("\n=== Nested Scope Test ===")
    level1 = "L1"
    if True:
        level2 = "L2"
        print("Can see L1: " + level1)
        print("Can see L2: " + level2)
        if True:
            level3 = "L3"
            print("Can see L1: " + level1)
            print("Can see L2: " + level2)
            print("Can see L3: " + level3)
            level1 = "L1_SHADOW"
            print("Shadowed L1: " + level1)
        print("L1 restored: " + level1)
    print("Only L1 remains: " + level1)
    return

def test_builtin_access():
    print("\n=== Built-in Access Test ===")
    print("Built-in print works")
    print = "SHADOWED_PRINT"
    if True:
        msg = "Shadow value: " + print
    return

def test_loop_scopes():
    print("\n=== Loop Scope Test ===")
    outer = "OUTER"
    for i in [1,2,3]:
        loop_var = "LOOP_" + str(i)
        print(loop_var)
        print("Outer in loop: " + outer)
        outer = "LOOP_SHADOW"
        print("Shadowed in loop: " + outer)
    print("After loop: " + outer)
    return
class TestSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__testsystem_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.domain_var: str = "DOMAIN"
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def test(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __testsystem_state_Start(self, __e, compartment):
        if __e._message == "test":
            print("\n=== System Scope Test ===")
            handler_var = "HANDLER"
            print(handler_var)
            print(self.domain_var)
            if True:
                nested = "NESTED_IN_HANDLER"
                print(nested)
                print(handler_var)
                print(self.domain_var)
            test_action()
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__testsystem_state_Start(__e, None)
    # ===================== Actions Block =================== #
    
    def test_action_do(self):
        
        print("Action scope")
        action_var = "ACTION"
        print(action_var)
        print(self.domain_var)
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
        if target_compartment.state == '__testsystem_state_Start':
            self.__testsystem_state_Start(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
