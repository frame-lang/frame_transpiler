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
    print("=== System Scope Isolation Test ===")
    sys1 = SystemOne()
    sys2 = SystemTwo()
    sys1.public_method()
    sys2.public_method()
    sys1.try_cross_call()
    sys2.try_cross_call()
    print("\nSystem isolation tests completed")
    return
class SystemOne:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systemone_state_Active', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.domain_one: str = "SystemOne Domain"
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def internal_op_one(self):
        return "SystemOne internal operation"
    # ==================== Interface Block ================== #
    
    def public_method(self,):
        self.return_stack.append(None)
        __e = FrameEvent("public_method",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def try_cross_call(self,):
        self.return_stack.append(None)
        __e = FrameEvent("try_cross_call",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Active
    
    def __systemone_state_Active(self, __e, compartment):
        if __e._message == "public_method":
            print("\n=== SystemOne Public Method ===")
            result = self.internal_op_one()
            print("Own operation: " + result)
            self.private_action_one_do()
            print("Own domain: " + self.domain_one)
            return
        elif __e._message == "try_cross_call":
            print("\n=== SystemOne Trying Cross-System Access ===")
            other = SystemTwo()
            other.public_method()
            print("Can only access SystemTwo through public interface")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sActive(self, __e):
        return self.__systemone_state_Active(__e, None)
    # ===================== Actions Block =================== #
    
    def private_action_one_do(self):
        
        print("SystemOne private action")
        self.domain_one = "Modified by SystemOne"
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
        if target_compartment.state == '__systemone_state_Active':
            self.__systemone_state_Active(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SystemTwo:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systemtwo_state_Running', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.domain_two: str = "SystemTwo Domain"
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def internal_op_two(self):
        return "SystemTwo internal operation"
    # ==================== Interface Block ================== #
    
    def public_method(self,):
        self.return_stack.append(None)
        __e = FrameEvent("public_method",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def try_cross_call(self,):
        self.return_stack.append(None)
        __e = FrameEvent("try_cross_call",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def get_value(self,):
        self.return_stack.append(None)
        __e = FrameEvent("get_value",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Running
    
    def __systemtwo_state_Running(self, __e, compartment):
        if __e._message == "public_method":
            print("\n=== SystemTwo Public Method ===")
            result = self.internal_op_two()
            print("Own operation: " + result)
            self.private_action_two_do()
            print("Own domain: " + self.domain_two)
            return
        elif __e._message == "try_cross_call":
            print("\n=== SystemTwo Trying Cross-System Access ===")
            other = SystemOne()
            other.public_method()
            print("Can only access SystemOne through public interface")
            return
        elif __e._message == "get_value":
            return self.domain_two
    
    # ===================== State Dispatchers =================== #
    
    def _sRunning(self, __e):
        return self.__systemtwo_state_Running(__e, None)
    # ===================== Actions Block =================== #
    
    def private_action_two_do(self):
        
        print("SystemTwo private action")
        self.domain_two = "Modified by SystemTwo"
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
        if target_compartment.state == '__systemtwo_state_Running':
            self.__systemtwo_state_Running(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SystemThree:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systemthree_state_Waiting', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def test_isolation(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test_isolation",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Waiting
    
    def __systemthree_state_Waiting(self, __e, compartment):
        if __e._message == "test_isolation":
            print("\n=== SystemThree Isolation Test ===")
            s1 = SystemOne()
            s2 = SystemTwo()
            s1.public_method()
            s2.public_method()
            value = s2.get_value()
            print("Got value from SystemTwo: " + value)
            self.own_action_do()
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sWaiting(self, __e):
        return self.__systemthree_state_Waiting(__e, None)
    # ===================== Actions Block =================== #
    
    def own_action_do(self):
        
        print("SystemThree own action")
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
        if target_compartment.state == '__systemthree_state_Waiting':
            self.__systemthree_state_Waiting(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
