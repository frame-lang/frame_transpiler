#Emitted from framec_v0.11.0



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
        
        self.__compartment: 'HierarchicalCompartment' = HierarchicalCompartment('__hierarchical_state_I')
        self.__next_compartment: 'HierarchicalCompartment' = None
        self.__compartment: HierarchicalCompartment = HierarchicalCompartment(self.__state)
        self.__next_compartment: HierarchicalCompartment = None
        
        # Initialize domain
        
        self.enters  = []
        self.exits  = []
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def A(self,):
        __e = FrameEvent("A",None)
        self.__kernel(__e)
    
    def B(self,):
        __e = FrameEvent("B",None)
        self.__kernel(__e)
    
    def C(self,):
        __e = FrameEvent("C",None)
        self.__kernel(__e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $I
    
    def __hierarchical_state_I(self, __e):
        if __e._message == ">":
            next_compartment = HierarchicalCompartment('__hierarchical_state_S')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S
    
    def __hierarchical_state_S(self, __e):
        if __e._message == ">":
            self.enter_do("S")
            return
        elif __e._message == "<":
            self.exit_do("S")
            return
        elif __e._message == "A":
            self.log_do("S.A")
            next_compartment = HierarchicalCompartment('__hierarchical_state_S0')
            self.__transition(next_compartment)
            return
        elif __e._message == "B":
            self.log_do("S.B")
            next_compartment = HierarchicalCompartment('__hierarchical_state_S1')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $S0
    
    def __hierarchical_state_S0(self, __e):
        if __e._message == ">":
            self.enter_do("S0")
        elif __e._message == "<":
            self.exit_do("S0")
          #  override parent handler
        elif __e._message == "A":
            self.log_do("S0.A")
            next_compartment = HierarchicalCompartment('__hierarchical_state_T')
            self.__transition(next_compartment)
            return
          #  do this, then parent handler
        elif __e._message == "B":
            self.log_do("S0.B")
          #  extend parent handler
        elif __e._message == "C":
            self.log_do("S0.C")
            next_compartment = HierarchicalCompartment('__hierarchical_state_S2')
            self.__transition(next_compartment)
            return
        
        self.__hierarchical_state_S(__e)
        
    
    # ----------------------------------------
    # $S1
    
    def __hierarchical_state_S1(self, __e):
        if __e._message == ">":
            self.enter_do("S1")
            return
        elif __e._message == "<":
            self.exit_do("S1")
            return
          #  defer to parent for A
          #  do this, then parent, which transitions here
        elif __e._message == "B":
            self.log_do("S1.B")
          #  propagate message not handled by parent
        elif __e._message == "C":
            self.log_do("S1.C")
        
        self.__hierarchical_state_S(__e)
        
    
    # ----------------------------------------
    # $S2
    
    def __hierarchical_state_S2(self, __e):
        if __e._message == ">":
            self.enter_do("S2")
        elif __e._message == "<":
            self.exit_do("S2")
          #  will propagate to S0 and S
        elif __e._message == "B":
            self.log_do("S2.B")
        elif __e._message == "C":
            self.log_do("S2.C")
            next_compartment = HierarchicalCompartment('__hierarchical_state_T')
            self.__transition(next_compartment)
            return
        
        self.__hierarchical_state_S0(__e)
        
      #  continue after transition (should be ignored)
    
    
    # ----------------------------------------
    # $S3
    
    def __hierarchical_state_S3(self, __e):
        if __e._message == ">":
            self.enter_do("S3")
        elif __e._message == "<":
            self.exit_do("S3")
          #  defer to grandparent for A
          #  override and move to sibling
        elif __e._message == "B":
            self.log_do("S3.B")
            next_compartment = HierarchicalCompartment('__hierarchical_state_S2')
            self.__transition(next_compartment)
            return
        
        self.__hierarchical_state_S1(__e)
        
    
    # ----------------------------------------
    # $T
    
    def __hierarchical_state_T(self, __e):
        if __e._message == ">":
            self.enter_do("T")
            return
        elif __e._message == "<":
            self.exit_do("T")
            return
        elif __e._message == "A":
            self.log_do("T.A")
            next_compartment = HierarchicalCompartment('__hierarchical_state_S')
            self.__transition(next_compartment)
            return
        elif __e._message == "B":
            self.log_do("T.B")
            next_compartment = HierarchicalCompartment('__hierarchical_state_S2')
            self.__transition(next_compartment)
            return
        elif __e._message == "C":
            self.log_do("T.C")
            next_compartment = HierarchicalCompartment('__hierarchical_state_S3')
            self.__transition(next_compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def enter_do(self,msg: str):
        raise NotImplementedError
    
    def exit_do(self,msg: str):
        raise NotImplementedError
    
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
        if self.__compartment.state == '__hierarchical_state_I':
            self.__hierarchical_state_I(__e)
        elif self.__compartment.state == '__hierarchical_state_S':
            self.__hierarchical_state_S(__e)
        elif self.__compartment.state == '__hierarchical_state_S0':
            self.__hierarchical_state_S0(__e)
        elif self.__compartment.state == '__hierarchical_state_S1':
            self.__hierarchical_state_S1(__e)
        elif self.__compartment.state == '__hierarchical_state_S2':
            self.__hierarchical_state_S2(__e)
        elif self.__compartment.state == '__hierarchical_state_S3':
            self.__hierarchical_state_S3(__e)
        elif self.__compartment.state == '__hierarchical_state_T':
            self.__hierarchical_state_T(__e)
        
    def __transition(self, next_compartment: 'HierarchicalCompartment'):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class HierarchicalCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    