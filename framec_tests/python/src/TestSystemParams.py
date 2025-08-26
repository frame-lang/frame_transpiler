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
    sys1 = NoParamsSystem()
    sys2 = StartStateParameters("hello")
    sys3 = StartStateEnterParameters("world")
    sys4 = DomainVariables(1,2)
    sys5 = AllParameterTypes("hello","world",1,2)
    return
class NoParamsSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__noparamssystem_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __noparamssystem_state_Start(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("NoParamsSystem started")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__noparamssystem_state_Start(__e, None)
    
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
        if target_compartment.state == '__noparamssystem_state_Start':
            self.__noparamssystem_state_Start(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class StartStateParameters:
    def __init__(self, arg0):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__startstateparameters_state_S1', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        self.__compartment.state_args = {"p1": arg0}
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $S1
    
    def __startstateparameters_state_S1(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print((compartment.state_args["p1"]))
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sS1(self, __e):
        return self.__startstateparameters_state_S1(__e, None)
    
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
        if target_compartment.state == '__startstateparameters_state_S1':
            self.__startstateparameters_state_S1(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class StartStateEnterParameters:
    def __init__(self, arg0):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__startstateenterparameters_state_S1', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        enter_params = {"p1": arg0}
        frame_event = FrameEvent("$>", enter_params)
        self.__kernel(frame_event)
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $S1
    
    def __startstateenterparameters_state_S1(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(__e._parameters["p1"])
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sS1(self, __e):
        return self.__startstateenterparameters_state_S1(__e, None)
    
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
        if target_compartment.state == '__startstateenterparameters_state_S1':
            self.__startstateenterparameters_state_S1(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class DomainVariables:
    def __init__(self, arg0, arg1):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__domainvariables_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.a = arg0
        self.b = None
        self.c = arg1
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __domainvariables_state_Start(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print(a + c)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__domainvariables_state_Start(__e, None)
    
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
        if target_compartment.state == '__domainvariables_state_Start':
            self.__domainvariables_state_Start(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class AllParameterTypes:
    def __init__(self, arg0, arg1, arg2, arg3):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__allparametertypes_state_S1', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        self.__compartment.state_args = {"p1": arg0}
        # Initialize domain variables
        self.a = arg2
        self.b = None
        self.c = arg3
        
        # Send system start event
        enter_params = {"p2": arg1}
        frame_event = FrameEvent("$>", enter_params)
        self.__kernel(frame_event)
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $S1
    
    def __allparametertypes_state_S1(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print((compartment.state_args["p1"]) + __e._parameters["p2"] + a + c)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sS1(self, __e):
        return self.__allparametertypes_state_S1(__e, None)
    
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
        if target_compartment.state == '__allparametertypes_state_S1':
            self.__allparametertypes_state_S1(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()