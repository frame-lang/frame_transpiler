#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    sys = ContinueTerminatorDemo()
    sys.passMe1()
    sys.passMe2()
    return

class ContinueTerminatorDemo:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = ContinueTerminatorDemoCompartment('__continueterminatordemo_state_Parent', next_compartment)
        next_compartment = ContinueTerminatorDemoCompartment('__continueterminatordemo_state_Child', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def passMe1(self,):
        self.return_stack.append(None)
        __e = FrameEvent("passMe1",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def passMe2(self,):
        self.return_stack.append(None)
        __e = FrameEvent("passMe2",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Child
    
    def __continueterminatordemo_state_Child(self, __e, compartment):
        if __e._message == "passMe1":
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            return
        elif __e._message == "passMe2":
            print("handled in $Child")
            # => $^ parent dispatch
            self.__router(__e, compartment.parent_compartment)
            if self.__next_compartment is not None:
                return
            return
        
        self.__router(__e, compartment.parent_compartment)
        
    
    
    # ----------------------------------------
    # $Parent
    
    def __continueterminatordemo_state_Parent(self, __e, compartment):
        if __e._message == "passMe1":
            print("handled in $Parent")
            return
        elif __e._message == "passMe2":
            print("handled in $Parent")
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
            self.__router(FrameEvent( "<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else: # there is a forwarded event
                if next_compartment.forward_event._message == "$>":
                    # forwarded event is enter event
                    self.__router(next_compartment.forward_event)
                else:
                    # forwarded event is not enter event
                    # send normal enter event
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
                    # and now forward event to new, intialized state
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
                
    
    def __router(self, __e, compartment=None):
        target_compartment = compartment or self.__compartment
        if target_compartment.state == '__continueterminatordemo_state_Child':
            self.__continueterminatordemo_state_Child(__e, target_compartment)
        elif target_compartment.state == '__continueterminatordemo_state_Parent':
            self.__continueterminatordemo_state_Parent(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class ContinueTerminatorDemoCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    
if __name__ == '__main__':
    main()
