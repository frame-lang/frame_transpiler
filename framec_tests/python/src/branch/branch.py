# emitted from framec_v0.11.0
# get include files at https://github.com/frame-lang/frame-ancillary-files
from framelang.framelang import FrameEvent

class Branch:
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        self.__state = self.__branch_state_I
        self.__compartment: 'BranchCompartment' = BranchCompartment(self.__state)
        self.__next_compartment: 'BranchCompartment' = None
        
        # Initialize domain
        
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
    
    def D(self,):
        e = FrameEvent("D",None)
        self.__mux(e)
    
    def E(self,):
        e = FrameEvent("E",None)
        self.__mux(e)
    
    def F(self,):
        e = FrameEvent("F",None)
        self.__mux(e)
    
    def OnBool(self,b: bool):
        parameters = {}
        parameters["b"] = b

        e = FrameEvent("OnBool",parameters)
        self.__mux(e)
    
    def OnInt(self,i: int):
        parameters = {}
        parameters["i"] = i

        e = FrameEvent("OnInt",parameters)
        self.__mux(e)
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__branch_state_I':
            self.__branch_state_I(e)
        elif self.__compartment.state.__name__ == '__branch_state_SimpleIf':
            self.__branch_state_SimpleIf(e)
        elif self.__compartment.state.__name__ == '__branch_state_NegatedIf':
            self.__branch_state_NegatedIf(e)
        elif self.__compartment.state.__name__ == '__branch_state_Precedence':
            self.__branch_state_Precedence(e)
        elif self.__compartment.state.__name__ == '__branch_state_NestedIf':
            self.__branch_state_NestedIf(e)
        elif self.__compartment.state.__name__ == '__branch_state_GuardedTransition':
            self.__branch_state_GuardedTransition(e)
        elif self.__compartment.state.__name__ == '__branch_state_NestedGuardedTransition':
            self.__branch_state_NestedGuardedTransition(e)
        elif self.__compartment.state.__name__ == '__branch_state_F1':
            self.__branch_state_F1(e)
        elif self.__compartment.state.__name__ == '__branch_state_F2':
            self.__branch_state_F2(e)
        elif self.__compartment.state.__name__ == '__branch_state_F3':
            self.__branch_state_F3(e)
        
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
    
    def __branch_state_I(self, e):
        if e._message == "A":
            compartment = BranchCompartment(self.__branch_state_SimpleIf)
            self.__transition(compartment)
            
            return
        
        elif e._message == "B":
            compartment = BranchCompartment(self.__branch_state_NegatedIf)
            self.__transition(compartment)
            
            return
        
        elif e._message == "C":
            compartment = BranchCompartment(self.__branch_state_Precedence)
            self.__transition(compartment)
            
            return
        
        elif e._message == "D":
            compartment = BranchCompartment(self.__branch_state_NestedIf)
            self.__transition(compartment)
            
            return
        
        elif e._message == "E":
            compartment = BranchCompartment(self.__branch_state_GuardedTransition)
            self.__transition(compartment)
            
            return
        
        elif e._message == "F":
            compartment = BranchCompartment(self.__branch_state_NestedGuardedTransition)
            self.__transition(compartment)
            
            return
        
    def __branch_state_SimpleIf(self, e):
        if e._message == "OnBool":
            ___b___ = e._parameters["b"]
            if ___b___:
                self.log_do("then 1")
            else:
                pass
            
            ___b___ = e._parameters["b"]
            if ___b___:
                pass
            else:
                self.log_do("else 1")
            
            ___b___ = e._parameters["b"]
            if ___b___:
                self.log_do("then 2")
            else:
                self.log_do("else 2")
            
            ___b___ = e._parameters["b"]
            if ___b___:
                compartment = BranchCompartment(self.__branch_state_F1)
                self.__transition(compartment)
                return
            else:
                compartment = BranchCompartment(self.__branch_state_F2)
                self.__transition(compartment)
                return
            
            
            return
        
        elif e._message == "OnInt":
            ___b___ = e._parameters["i"] > 5
            if ___b___:
                self.log_do("> 5")
            else:
                self.log_do("<= 5")
            
            ___b___ = e._parameters["i"] < 10
            if ___b___:
                self.log_do("< 10")
            else:
                self.log_do(">= 10")
            
            ___b___ = e._parameters["i"] == 7
            if ___b___:
                self.log_do("== 7")
                compartment = BranchCompartment(self.__branch_state_F1)
                self.__transition(compartment)
                return
            else:
                self.log_do("!= 7")
                compartment = BranchCompartment(self.__branch_state_F2)
                self.__transition(compartment)
                return
            
            
            return
        
    def __branch_state_NegatedIf(self, e):
        if e._message == "OnBool":
            ___b___ = not (e._parameters["b"])
            if ___b___:
                self.log_do("then 1")
            else:
                pass
            
            ___b___ = not (e._parameters["b"])
            if ___b___:
                pass
            else:
                self.log_do("else 1")
            
            ___b___ = not (e._parameters["b"])
            if ___b___:
                self.log_do("then 2")
            else:
                self.log_do("else 2")
            
            ___b___ = not (e._parameters["b"])
            if ___b___:
                compartment = BranchCompartment(self.__branch_state_F1)
                self.__transition(compartment)
                return
            else:
                compartment = BranchCompartment(self.__branch_state_F2)
                self.__transition(compartment)
                return
            
            
            return
        
        elif e._message == "OnInt":
            ___b___ = not (e._parameters["i"] >= 5)
            if ___b___:
                self.log_do("< 5")
            else:
                self.log_do(">= 5")
            
            ___b___ = not (e._parameters["i"] <= 10)
            if ___b___:
                self.log_do("> 10")
            else:
                self.log_do("<= 10")
            
            ___b___ = not (e._parameters["i"] != 7)
            if ___b___:
                self.log_do("== 7")
                compartment = BranchCompartment(self.__branch_state_F1)
                self.__transition(compartment)
                return
            else:
                self.log_do("!= 7")
                compartment = BranchCompartment(self.__branch_state_F2)
                self.__transition(compartment)
                return
            
            
            return
        
    def __branch_state_Precedence(self, e):
        if e._message == "OnInt":
            ___b___ = -e._parameters["i"] >= 0 and -e._parameters["i"] <= 5
            if ___b___:
                self.log_do("then 1")
            else:
                self.log_do("else 1")
            
            ___b___ =  not (e._parameters["i"] >= -5 and e._parameters["i"] <= 5) and (e._parameters["i"] >= -10 and e._parameters["i"] <= 10)
            if ___b___:
                self.log_do("then 2")
            else:
                self.log_do("else 2")
            
            ___b___ = e._parameters["i"] >= 0 and e._parameters["i"] <= 5 or e._parameters["i"] >= 10 and e._parameters["i"] <= 20
            if ___b___:
                self.log_do("then 3")
            else:
                self.log_do("else 3")
            
            ___b___ = not ((e._parameters["i"] < 0 or e._parameters["i"] > 10) and e._parameters["i"] + 5 < 20)
            if ___b___:
                self.log_do("then 4")
            else:
                self.log_do("else 4")
            
            
            return
        
    def __branch_state_NestedIf(self, e):
        if e._message == "OnInt":
            ___b___ = e._parameters["i"] > 0
            if ___b___:
                self.log_do("> 0")
                ___b___ = e._parameters["i"] < 100
                if ___b___:
                    self.log_do("< 100")
                    compartment = BranchCompartment(self.__branch_state_F1)
                    self.__transition(compartment)
                    return
                else:
                    self.log_do(">= 100")
                
            else:
                self.log_do("<= 0")
                ___b___ = e._parameters["i"] > -10
                if ___b___:
                    self.log_do("> -10")
                else:
                    self.log_do("<= -10")
                    compartment = BranchCompartment(self.__branch_state_F2)
                    self.__transition(compartment)
                    return
                
            
            
            return
        
    def __branch_state_GuardedTransition(self, e):
        if e._message == "OnInt":
            ___b___ = e._parameters["i"] > 100
            if ___b___:
                self.log_do("-> $F1")
                compartment = BranchCompartment(self.__branch_state_F1)
                self.__transition(compartment)
                return
            else:
                pass
            
            ___b___ = not (e._parameters["i"] > 10)
            if ___b___:
                pass
            else:
                self.log_do("-> $F2")
                compartment = BranchCompartment(self.__branch_state_F2)
                self.__transition(compartment)
                return
            
            self.log_do("-> $F3")
            compartment = BranchCompartment(self.__branch_state_F3)
            self.__transition(compartment)
            
            return
        
    def __branch_state_NestedGuardedTransition(self, e):
        if e._message == "OnInt":
            ___b___ = e._parameters["i"] > 10
            if ___b___:
                ___b___ = e._parameters["i"] > 100
                if ___b___:
                    self.log_do("-> $F1")
                    compartment = BranchCompartment(self.__branch_state_F1)
                    self.__transition(compartment)
                    return
                else:
                    pass
                
                ___b___ = e._parameters["i"] > 50
                if ___b___:
                    pass
                else:
                    self.log_do("-> $F2")
                    compartment = BranchCompartment(self.__branch_state_F2)
                    self.__transition(compartment)
                    return
                
            else:
                pass
            
            self.log_do("-> $F3")
            compartment = BranchCompartment(self.__branch_state_F3)
            self.__transition(compartment)
            
            return
        
    def __branch_state_F1(self, e):
        pass
        
    def __branch_state_F2(self, e):
        pass
        
    def __branch_state_F3(self, e):
        pass
        
    
    # ===================== Actions Block =================== #
    
    
    
    def log_do(self,msg: str):
        raise NotImplementedError
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'BranchCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'BranchCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class BranchCompartment:

    def __init__(self, state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class BranchController(Branch):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,msg: str):
        #pass

# ********************

