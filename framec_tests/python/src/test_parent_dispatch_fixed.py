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
    hsm = ParentDispatchTest()
    hsm.test1()
    hsm.test2()
    hsm.test3()
    hsm.next()
    return
class ParentDispatchTest:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__parentdispatchtest_state_Parent', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def test1(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test1",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def test2(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test2",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def test3(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test3",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def next(self,):
        self.return_stack.append(None)
        __e = FrameEvent("next",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Parent
    
    def __parentdispatchtest_state_Parent(self, __e, compartment):
        if __e._message == "test1":
            print("test1 handled in parent")
            return
        elif __e._message == "test2":
            print("test2 handled in parent")
            return
        elif __e._message == "test3":
            print("test3 parent triggers transition")
            next_compartment = FrameCompartment('__parentdispatchtest_state_Child2', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Child1
    
    def __parentdispatchtest_state_Child1(self, __e, compartment):
        if __e._message == "test1":
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            return
        elif __e._message == "test2":
            print("test2 in child before dispatch")
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            print("test2 in child after dispatch - should execute")
            return
        elif __e._message == "test3":
            print("test3 in child before dispatch")
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            print("test3 in child after dispatch - should NOT execute due to transition")
            return
        elif __e._message == "$>":
            print("enter child1")
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            print("enter child1 - after parent dispatch")
            return
        elif __e._message == "<$":
            print("exit child1")
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            print("exit child1 - after parent dispatch")
            return
        elif __e._message == "next":
            next_compartment = FrameCompartment('__parentdispatchtest_state_Child2', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Child2
    
    def __parentdispatchtest_state_Child2(self, __e, compartment):
        if __e._message == "$>":
            print("enter child2")
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            return
        elif __e._message == "<$":
            print("exit child2")
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            return
        elif __e._message == "next":
            next_compartment = FrameCompartment('__parentdispatchtest_state_Child1', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sParent(self, __e):
        return self.__parentdispatchtest_state_Parent(__e, None)
    def _sChild1(self, __e):
        return self.__parentdispatchtest_state_Child1(__e, None)
    def _sChild2(self, __e):
        return self.__parentdispatchtest_state_Child2(__e, None)
    
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
        if target_compartment.state == '__parentdispatchtest_state_Parent':
            self.__parentdispatchtest_state_Parent(__e, target_compartment)
        elif target_compartment.state == '__parentdispatchtest_state_Child1':
            self.__parentdispatchtest_state_Child1(__e, target_compartment)
        elif target_compartment.state == '__parentdispatchtest_state_Child2':
            self.__parentdispatchtest_state_Child2(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
