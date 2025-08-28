#Emitted from framec_v0.30.0

from enum import Enum

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None, state_vars=None, state_args=None):
        self.state = state
        self.forward_event = forward_event
        self.exit_args = exit_args
        self.enter_args = enter_args
        self.parent_compartment = parent_compartment
        self.state_vars = state_vars or {}
        self.state_args = state_args or {}


def main():
    print("=== Function Scope Isolation Test ===")
    sys = IsolatedSystem()
    sys.public_interface()
    test_function_cannot_access_internals()
    test_function_can_call_functions()
    test_function_can_use_builtins()
    return

def test_function_cannot_access_internals():
    print("\n=== Function Cannot Access System Internals ===")
    local_sys = IsolatedSystem()
    local_sys.public_interface()
    print("Function isolation test completed")
    return

def test_function_can_call_functions():
    print("\n=== Function Can Call Other Functions ===")
    helper_function()
    result = compute_value(5,3)
    print("Computed: " + str(result))
    return

def test_function_can_use_builtins():
    print("\n=== Function Can Use Built-ins ===")
    print("Print works")
    text = str(42)
    print("Stringified: " + text)
    num = 10
    print("Number: " + str(num))
    return

def helper_function():
    print("Helper function called successfully")
    return

def compute_value(a,b):
    self.return_stack[-1] = a + b
    return
    return
class IsolatedSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__isolatedsystem_state_Idle', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.internal_data: str = "INTERNAL"
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def internal_operation(self):
        print("Internal operation - should not be callable from functions")
    # ==================== Interface Block ================== #
    
    def public_interface(self,):
        self.return_stack.append(None)
        __e = FrameEvent("public_interface",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Idle
    
    def __isolatedsystem_state_Idle(self, __e, compartment):
        if __e._message == "public_interface":
            print("Public interface called")
            self.internal_operation()
            self.private_action_do()
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sIdle(self, __e):
        return self.__isolatedsystem_state_Idle(__e, None)
    # ===================== Actions Block =================== #
    
    def private_action_do(self):
        
        print("Private action - should not be callable from functions")
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
        if target_compartment.state == '__isolatedsystem_state_Idle':
            self.__isolatedsystem_state_Idle(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class AnotherSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__anothersystem_state_Ready', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def another_interface(self,):
        self.return_stack.append(None)
        __e = FrameEvent("another_interface",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Ready
    
    def __anothersystem_state_Ready(self, __e, compartment):
        if __e._message == "another_interface":
            print("Another system's interface")
            other = IsolatedSystem()
            other.public_interface()
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sReady(self, __e):
        return self.__anothersystem_state_Ready(__e, None)
    
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
        if target_compartment.state == '__anothersystem_state_Ready':
            self.__anothersystem_state_Ready(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
