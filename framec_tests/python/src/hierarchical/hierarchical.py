# emitted from framec_v0.11.0
# get include files at https://github.com/frame-lang/frame-ancillary-files


from framelang.framelang import FrameEvent




class Hierarchical:
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__hierarchical_state_I
        self.__compartment: 'HierarchicalCompartment' = HierarchicalCompartment(self.__state)
        self.__next_compartment: 'HierarchicalCompartment' = None
        
        # Initialize domain
        
        self.enters  = []
        self.exits  = []
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def A(self,):
        e = FrameEvent("A",None)
        self.__mux(e)
    
    def B(self,):
        e = FrameEvent("B",None)
        self.__mux(e)
    
    def C(self,):
        e = FrameEvent("C",None)
        self.__mux(e)
    
    # ===================== Machine Block =================== #
    
    def __hierarchical_state_I(self, e):
        if e._message == ">":
            compartment = HierarchicalCompartment(self.__hierarchical_state_S)
            self.__transition(compartment)
            
            return
        
    def __hierarchical_state_S(self, e):
        if e._message == ">":
            self.enter_do("S")
            
            return
        
        elif e._message == "<":
            self.exit_do("S")
            
            return
        
        elif e._message == "A":
            self.log_do("S.A")
            compartment = HierarchicalCompartment(self.__hierarchical_state_S0)
            self.__transition(compartment)
            
            return
        
        elif e._message == "B":
            self.log_do("S.B")
            compartment = HierarchicalCompartment(self.__hierarchical_state_S1)
            self.__transition(compartment)
            
            return
        
    def __hierarchical_state_S0(self, e):
        if e._message == ">":
            self.enter_do("S0")
            
        
        elif e._message == "<":
            self.exit_do("S0")
            
        
          #  override parent handler
        elif e._message == "A":
            self.log_do("S0.A")
            compartment = HierarchicalCompartment(self.__hierarchical_state_T)
            self.__transition(compartment)
            
            return
        
          #  do this, then parent handler
        elif e._message == "B":
            self.log_do("S0.B")
            
        
          #  extend parent handler
        elif e._message == "C":
            self.log_do("S0.C")
            compartment = HierarchicalCompartment(self.__hierarchical_state_S2)
            self.__transition(compartment)
            
            return
        
        self.__hierarchical_state_S(e)
        
    def __hierarchical_state_S1(self, e):
        if e._message == ">":
            self.enter_do("S1")
            
            return
        
        elif e._message == "<":
            self.exit_do("S1")
            
            return
        
          #  defer to parent for A
          #  do this, then parent, which transitions here
        elif e._message == "B":
            self.log_do("S1.B")
            
        
          #  propagate message not handled by parent
        elif e._message == "C":
            self.log_do("S1.C")
            
        
        self.__hierarchical_state_S(e)
        
    def __hierarchical_state_S2(self, e):
        if e._message == ">":
            self.enter_do("S2")
            
        
        elif e._message == "<":
            self.exit_do("S2")
            
        
          #  will propagate to S0 and S
        elif e._message == "B":
            self.log_do("S2.B")
            
        
        elif e._message == "C":
            self.log_do("S2.C")
            compartment = HierarchicalCompartment(self.__hierarchical_state_T)
            self.__transition(compartment)
            
            return
        
        self.__hierarchical_state_S0(e)
        
      #  continue after transition (should be ignored)
    
    def __hierarchical_state_S3(self, e):
        if e._message == ">":
            self.enter_do("S3")
            
        
        elif e._message == "<":
            self.exit_do("S3")
            
        
          #  defer to grandparent for A
          #  override and move to sibling
        elif e._message == "B":
            self.log_do("S3.B")
            compartment = HierarchicalCompartment(self.__hierarchical_state_S2)
            self.__transition(compartment)
            
            return
        
        self.__hierarchical_state_S1(e)
        
    def __hierarchical_state_T(self, e):
        if e._message == ">":
            self.enter_do("T")
            
            return
        
        elif e._message == "<":
            self.exit_do("T")
            
            return
        
        elif e._message == "A":
            self.log_do("T.A")
            compartment = HierarchicalCompartment(self.__hierarchical_state_S)
            self.__transition(compartment)
            
            return
        
        elif e._message == "B":
            self.log_do("T.B")
            compartment = HierarchicalCompartment(self.__hierarchical_state_S2)
            self.__transition(compartment)
            
            return
        
        elif e._message == "C":
            self.log_do("T.C")
            compartment = HierarchicalCompartment(self.__hierarchical_state_S3)
            self.__transition(compartment)
            
            return
        
    
    # ===================== Actions Block =================== #
    
    
    
    def enter_do(self,msg: str):
        raise NotImplementedError
    
    def exit_do(self,msg: str):
        raise NotImplementedError
    
    def log_do(self,msg: str):
        raise NotImplementedError
    
    
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__hierarchical_state_I':
            self.__hierarchical_state_I(e)
        elif self.__compartment.state.__name__ == '__hierarchical_state_S':
            self.__hierarchical_state_S(e)
        elif self.__compartment.state.__name__ == '__hierarchical_state_S0':
            self.__hierarchical_state_S0(e)
        elif self.__compartment.state.__name__ == '__hierarchical_state_S1':
            self.__hierarchical_state_S1(e)
        elif self.__compartment.state.__name__ == '__hierarchical_state_S2':
            self.__hierarchical_state_S2(e)
        elif self.__compartment.state.__name__ == '__hierarchical_state_S3':
            self.__hierarchical_state_S3(e)
        elif self.__compartment.state.__name__ == '__hierarchical_state_T':
            self.__hierarchical_state_T(e)
        
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
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'HierarchicalCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'HierarchicalCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class HierarchicalCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class HierarchicalController(Hierarchical):
	#def __init__(self,):
	    #super().__init__()

    #def enter_do(self,msg: str):
        #pass

    #def exit_do(self,msg: str):
        #pass

    #def log_do(self,msg: str):
        #pass

# ********************

