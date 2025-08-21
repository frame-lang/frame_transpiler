#Emitted from framec_v0.30.0


import time

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    looper = Looper(10)
    return

class Looper:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self,start_state_enter_param_loops):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = LooperCompartment('__looper_state_Start', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        self.__compartment.enter_args["loops"] = start_state_enter_param_loops
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent("$>", self.__compartment.enter_args)
        self.__kernel(frame_event)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __looper_state_Start(self, __e, compartment):
        if __e._message == "$>":
            print("Starting")
            next_compartment = None
            next_compartment = LooperCompartment('__looper_state_A', next_compartment)
            next_compartment.enter_args["total_loops"] = __e._parameters["loops"]
            next_compartment.enter_args["loops_left"] = __e._parameters["loops"]
            next_compartment.enter_args["start"] = time.time()
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $A
    
    def __looper_state_A(self, __e, compartment):
        if __e._message == "$>":
            if __e._parameters["loops_left"] == 0:
                next_compartment = None
                next_compartment = LooperCompartment('__looper_state_Done', next_compartment)
                next_compartment.enter_args["total_loops"] = __e._parameters["total_loops"]
                next_compartment.enter_args["start"] = __e._parameters["start"]
                self.__transition(next_compartment)
            next_compartment = None
            next_compartment = LooperCompartment('__looper_state_B', next_compartment)
            next_compartment.enter_args["total_loops"] = __e._parameters["total_loops"]
            next_compartment.enter_args["loops_left"] = __e._parameters["loops_left"]
            next_compartment.enter_args["start"] = __e._parameters["start"]
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $B
    
    def __looper_state_B(self, __e, compartment):
        if __e._message == "$>":
            __e._parameters["loops_left"] = __e._parameters["loops_left"] - 1
            next_compartment = None
            next_compartment = LooperCompartment('__looper_state_A', next_compartment)
            next_compartment.enter_args["total_loops"] = __e._parameters["total_loops"]
            next_compartment.enter_args["loops_left"] = __e._parameters["loops_left"]
            next_compartment.enter_args["start"] = __e._parameters["start"]
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Done
    
    def __looper_state_Done(self, __e, compartment):
        if __e._message == "$>":
            print("Done. Looped " + str(__e._parameters["total_loops"]) + " times in ",end = " ")
            print(str(time.time() - __e._parameters["start"]) + " seconds.")
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
        if target_compartment.state == '__looper_state_Start':
            self.__looper_state_Start(__e, target_compartment)
        elif target_compartment.state == '__looper_state_A':
            self.__looper_state_A(__e, target_compartment)
        elif target_compartment.state == '__looper_state_B':
            self.__looper_state_B(__e, target_compartment)
        elif target_compartment.state == '__looper_state_Done':
            self.__looper_state_Done(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class LooperCompartment:

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
