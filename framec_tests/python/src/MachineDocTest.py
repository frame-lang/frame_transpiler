#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


class HelloWorldSystem:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = HelloWorldSystemCompartment('__helloworldsystem_state_Hello', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def sayHello(self,):
        self.return_stack.append(None)
        __e = FrameEvent("sayHello",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def sayWorld(self,):
        self.return_stack.append(None)
        __e = FrameEvent("sayWorld",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Hello
    
    def __helloworldsystem_state_Hello(self, __e, compartment):
        if __e._message == "sayHello":
            next_compartment = None
            next_compartment = HelloWorldSystemCompartment('__helloworldsystem_state_World', next_compartment)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $World
    
    def __helloworldsystem_state_World(self, __e, compartment):
        if __e._message == "sayWorld":
            next_compartment = None
            next_compartment = HelloWorldSystemCompartment('__helloworldsystem_state_Done', next_compartment)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Done
    
    def __helloworldsystem_state_Done(self, __e, compartment):
        pass
        
    
    
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
        if target_compartment.state == '__helloworldsystem_state_Hello':
            self.__helloworldsystem_state_Hello(__e, target_compartment)
        elif target_compartment.state == '__helloworldsystem_state_World':
            self.__helloworldsystem_state_World(__e, target_compartment)
        elif target_compartment.state == '__helloworldsystem_state_Done':
            self.__helloworldsystem_state_Done(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class HelloWorldSystemCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    