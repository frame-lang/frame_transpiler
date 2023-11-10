# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class Branch:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = '__branch_state_I'
        self.__compartment: 'BranchCompartment' = BranchCompartment(self.__state)
        self.__next_compartment: 'BranchCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def A(self,):
        e = FrameEvent("A",None)
        self.__kernel(e)
    
    def B(self,):
        e = FrameEvent("B",None)
        self.__kernel(e)
    
    def C(self,):
        e = FrameEvent("C",None)
        self.__kernel(e)
    
    def D(self,):
        e = FrameEvent("D",None)
        self.__kernel(e)
    
    def E(self,):
        e = FrameEvent("E",None)
        self.__kernel(e)
    
    def F(self,):
        e = FrameEvent("F",None)
        self.__kernel(e)
    
    def OnBool(self,b: bool):
        parameters = {}
        parameters["b"] = b

        e = FrameEvent("OnBool",parameters)
        self.__kernel(e)
    
    def OnInt(self,i: int):
        parameters = {}
        parameters["i"] = i

        e = FrameEvent("OnInt",parameters)
        self.__kernel(e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $I
    
    def __branch_state_I(self, e):
        if e._message == "A":
            compartment = BranchCompartment('__branch_state_SimpleIf')
            self.__transition(compartment)
            return
        elif e._message == "B":
            compartment = BranchCompartment('__branch_state_NegatedIf')
            self.__transition(compartment)
            return
        elif e._message == "C":
            compartment = BranchCompartment('__branch_state_Precedence')
            self.__transition(compartment)
            return
        elif e._message == "D":
            compartment = BranchCompartment('__branch_state_NestedIf')
            self.__transition(compartment)
            return
        elif e._message == "E":
            compartment = BranchCompartment('__branch_state_GuardedTransition')
            self.__transition(compartment)
            return
        elif e._message == "F":
            compartment = BranchCompartment('__branch_state_NestedGuardedTransition')
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $SimpleIf
    
    def __branch_state_SimpleIf(self, e):
        if e._message == "OnBool":
            if  e._parameters["b"]:
                self.log_do("then 1")
            else:
                pass
            
            if  e._parameters["b"]:
                pass
            else:
                self.log_do("else 1")
            
            if  e._parameters["b"]:
                self.log_do("then 2")
            else:
                self.log_do("else 2")
            
            if  e._parameters["b"]:
                compartment = BranchCompartment('__branch_state_F1')
                self.__transition(compartment)
                return
            else:
                compartment = BranchCompartment('__branch_state_F2')
                self.__transition(compartment)
                return
            
            return
        elif e._message == "OnInt":
            if  e._parameters["i"] > 5:
                self.log_do("> 5")
            else:
                self.log_do("<= 5")
            
            if  e._parameters["i"] < 10:
                self.log_do("< 10")
            else:
                self.log_do(">= 10")
            
            if  e._parameters["i"] == 7:
                self.log_do("== 7")
                compartment = BranchCompartment('__branch_state_F1')
                self.__transition(compartment)
                return
            else:
                self.log_do("!= 7")
                compartment = BranchCompartment('__branch_state_F2')
                self.__transition(compartment)
                return
            
            return
    
    # ----------------------------------------
    # $NegatedIf
    
    def __branch_state_NegatedIf(self, e):
        if e._message == "OnBool":
            if  not (e._parameters["b"]):
                self.log_do("then 1")
            else:
                pass
            
            if  not (e._parameters["b"]):
                pass
            else:
                self.log_do("else 1")
            
            if  not (e._parameters["b"]):
                self.log_do("then 2")
            else:
                self.log_do("else 2")
            
            if  not (e._parameters["b"]):
                compartment = BranchCompartment('__branch_state_F1')
                self.__transition(compartment)
                return
            else:
                compartment = BranchCompartment('__branch_state_F2')
                self.__transition(compartment)
                return
            
            return
        elif e._message == "OnInt":
            if  not (e._parameters["i"] >= 5):
                self.log_do("< 5")
            else:
                self.log_do(">= 5")
            
            if  not (e._parameters["i"] <= 10):
                self.log_do("> 10")
            else:
                self.log_do("<= 10")
            
            if  not (e._parameters["i"] != 7):
                self.log_do("== 7")
                compartment = BranchCompartment('__branch_state_F1')
                self.__transition(compartment)
                return
            else:
                self.log_do("!= 7")
                compartment = BranchCompartment('__branch_state_F2')
                self.__transition(compartment)
                return
            
            return
    
    # ----------------------------------------
    # $Precedence
    
    def __branch_state_Precedence(self, e):
        if e._message == "OnInt":
            if  -e._parameters["i"] >= 0 and -e._parameters["i"] <= 5:
                self.log_do("then 1")
            else:
                self.log_do("else 1")
            
            if   not (e._parameters["i"] >= -5 and e._parameters["i"] <= 5) and (e._parameters["i"] >= -10 and e._parameters["i"] <= 10):
                self.log_do("then 2")
            else:
                self.log_do("else 2")
            
            if  e._parameters["i"] >= 0 and e._parameters["i"] <= 5 or e._parameters["i"] >= 10 and e._parameters["i"] <= 20:
                self.log_do("then 3")
            else:
                self.log_do("else 3")
            
            if  not ((e._parameters["i"] < 0 or e._parameters["i"] > 10) and e._parameters["i"] + 5 < 20):
                self.log_do("then 4")
            else:
                self.log_do("else 4")
            
            return
    
    # ----------------------------------------
    # $NestedIf
    
    def __branch_state_NestedIf(self, e):
        if e._message == "OnInt":
            if  e._parameters["i"] > 0:
                self.log_do("> 0")
                if  e._parameters["i"] < 100:
                    self.log_do("< 100")
                    compartment = BranchCompartment('__branch_state_F1')
                    self.__transition(compartment)
                    return
                else:
                    self.log_do(">= 100")
                
            else:
                self.log_do("<= 0")
                if  e._parameters["i"] > -10:
                    self.log_do("> -10")
                else:
                    self.log_do("<= -10")
                    compartment = BranchCompartment('__branch_state_F2')
                    self.__transition(compartment)
                    return
                
            
            return
    
    # ----------------------------------------
    # $GuardedTransition
    
    def __branch_state_GuardedTransition(self, e):
        if e._message == "OnInt":
            if  e._parameters["i"] > 100:
                self.log_do("-> $F1")
                compartment = BranchCompartment('__branch_state_F1')
                self.__transition(compartment)
                return
            else:
                pass
            
            if  not (e._parameters["i"] > 10):
                pass
            else:
                self.log_do("-> $F2")
                compartment = BranchCompartment('__branch_state_F2')
                self.__transition(compartment)
                return
            
            self.log_do("-> $F3")
            compartment = BranchCompartment('__branch_state_F3')
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $NestedGuardedTransition
    
    def __branch_state_NestedGuardedTransition(self, e):
        if e._message == "OnInt":
            if  e._parameters["i"] > 10:
                if  e._parameters["i"] > 100:
                    self.log_do("-> $F1")
                    compartment = BranchCompartment('__branch_state_F1')
                    self.__transition(compartment)
                    return
                else:
                    pass
                
                if  e._parameters["i"] > 50:
                    pass
                else:
                    self.log_do("-> $F2")
                    compartment = BranchCompartment('__branch_state_F2')
                    self.__transition(compartment)
                    return
                
            else:
                pass
            
            self.log_do("-> $F3")
            compartment = BranchCompartment('__branch_state_F3')
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $F1
    
    def __branch_state_F1(self, e):
        pass
        
    
    # ----------------------------------------
    # $F2
    
    def __branch_state_F2(self, e):
        pass
        
    
    # ----------------------------------------
    # $F3
    
    def __branch_state_F3(self, e):
        pass
        
    
    # ===================== Actions Block =================== #
    
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
        if self.__compartment.state == '__branch_state_I':
            self.__branch_state_I(e)
        elif self.__compartment.state == '__branch_state_SimpleIf':
            self.__branch_state_SimpleIf(e)
        elif self.__compartment.state == '__branch_state_NegatedIf':
            self.__branch_state_NegatedIf(e)
        elif self.__compartment.state == '__branch_state_Precedence':
            self.__branch_state_Precedence(e)
        elif self.__compartment.state == '__branch_state_NestedIf':
            self.__branch_state_NestedIf(e)
        elif self.__compartment.state == '__branch_state_GuardedTransition':
            self.__branch_state_GuardedTransition(e)
        elif self.__compartment.state == '__branch_state_NestedGuardedTransition':
            self.__branch_state_NestedGuardedTransition(e)
        elif self.__compartment.state == '__branch_state_F1':
            self.__branch_state_F1(e)
        elif self.__compartment.state == '__branch_state_F2':
            self.__branch_state_F2(e)
        elif self.__compartment.state == '__branch_state_F3':
            self.__branch_state_F3(e)
        
    def __transition(self, compartment: 'BranchCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class BranchCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    
# ********************

#class BranchController(Branch):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,msg: str):
        #pass

# ********************

