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


def main():
    sys1 = SystemA()
    sys2 = SystemB("parameter")
    sys3 = SystemC()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys1# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys2# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys3
    return

def helper():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Helper function works")
    return
class SystemA:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systema_state_Idle', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def start(self,):
        self.return_stack.append(None)
        __e = FrameEvent("start",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def stop(self,):
        self.return_stack.append(None)
        __e = FrameEvent("stop",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Idle
    
    def __systema_state_Idle(self, __e, compartment):
        if __e._message == "start":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            helper()# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__systema_state_Running', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Running
    
    def __systema_state_Running(self, __e, compartment):
        if __e._message == "stop":# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__systema_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sIdle(self, __e):
        return self.__systema_state_Idle(__e, None)
    def _sRunning(self, __e):
        return self.__systema_state_Running(__e, None)
    
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
        if target_compartment.state == '__systema_state_Idle':
            self.__systema_state_Idle(__e, target_compartment)
        elif target_compartment.state == '__systema_state_Running':
            self.__systema_state_Running(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SystemB:
    def __init__(self, arg0):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systemb_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.value = ""
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def activate(self,data):
        parameters = {}
        parameters["data"] = data
        self.return_stack.append(None)
        __e = FrameEvent("activate",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __systemb_state_Start(self, __e, compartment):
        if __e._message == "activate":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(param)# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(__e._parameters["data"])# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__systemb_state_Active', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Active
    
    def __systemb_state_Active(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("SystemB active")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__systemb_state_Start(__e, None)
    def _sActive(self, __e):
        return self.__systemb_state_Active(__e, None)
    
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
        elif target_compartment.state == '__systemb_state_Active':
            self.__systemb_state_Active(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SystemC:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systemc_state_Begin', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def run(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("SystemC operation running")
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Begin
    
    def __systemc_state_Begin(self, __e, compartment):
        pass
        
    
    # ===================== State Dispatchers =================== #
    
    def _sBegin(self, __e):
        return self.__systemc_state_Begin(__e, None)
    # ===================== Actions Block =================== #
    
    def run_do(self):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("SystemC running")
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
        if target_compartment.state == '__systemc_state_Begin':
            self.__systemc_state_Begin(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()