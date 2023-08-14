# emitted from framec_v0.11.0
# get include files at https://github.com/frame-lang/frame-ancillary-files
from framelang.framelang import FrameEvent

class HierarchicalGuard:
    
    def __init__(self):
        
        # Create and intialize start state compartment.
        self.__state = self.__hierarchicalguard_state_I
        self.__compartment: 'HierarchicalGuardCompartment' = HierarchicalGuardCompartment(self.__state)
        self.__next_compartment: 'HierarchicalGuardCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def A(self,i: int):
        parameters = {}
        parameters["i"] = i

        e = FrameEvent("A",parameters)
        self.__mux(e)
    
    def B(self,i: int):
        parameters = {}
        parameters["i"] = i

        e = FrameEvent("B",parameters)
        self.__mux(e)
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__hierarchicalguard_state_I':
            self.__hierarchicalguard_state_I(e)
        elif self.__compartment.state.__name__ == '__hierarchicalguard_state_S':
            self.__hierarchicalguard_state_S(e)
        elif self.__compartment.state.__name__ == '__hierarchicalguard_state_S0':
            self.__hierarchicalguard_state_S0(e)
        elif self.__compartment.state.__name__ == '__hierarchicalguard_state_S1':
            self.__hierarchicalguard_state_S1(e)
        elif self.__compartment.state.__name__ == '__hierarchicalguard_state_S2':
            self.__hierarchicalguard_state_S2(e)
        elif self.__compartment.state.__name__ == '__hierarchicalguard_state_S3':
            self.__hierarchicalguard_state_S3(e)
        elif self.__compartment.state.__name__ == '__hierarchicalguard_state_S4':
            self.__hierarchicalguard_state_S4(e)
        
        if self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            if(next_compartment.forward_event is not None and 
               next_compartment.forward_event._message == ">"):
                self.__mux(FrameEvent( "<", self.__compartment.exit_args))
                self.__compartment = next_compartment
                self.__mux(next_compartment.forward_event)
            else:
                self.__do_transition(next_compartment)
                if next_compartment.forward_event is not None:
                    self.__mux(next_compartment.forward_event)
            next_compartment.forward_event = None
    
    # ===================== Machine Block =================== #
    
    def __hierarchicalguard_state_I(self, e):
        if e._message == ">":
            compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S)
            self.__transition(compartment)
            
            return
        
    def __hierarchicalguard_state_S(self, e):
        if e._message == "A":
            self.log_do("S.A")
            if  e._parameters["i"] < 10:
                compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S0)
                self.__transition(compartment)
                return
            else:
                compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S1)
                self.__transition(compartment)
                return
            
            
            return
        
        elif e._message == "B":
            self.log_do("S.B")
            if  e._parameters["i"] < 10:
                compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S2)
                self.__transition(compartment)
                return
            else:
                compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S3)
                self.__transition(compartment)
                return
            
            
            return
        
    def __hierarchicalguard_state_S0(self, e):
        if e._message == "A":
            self.log_do("S0.A")
            if  e._parameters["i"] > 0:
                compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S2)
                self.__transition(compartment)
                return
            else:
                pass
            
            
        
          #  fall through else branch
        elif e._message == "B":
            self.log_do("S0.B")
            if  e._parameters["i"] > 0:
                pass
            else:
                compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S1)
                self.__transition(compartment)
                return
            
            
        
        self.__hierarchicalguard_state_S(e)
        
      #  fall through then branch
    
    def __hierarchicalguard_state_S1(self, e):
        if e._message == "A":
            self.log_do("S1.A")
            if  e._parameters["i"] > 5:
                compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S3)
                self.__transition(compartment)
                return
            else:
                pass
            
            
        
        self.__hierarchicalguard_state_S0(e)
        
      #  fall through else branch
    
    def __hierarchicalguard_state_S2(self, e):
        if e._message == "A":
            self.log_do("S2.A")
            if  e._parameters["i"] > 10:
                compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S4)
                self.__transition(compartment)
                return
            else:
                pass
            
            
        
          #  fall through then branch
        elif e._message == "B":
            self.log_do("S2.B")
            if  not (e._parameters["i"] > 10):
                pass
            else:
                compartment = HierarchicalGuardCompartment(self.__hierarchicalguard_state_S4)
                self.__transition(compartment)
                return
            
            
        
        self.__hierarchicalguard_state_S1(e)
        
      #  fall through then branch
    
    def __hierarchicalguard_state_S3(self, e):
        if e._message == "A":
            self.log_do("S3.A")
            if  e._parameters["i"] > 0:
                self.log_do("stop")
                
                return
            else:
                self.log_do("continue")
            
            
        
        elif e._message == "B":
            self.log_do("S3.B")
            if  e._parameters["i"] > 0:
                self.log_do("continue")
            else:
                self.log_do("stop")
                
                return
            
            
        
        self.__hierarchicalguard_state_S(e)
        
    def __hierarchicalguard_state_S4(self, e):
        pass
        
    
    # ===================== Actions Block =================== #
    
    
    
    def log_do(self,msg: str):
        raise NotImplementedError
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'HierarchicalGuardCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'HierarchicalGuardCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class HierarchicalGuardCompartment:

    def __init__(self, state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class HierarchicalGuardController(HierarchicalGuard):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,msg: str):
        #pass

# ********************

