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
    counter = SimpleCounter()
    counter.trigger()
    counter.trigger()
    counter.trigger()
    counter.getCount()
    return
class SimpleCounter:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__simplecounter_state_Counting', None, None, None, None, {'count': 0}, {})
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def trigger(self,):
        self.return_stack.append(None)
        __e = FrameEvent("trigger",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def getCount(self,):
        self.return_stack.append(None)
        __e = FrameEvent("getCount",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Counting
    
    def __simplecounter_state_Counting(self, __e, compartment):
        if __e._message == "trigger":
            (compartment.state_vars["count"]) = compartment.state_vars["count"] + 1
            print("Count incremented to: " + str((compartment.state_vars["count"])))
            return
        elif __e._message == "getCount":
            print("Current count is: " + str((compartment.state_vars["count"])))
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sCounting(self, __e):
        return self.__simplecounter_state_Counting(__e, None)
    
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
        if target_compartment.state == '__simplecounter_state_Counting':
            self.__simplecounter_state_Counting(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
