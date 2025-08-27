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
    print("Starting test")
    controller = Controller()
    print("Testing SystemA:")
    sysA = SystemA()
    sysA.step()
    sysA.step()
    print("Testing SystemB:")
    sysB = SystemB()
    sysB.step()
    sysB.step()
    print("Test complete")
    return
class Controller:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__controller_state_Active', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def manage(self,):
        self.return_stack.append(None)
        __e = FrameEvent("manage",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Active
    
    def __controller_state_Active(self, __e, compartment):
        if __e._message == "manage":
            print("Controller managing")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sActive(self, __e):
        return self.__controller_state_Active(__e, None)
    
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
        if target_compartment.state == '__controller_state_Active':
            self.__controller_state_Active(__e, target_compartment)
    
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
    
    def step(self,):
        self.return_stack.append(None)
        __e = FrameEvent("step",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __systema_state_Start(self, __e, compartment):
        if __e._message == "step":
            print("SystemA: Start -> Middle")
            next_compartment = FrameCompartment('__systema_state_Middle', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Middle
    
    def __systema_state_Middle(self, __e, compartment):
        if __e._message == "step":
            print("SystemA: Middle -> End")
            next_compartment = FrameCompartment('__systema_state_End', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $End
    
    def __systema_state_End(self, __e, compartment):
        if __e._message == "step":
            print("SystemA: Already at End")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__systema_state_Start(__e, None)
    def _sMiddle(self, __e):
        return self.__systema_state_Middle(__e, None)
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
        elif target_compartment.state == '__systema_state_Middle':
            self.__systema_state_Middle(__e, target_compartment)
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
    
    def step(self,):
        self.return_stack.append(None)
        __e = FrameEvent("step",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __systemb_state_Start(self, __e, compartment):
        if __e._message == "step":
            print("SystemB: Start -> Middle")
            next_compartment = FrameCompartment('__systemb_state_Middle', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Middle
    
    def __systemb_state_Middle(self, __e, compartment):
        if __e._message == "step":
            print("SystemB: Middle -> End")
            next_compartment = FrameCompartment('__systemb_state_End', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $End
    
    def __systemb_state_End(self, __e, compartment):
        if __e._message == "step":
            print("SystemB: Already at End")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__systemb_state_Start(__e, None)
    def _sMiddle(self, __e):
        return self.__systemb_state_Middle(__e, None)
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
        elif target_compartment.state == '__systemb_state_Middle':
            self.__systemb_state_Middle(__e, target_compartment)
        elif target_compartment.state == '__systemb_state_End':
            self.__systemb_state_End(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
