# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class Hierarchical:
    
    
    # ==================== System Factory =================== #
    
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
        self.__kernel(frame_event)
    
    # ===================== Interface Block =================== #
    
    def A(self,):
        e = FrameEvent("A",None)
        self.__kernel(e)
    
    def B(self,):
        e = FrameEvent("B",None)
        self.__kernel(e)
    
    def C(self,):
        e = FrameEvent("C",None)
        self.__kernel(e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $I
    
    def __hierarchical_state_I(self, e):
        if e._message == ">":
            compartment = HierarchicalCompartment(self.__hierarchical_state_S)
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $S
    
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
    
    # ----------------------------------------
    # $S0
    
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
        
    
    # ----------------------------------------
    # $S1
    
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
        
    
    # ----------------------------------------
    # $S2
    
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
    
    
    # ----------------------------------------
    # $S3
    
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
        
    
    # ----------------------------------------
    # $T
    
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
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, e):
        
        # send event to current state
        self.__router(e)
        
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
                
    
    def __router(self, e):
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
        
    def __transition(self, compartment: 'HierarchicalCompartment'):
        self.__next_compartment = compartment
    
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
        self.forward_event = None
    
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

