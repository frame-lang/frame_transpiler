



from framelang.framelang import FrameEvent


# Emitted from framec_v0.11.0

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class HierarchicalGuard:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__compartment: 'HierarchicalGuardCompartment' = HierarchicalGuardCompartment('__hierarchicalguard_state_I')
        self.__next_compartment: 'HierarchicalGuardCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def A(self,i: int):
        parameters = {}
        parameters["i"] = i
        __e = FrameEvent("A",parameters)
        self.__kernel(__e)
    
    def B(self,i: int):
        parameters = {}
        parameters["i"] = i
        __e = FrameEvent("B",parameters)
        self.__kernel(__e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $I
    
    def __hierarchicalguard_state_I(self, __e):
        if __e._message == ">":
            compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S')
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $S
    
    def __hierarchicalguard_state_S(self, __e):
        if __e._message == "A":
            self.log_do("S.A")
            if  __e._parameters["i"] < 10:
                compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S0')
                self.__transition(compartment)
                return
            else:
                compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S1')
                self.__transition(compartment)
                return
            
            return
        elif __e._message == "B":
            self.log_do("S.B")
            if  __e._parameters["i"] < 10:
                compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S2')
                self.__transition(compartment)
                return
            else:
                compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S3')
                self.__transition(compartment)
                return
            
            return
    
    # ----------------------------------------
    # $S0
    
    def __hierarchicalguard_state_S0(self, __e):
        if __e._message == "A":
            self.log_do("S0.A")
            if  __e._parameters["i"] > 0:
                compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S2')
                self.__transition(compartment)
                return
            else:
                pass
            
          #  fall through else branch
        elif __e._message == "B":
            self.log_do("S0.B")
            if  __e._parameters["i"] > 0:
                pass
            else:
                compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S1')
                self.__transition(compartment)
                return
            
        
        self.__hierarchicalguard_state_S(__e)
        
      #  fall through then branch
    
    
    # ----------------------------------------
    # $S1
    
    def __hierarchicalguard_state_S1(self, __e):
        if __e._message == "A":
            self.log_do("S1.A")
            if  __e._parameters["i"] > 5:
                compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S3')
                self.__transition(compartment)
                return
            else:
                pass
            
        
        self.__hierarchicalguard_state_S0(__e)
        
      #  fall through else branch
    
    
    # ----------------------------------------
    # $S2
    
    def __hierarchicalguard_state_S2(self, __e):
        if __e._message == "A":
            self.log_do("S2.A")
            if  __e._parameters["i"] > 10:
                compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S4')
                self.__transition(compartment)
                return
            else:
                pass
            
          #  fall through then branch
        elif __e._message == "B":
            self.log_do("S2.B")
            if  not (__e._parameters["i"] > 10):
                pass
            else:
                compartment = HierarchicalGuardCompartment('__hierarchicalguard_state_S4')
                self.__transition(compartment)
                return
            
        
        self.__hierarchicalguard_state_S1(__e)
        
      #  fall through then branch
    
    
    # ----------------------------------------
    # $S3
    
    def __hierarchicalguard_state_S3(self, __e):
        if __e._message == "A":
            self.log_do("S3.A")
            if  __e._parameters["i"] > 0:
                self.log_do("stop")
                
                return
            else:
                self.log_do("continue")
            
        elif __e._message == "B":
            self.log_do("S3.B")
            if  __e._parameters["i"] > 0:
                self.log_do("continue")
            else:
                self.log_do("stop")
                
                return
            
        
        self.__hierarchicalguard_state_S(__e)
        
    
    # ----------------------------------------
    # $S4
    
    def __hierarchicalguard_state_S4(self, __e):
        pass
        
    
    # ===================== Actions Block =================== #
    
    def log_do(self,msg: str):
        raise NotImplementedError
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent( "<", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent(">", self.__compartment.enter_args))
            else: # there is a forwarded event
                if next_compartment.forward_event._message == ">":
                    # forwarded event is enter event
                    self.__router(next_compartment.forward_event)
                else:
                    # forwarded event is not enter event
                    # send normal enter event
                    self.__router(FrameEvent(">", self.__compartment.enter_args))
                    # and now forward event to new, intialized state
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
                
    
    def __router(self, __e):
        if self.__compartment.state == '__hierarchicalguard_state_I':
            self.__hierarchicalguard_state_I(__e)
        elif self.__compartment.state == '__hierarchicalguard_state_S':
            self.__hierarchicalguard_state_S(__e)
        elif self.__compartment.state == '__hierarchicalguard_state_S0':
            self.__hierarchicalguard_state_S0(__e)
        elif self.__compartment.state == '__hierarchicalguard_state_S1':
            self.__hierarchicalguard_state_S1(__e)
        elif self.__compartment.state == '__hierarchicalguard_state_S2':
            self.__hierarchicalguard_state_S2(__e)
        elif self.__compartment.state == '__hierarchicalguard_state_S3':
            self.__hierarchicalguard_state_S3(__e)
        elif self.__compartment.state == '__hierarchicalguard_state_S4':
            self.__hierarchicalguard_state_S4(__e)
        
    def __transition(self, compartment: 'HierarchicalGuardCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class HierarchicalGuardCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    