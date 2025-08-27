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
    print("=== Single System Test ===")
    sys = SingleSystem()
    sys.next()
    sys.next()
    sys.next()
    print("=== Test Complete ===")
    return
class SingleSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__singlesystem_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def next(self,):
        self.return_stack.append(None)
        __e = FrameEvent("next",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __singlesystem_state_Start(self, __e, compartment):
        if __e._message == "$>":
            print("Entering Start")
            return
        elif __e._message == "<$":
            print("Exiting Start")
            return
        elif __e._message == "next":
            print("Start.next() -> Working")
            self.return_stack[-1] = True
            next_compartment = FrameCompartment('__singlesystem_state_Working', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Working
    
    def __singlesystem_state_Working(self, __e, compartment):
        if __e._message == "$>":
            print("Entering Working")
            return
        elif __e._message == "<$":
            print("Exiting Working")
            return
        elif __e._message == "next":
            print("Working.next() -> End")
            self.return_stack[-1] = True
            next_compartment = FrameCompartment('__singlesystem_state_End', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $End
    
    def __singlesystem_state_End(self, __e, compartment):
        if __e._message == "$>":
            print("Entering End")
            return
        elif __e._message == "<$":
            print("Exiting End")
            return
        elif __e._message == "next":
            print("End.next() - complete")
            self.return_stack[-1] = False
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__singlesystem_state_Start(__e, None)
    def _sWorking(self, __e):
        return self.__singlesystem_state_Working(__e, None)
    def _sEnd(self, __e):
        return self.__singlesystem_state_End(__e, None)
    
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
        if target_compartment.state == '__singlesystem_state_Start':
            self.__singlesystem_state_Start(__e, target_compartment)
        elif target_compartment.state == '__singlesystem_state_Working':
            self.__singlesystem_state_Working(__e, target_compartment)
        elif target_compartment.state == '__singlesystem_state_End':
            self.__singlesystem_state_End(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
