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
    print("=== System Isolation Test ===")
    sys1 = SystemOne()
    sys2 = SystemTwo()
    sys1.test_public()
    sys2.test_public()
    print("System isolation test completed")
    return
class SystemOne:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systemone_state_Active', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def internal_one(self):
        print("SystemOne internal operation")
    # ==================== Interface Block ================== #
    
    def test_public(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test_public",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Active
    
    def __systemone_state_Active(self, __e, compartment):
        if __e._message == "test_public":
            print("SystemOne public method")
            self.internal_one()
            self.action_one_do()
            other = SystemTwo()
            other.test_public()
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sActive(self, __e):
        return self.__systemone_state_Active(__e, None)
    # ===================== Actions Block =================== #
    
    def action_one_do(self):
        
        print("SystemOne action")
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
        self.__compartment = FrameCompartment('__systemtwo_state_Ready', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def internal_two(self):
        print("SystemTwo internal operation")
    # ==================== Interface Block ================== #
    
    def test_public(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test_public",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Ready
    
    def __systemtwo_state_Ready(self, __e, compartment):
        if __e._message == "test_public":
            print("SystemTwo public method")
            self.internal_two()
            self.action_two_do()
            other = SystemOne()
            other.test_public()
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sReady(self, __e):
        return self.__systemtwo_state_Ready(__e, None)
    # ===================== Actions Block =================== #
    
    def action_two_do(self):
        
        print("SystemTwo action")
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
        if target_compartment.state == '__systemtwo_state_Ready':
            self.__systemtwo_state_Ready(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
