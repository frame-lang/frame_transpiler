#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    demo = StateVariableDemo()
    return

class StateVariableDemo:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = StateVariableDemoCompartment('__statevariabledemo_state_JoeName', next_compartment)
        next_compartment.state_vars["name"] = "Joe"
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        next_compartment.state_vars["name"] = "Joe"
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def print(self,):
        self.return_stack.append(None)
        __e = FrameEvent("print",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def updateName(self,newName: string):
        parameters = {}
        parameters["newName"] = newName
        self.return_stack.append(None)
        __e = FrameEvent("updateName",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def forgetMe(self,):
        self.return_stack.append(None)
        __e = FrameEvent("forgetMe",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $JoeName
    
    def __statevariabledemo_state_JoeName(self, __e, compartment):
        if __e._message == "print":
            self.print_do((compartment.state_vars["name"]))
            return
        elif __e._message == "updateName":
            (compartment.state_vars["name"]) = __e._parameters["newName"]
            return
        elif __e._message == "forgetMe":
            next_compartment = None
            next_compartment = StateVariableDemoCompartment('__statevariabledemo_state_ResetName', next_compartment)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $ResetName
    
    def __statevariabledemo_state_ResetName(self, __e, compartment):
        if __e._message == "$>":
            next_compartment = None
            next_compartment = StateVariableDemoCompartment('__statevariabledemo_state_JoeName', next_compartment)
            next_compartment.state_vars["name"] = "Joe"
            self.__transition(next_compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def print_do(self,msg: string):
        pass
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
        if target_compartment.state == '__statevariabledemo_state_JoeName':
            self.__statevariabledemo_state_JoeName(__e, target_compartment)
        elif target_compartment.state == '__statevariabledemo_state_ResetName':
            self.__statevariabledemo_state_ResetName(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class StateVariableDemoCompartment:

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
