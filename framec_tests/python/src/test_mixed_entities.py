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
    print("=== Mixed Entity Test ===")
    module_var = "MODULE"
    print(module_var)
    function_one()
    function_two()
    s1 = SystemA()
    s2 = SystemB()
    s1.interface_a()
    s2.interface_b()
    print("Mixed entity test completed")
    return

def function_one():
    print("Function One")
    local_one = "F1"
    print(local_one)
    helper()
    return

def function_two():
    print("Function Two")
    local_two = "F2"
    print(local_two)
    helper()
    return

def helper():
    print("Helper function")
    return
class SystemA:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systema_state_StateA', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def interface_a(self,):
        self.return_stack.append(None)
        __e = FrameEvent("interface_a",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $StateA
    
    def __systema_state_StateA(self, __e, compartment):
        if __e._message == "interface_a":
            print("SystemA interface")
            action_a()
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStateA(self, __e):
        return self.__systema_state_StateA(__e, None)
    # ===================== Actions Block =================== #
    
    def action_a_do(self):
        
        print("SystemA action")
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
        if target_compartment.state == '__systema_state_StateA':
            self.__systema_state_StateA(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SystemB:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__systemb_state_StateB', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def interface_b(self,):
        self.return_stack.append(None)
        __e = FrameEvent("interface_b",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $StateB
    
    def __systemb_state_StateB(self, __e, compartment):
        if __e._message == "interface_b":
            print("SystemB interface")
            action_b()
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStateB(self, __e):
        return self.__systemb_state_StateB(__e, None)
    # ===================== Actions Block =================== #
    
    def action_b_do(self):
        
        print("SystemB action")
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
        if target_compartment.state == '__systemb_state_StateB':
            self.__systemb_state_StateB(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
