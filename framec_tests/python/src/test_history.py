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
    sys = History103()
    sys.gotoC()
    sys.ret()
    return
class History103:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__history103_state_A', None, None, None, None)
        self.__next_compartment = None
        self.__state_stack = []
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def gotoC(self,):
        self.return_stack.append(None)
        __e = FrameEvent("gotoC",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def ret(self,):
        self.return_stack.append(None)
        __e = FrameEvent("ret",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $A
    
    def __history103_state_A(self, __e, compartment):
        if __e._message == "$>":
            print("In $A")
            return
        elif __e._message == "gotoC":
            print("$A pushing to stack and going to $C")
            self.__state_stack_push(self.__compartment)
            # $$[+]
            next_compartment = FrameCompartment('__history103_state_C', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $B
    
    def __history103_state_B(self, __e, compartment):
        if __e._message == "$>":
            print("In $B")
            return
        elif __e._message == "gotoC":
            print("$B pushing to stack and going to $C")
            self.__state_stack_push(self.__compartment)
            # $$[+]
            next_compartment = FrameCompartment('__history103_state_C', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $C
    
    def __history103_state_C(self, __e, compartment):
        if __e._message == "$>":
            print("In $C")
            return
        elif __e._message == "ret":
            print("Popping from stack and returning")
            # $$[-]
            next_compartment = self.__state_stack_pop()
            self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sA(self, __e):
        return self.__history103_state_A(__e, None)
    def _sB(self, __e):
        return self.__history103_state_B(__e, None)
    def _sC(self, __e):
        return self.__history103_state_C(__e, None)
    
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
        if target_compartment.state == '__history103_state_A':
            self.__history103_state_A(__e, target_compartment)
        elif target_compartment.state == '__history103_state_B':
            self.__history103_state_B(__e, target_compartment)
        elif target_compartment.state == '__history103_state_C':
            self.__history103_state_C(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    
    def __state_stack_push(self, compartment):
        self.__state_stack.append(compartment)
    
    def __state_stack_pop(self):
        return self.__state_stack.pop()

if __name__ == '__main__':
    main()
