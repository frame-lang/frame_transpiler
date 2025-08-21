#Emitted from framec_v0.30.0


import time
import signal
import sys

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    service = SignalMachineService()
    return

class SignalMachineService:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = SignalMachineServiceCompartment('__signalmachineservice_state_Init', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def quit(self,):
        self.return_stack.append(None)
        __e = FrameEvent("quit",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Init
    
    def __signalmachineservice_state_Init(self, __e, compartment):
        if __e._message == "$>":
            signal.signal(signal.SIGINT,self.signal_handler)
            next_compartment = None
            next_compartment = SignalMachineServiceCompartment('__signalmachineservice_state_Done', next_compartment)
            next_compartment = SignalMachineServiceCompartment('__signalmachineservice_state_A', next_compartment)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $A
    
    def __signalmachineservice_state_A(self, __e, compartment):
        if __e._message == "$>":
            print("$A")
            time.sleep(.2)
            next_compartment = None
            next_compartment = SignalMachineServiceCompartment('__signalmachineservice_state_Done', next_compartment)
            next_compartment = SignalMachineServiceCompartment('__signalmachineservice_state_B', next_compartment)
            self.__transition(next_compartment)
            return
        
        self.__router(__e, compartment.parent_compartment)
        
    
    
    # ----------------------------------------
    # $B
    
    def __signalmachineservice_state_B(self, __e, compartment):
        if __e._message == "$>":
            print("$B")
            time.sleep(.2)
            next_compartment = None
            next_compartment = SignalMachineServiceCompartment('__signalmachineservice_state_Done', next_compartment)
            next_compartment = SignalMachineServiceCompartment('__signalmachineservice_state_A', next_compartment)
            self.__transition(next_compartment)
            return
        
        self.__router(__e, compartment.parent_compartment)
        
    
    
    # ----------------------------------------
    # $Done
    
    def __signalmachineservice_state_Done(self, __e, compartment):
        if __e._message == "quit":
            print("Goodbye!")
            sys.exit(0)
            return
    
    
    # ==================== Operations Block ================== #
    
    def signal_handler(self,sig,frame):
        quit()
    
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
        if target_compartment.state == '__signalmachineservice_state_Init':
            self.__signalmachineservice_state_Init(__e, target_compartment)
        elif target_compartment.state == '__signalmachineservice_state_A':
            self.__signalmachineservice_state_A(__e, target_compartment)
        elif target_compartment.state == '__signalmachineservice_state_B':
            self.__signalmachineservice_state_B(__e, target_compartment)
        elif target_compartment.state == '__signalmachineservice_state_Done':
            self.__signalmachineservice_state_Done(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class SignalMachineServiceCompartment:

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
