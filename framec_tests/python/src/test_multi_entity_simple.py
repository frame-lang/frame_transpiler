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


def helper(msg):
    print("Helper says: " + msg)
    return "processed"
    return

def log_event(info):
    print("[LOG] " + info)
    return

def main():
    print("=== Simple Multi-Entity Demo ===")
    result = helper("hello")
    print("Result: " + result)
    toggle = ToggleSwitch()
    toggle.flip()
    toggle.flip()
    toggle.flip()
    machine = SimpleStateMachine()
    machine.advance()
    machine.advance()
    machine.advance()
    print("=== Demo Complete ===")
    return
class ToggleSwitch:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__toggleswitch_state_Off', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def flip(self,):
        self.return_stack.append(None)
        __e = FrameEvent("flip",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Off
    
    def __toggleswitch_state_Off(self, __e, compartment):
        if __e._message == "flip":
            log_event("Switch: OFF -> ON")
            next_compartment = FrameCompartment('__toggleswitch_state_On', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "$>":
            print("Switch initialized to OFF")
            return
    
    
    # ----------------------------------------
    # $On
    
    def __toggleswitch_state_On(self, __e, compartment):
        if __e._message == "flip":
            log_event("Switch: ON -> OFF")
            next_compartment = FrameCompartment('__toggleswitch_state_Off', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "$>":
            print("Now ON")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sOff(self, __e):
        return self.__toggleswitch_state_Off(__e, None)
    def _sOn(self, __e):
        return self.__toggleswitch_state_On(__e, None)
    
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
        if target_compartment.state == '__toggleswitch_state_Off':
            self.__toggleswitch_state_Off(__e, target_compartment)
        elif target_compartment.state == '__toggleswitch_state_On':
            self.__toggleswitch_state_On(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SimpleStateMachine:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__simplestatemachine_state_StateA', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def advance(self,):
        self.return_stack.append(None)
        __e = FrameEvent("advance",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $StateA
    
    def __simplestatemachine_state_StateA(self, __e, compartment):
        if __e._message == "advance":
            print("State A -> B")
            next_compartment = FrameCompartment('__simplestatemachine_state_StateB', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "$>":
            print("Starting in State A")
            return
    
    
    # ----------------------------------------
    # $StateB
    
    def __simplestatemachine_state_StateB(self, __e, compartment):
        if __e._message == "advance":
            print("State B -> C")
            next_compartment = FrameCompartment('__simplestatemachine_state_StateC', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "$>":
            print("Entered State B")
            return
    
    
    # ----------------------------------------
    # $StateC
    
    def __simplestatemachine_state_StateC(self, __e, compartment):
        if __e._message == "advance":
            print("State C -> A (cycling back)")
            next_compartment = FrameCompartment('__simplestatemachine_state_StateA', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "$>":
            print("Entered State C")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStateA(self, __e):
        return self.__simplestatemachine_state_StateA(__e, None)
    def _sStateB(self, __e):
        return self.__simplestatemachine_state_StateB(__e, None)
    def _sStateC(self, __e):
        return self.__simplestatemachine_state_StateC(__e, None)
    
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
        if target_compartment.state == '__simplestatemachine_state_StateA':
            self.__simplestatemachine_state_StateA(__e, target_compartment)
        elif target_compartment.state == '__simplestatemachine_state_StateB':
            self.__simplestatemachine_state_StateB(__e, target_compartment)
        elif target_compartment.state == '__simplestatemachine_state_StateC':
            self.__simplestatemachine_state_StateC(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
