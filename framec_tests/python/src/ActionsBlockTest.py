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
    hws = HelloWorldSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hws.sayHello()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hws.sayWorld()
    return
class HelloWorldSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__helloworldsystem_state_Hello', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
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
        if __e._message == "sayHello":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            actionWriteHello()# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__helloworldsystem_state_World', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $World
    
    def __helloworldsystem_state_World(self, __e, compartment):
        if __e._message == "sayWorld":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            actionWriteWorld()# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__helloworldsystem_state_Done', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Done
    
    def __helloworldsystem_state_Done(self, __e, compartment):
        pass
        
    
    # ===================== State Dispatchers =================== #
    
    def _sHello(self, __e):
        return self.__helloworldsystem_state_Hello(__e, None)
    def _sWorld(self, __e):
        return self.__helloworldsystem_state_World(__e, None)
    def _sDone(self, __e):
        return self.__helloworldsystem_state_Done(__e, None)
    # ===================== Actions Block =================== #
    
    def actionWriteHello_do(self):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        actionWrite("Hello"," ")
        return
        
    
    def actionWriteWorld_do(self):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        actionWrite("World!","")
        return
        
    
    def actionWrite_do(self,msg,separator):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print(msg,end = separator)
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
        if target_compartment.state == '__helloworldsystem_state_Hello':
            self.__helloworldsystem_state_Hello(__e, target_compartment)
        elif target_compartment.state == '__helloworldsystem_state_World':
            self.__helloworldsystem_state_World(__e, target_compartment)
        elif target_compartment.state == '__helloworldsystem_state_Done':
            self.__helloworldsystem_state_Done(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()