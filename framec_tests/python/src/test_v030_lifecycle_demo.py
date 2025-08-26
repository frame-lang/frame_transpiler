#Emitted from framec_v0.30.0


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


def main():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== System Lifecycle Management Demo ===")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("This demonstrates the v0.30 multi-entity architecture")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("with MainSystem managing the lifecycle of SystemA and SystemB")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Creating MainSystem...")
    mainSys = MainSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Conceptual Flow:")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("1. MainSystem starts in StateA")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("   - Creates SystemA instance")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("   - Drives SystemA through states: Start -> Working -> End")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("   - When SystemA reaches End, transitions to StateB")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("2. MainSystem transitions to StateB")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("   - Destroys SystemA instance")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("   - Creates SystemB instance")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("   - Drives SystemB through states: Start -> Working -> End")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("   - When SystemB reaches End, transitions back to StateA")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("3. Cycle repeats as needed")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("System Structure:")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("- MainSystem: Orchestrator with StateA and StateB")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("- SystemA: Worker with Start, Working, End states")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("- SystemB: Worker with Start, Working, End states")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("v0.30 Features Demonstrated:")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("- Multiple systems in single file")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("- Multiple functions in single file")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("- State-scoped variables")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("- System instantiation and lifecycle management")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("- Interface method return values")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== Demo Complete ===")
    return

def logTransition(fromState,toState):# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Transition: " + fromState + " -> " + toState)
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
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            logTransition("","StateA")
            return
        elif __e._message == "<$":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            logTransition("StateA","")
            return
        elif __e._message == "next":# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__mainsystem_state_StateB', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $StateB
    
    def __mainsystem_state_StateB(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            logTransition("","StateB")
            return
        elif __e._message == "<$":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            logTransition("StateB","")
            return
        elif __e._message == "next":# DEBUG: TransitionStmt
            
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
        if __e._message == "next":
            self.return_stack[-1] = True# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__systema_state_Working', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Working
    
    def __systema_state_Working(self, __e, compartment):
        if __e._message == "next":
            self.return_stack[-1] = True# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__systema_state_End', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $End
    
    def __systema_state_End(self, __e, compartment):
        if __e._message == "next":
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
        if __e._message == "next":
            self.return_stack[-1] = True# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__systemb_state_Working', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Working
    
    def __systemb_state_Working(self, __e, compartment):
        if __e._message == "next":
            self.return_stack[-1] = True# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__systemb_state_End', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $End
    
    def __systemb_state_End(self, __e, compartment):
        if __e._message == "next":
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