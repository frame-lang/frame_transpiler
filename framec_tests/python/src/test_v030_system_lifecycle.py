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
    print("=== Starting System Lifecycle Test ===")
    mainSys = MainSystem()
    print("\n--- Cycle 1: StateA ---")
    mainSys.next()
    mainSys.next()
    mainSys.next()
    print("\n--- Cycle 1: StateB ---")
    mainSys.next()
    mainSys.next()
    mainSys.next()
    print("\n--- Cycle 2: StateA ---")
    mainSys.next()
    mainSys.next()
    mainSys.next()
    print("\n--- Cycle 2: StateB ---")
    mainSys.next()
    mainSys.next()
    mainSys.next()
    print("\n=== System Lifecycle Test Complete ===")
    return
class MainSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__mainsystem_state_StateA', None, None, None, None)
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
    # $StateA
    
    def __mainsystem_state_StateA(self, __e, compartment):
        if __e._message == "$>":
            print("MainSystem: Entering StateA")
            (compartment.state_vars["sysA"]) = SystemA()
            print("MainSystem: Created SystemA instance")
            return
        elif __e._message == "<$":
            print("MainSystem: Exiting StateA")
            (compartment.state_vars["sysA"]) = None
            print("MainSystem: Destroyed SystemA instance")
            return
        elif __e._message == "next":
            continueProcessing = compartment.state_vars["sysA"].next()
            if ( not continueProcessing):
                print("MainSystem: SystemA complete, transitioning to StateB")
                next_compartment = FrameCompartment('__mainsystem_state_StateB', None, None, None, None)
                self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $StateB
    
    def __mainsystem_state_StateB(self, __e, compartment):
        if __e._message == "$>":
            print("MainSystem: Entering StateB")
            (compartment.state_vars["sysB"]) = SystemB()
            print("MainSystem: Created SystemB instance")
            return
        elif __e._message == "<$":
            print("MainSystem: Exiting StateB")
            (compartment.state_vars["sysB"]) = None
            print("MainSystem: Destroyed SystemB instance")
            return
        elif __e._message == "next":
            continueProcessing = compartment.state_vars["sysB"].next()
            if ( not continueProcessing):
                print("MainSystem: SystemB complete, transitioning to StateA")
                next_compartment = FrameCompartment('__mainsystem_state_StateA', None, None, None, None)
                self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStateA(self, __e):
        return self.__mainsystem_state_StateA(__e, None)
    def _sStateB(self, __e):
        return self.__mainsystem_state_StateB(__e, None)
    
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
        if target_compartment.state == '__mainsystem_state_StateA':
            self.__mainsystem_state_StateA(__e, target_compartment)
        elif target_compartment.state == '__mainsystem_state_StateB':
            self.__mainsystem_state_StateB(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SystemA:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systema_state_Start', None, None, None, None)
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
    
    def __systema_state_Start(self, __e, compartment):
        if __e._message == "$>":
            print("SystemA: Entering Start state")
            return
        elif __e._message == "<$":
            print("SystemA: Exiting Start state")
            return
        elif __e._message == "next":
            print("SystemA: Start.next() -> Working (returning true)")
            self.return_stack[-1] = True
            next_compartment = FrameCompartment('__systema_state_Working', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Working
    
    def __systema_state_Working(self, __e, compartment):
        if __e._message == "$>":
            print("SystemA: Entering Working state")
            return
        elif __e._message == "<$":
            print("SystemA: Exiting Working state")
            return
        elif __e._message == "next":
            print("SystemA: Working.next() -> End (returning true)")
            self.return_stack[-1] = True
            next_compartment = FrameCompartment('__systema_state_End', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $End
    
    def __systema_state_End(self, __e, compartment):
        if __e._message == "$>":
            print("SystemA: Entering End state")
            return
        elif __e._message == "<$":
            print("SystemA: Exiting End state")
            return
        elif __e._message == "next":
            print("SystemA: End.next() - complete (returning false)")
            self.return_stack[-1] = False
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__systema_state_Start(__e, None)
    def _sWorking(self, __e):
        return self.__systema_state_Working(__e, None)
    def _sEnd(self, __e):
        return self.__systema_state_End(__e, None)
    
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
        if target_compartment.state == '__systema_state_Start':
            self.__systema_state_Start(__e, target_compartment)
        elif target_compartment.state == '__systema_state_Working':
            self.__systema_state_Working(__e, target_compartment)
        elif target_compartment.state == '__systema_state_End':
            self.__systema_state_End(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SystemB:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systemb_state_Start', None, None, None, None)
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
    
    def __systemb_state_Start(self, __e, compartment):
        if __e._message == "$>":
            print("SystemB: Entering Start state")
            return
        elif __e._message == "<$":
            print("SystemB: Exiting Start state")
            return
        elif __e._message == "next":
            print("SystemB: Start.next() -> Working (returning true)")
            self.return_stack[-1] = True
            next_compartment = FrameCompartment('__systemb_state_Working', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Working
    
    def __systemb_state_Working(self, __e, compartment):
        if __e._message == "$>":
            print("SystemB: Entering Working state")
            return
        elif __e._message == "<$":
            print("SystemB: Exiting Working state")
            return
        elif __e._message == "next":
            print("SystemB: Working.next() -> End (returning true)")
            self.return_stack[-1] = True
            next_compartment = FrameCompartment('__systemb_state_End', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $End
    
    def __systemb_state_End(self, __e, compartment):
        if __e._message == "$>":
            print("SystemB: Entering End state")
            return
        elif __e._message == "<$":
            print("SystemB: Exiting End state")
            return
        elif __e._message == "next":
            print("SystemB: End.next() - complete (returning false)")
            self.return_stack[-1] = False
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__systemb_state_Start(__e, None)
    def _sWorking(self, __e):
        return self.__systemb_state_Working(__e, None)
    def _sEnd(self, __e):
        return self.__systemb_state_End(__e, None)
    
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
        if target_compartment.state == '__systemb_state_Start':
            self.__systemb_state_Start(__e, target_compartment)
        elif target_compartment.state == '__systemb_state_Working':
            self.__systemb_state_Working(__e, target_compartment)
        elif target_compartment.state == '__systemb_state_End':
            self.__systemb_state_End(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
