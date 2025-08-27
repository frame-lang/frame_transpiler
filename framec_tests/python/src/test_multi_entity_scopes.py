#Emitted from framec_v0.30.0

from enum import Enum

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
    print("=== Multi-Entity Scope Test ===")
    shared_module_var = "MODULE_SHARED"
    print("Module var: " + shared_module_var)
    function_one()
    function_two()
    function_three()
    s1 = FirstSystem()
    s2 = SecondSystem()
    s1.test_scope()
    s2.test_scope()
    test_cross_entity_isolation()
    return

def function_one():
    print("\n=== Function One ===")
    local_one = "F1_LOCAL"
    print(local_one)
    function_helper()
    return

def function_two():
    print("\n=== Function Two ===")
    local_two = "F2_LOCAL"
    print(local_two)
    function_helper()
    return

def function_three():
    print("\n=== Function Three ===")
    local_three = "F3_LOCAL"
    if True:
        nested = "F3_NESTED"
        print(nested)
        print(local_three)
    return

def function_helper():
    print("Helper called")
    return

def test_cross_entity_isolation():
    print("\n=== Cross-Entity Isolation Test ===")
    sys = FirstSystem()
    sys.test_scope()
    print("Cross-entity isolation verified")
    return

def final_test():
    print("\n=== Final Isolation Check ===")
    FirstSystem = "NOT_A_SYSTEM"
    SecondSystem = "ALSO_NOT_A_SYSTEM"
    print(FirstSystem)
    print(SecondSystem)
    real_sys = FirstSystem()
    real_sys.test_scope()
    return
class FirstSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__firstsystem_state_StateOne', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.first_domain: str = "FIRST"
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def system_operation(self):
        print("FirstSystem operation")
    # ==================== Interface Block ================== #
    
    def test_scope(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test_scope",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $StateOne
    
    def __firstsystem_state_StateOne(self, __e, compartment):
        if __e._message == "test_scope":
            print("\n=== FirstSystem Scope ===")
            self.system_operation()
            system_action()
            print("Domain: " + self.first_domain)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStateOne(self, __e):
        return self.__firstsystem_state_StateOne(__e, None)
    # ===================== Actions Block =================== #
    
    def system_action_do(self):
        
        print("FirstSystem action")
        self.first_domain = "Modified"
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
        if target_compartment.state == '__firstsystem_state_StateOne':
            self.__firstsystem_state_StateOne(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SecondSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__secondsystem_state_StateTwo', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.second_domain: str = "SECOND"
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def second_operation(self):
        print("SecondSystem operation")
    # ==================== Interface Block ================== #
    
    def test_scope(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test_scope",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $StateTwo
    
    def __secondsystem_state_StateTwo(self, __e, compartment):
        if __e._message == "test_scope":
            print("\n=== SecondSystem Scope ===")
            self.second_operation()
            second_action()
            print("Domain: " + self.second_domain)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStateTwo(self, __e):
        return self.__secondsystem_state_StateTwo(__e, None)
    # ===================== Actions Block =================== #
    
    def second_action_do(self):
        
        print("SecondSystem action")
        self.second_domain = "Modified"
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
        if target_compartment.state == '__secondsystem_state_StateTwo':
            self.__secondsystem_state_StateTwo(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
