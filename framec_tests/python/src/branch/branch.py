#Emitted from framec_v0.11.0



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
        
        self.__compartment: 'BranchCompartment' = BranchCompartment('__branch_state_I')
        self.__next_compartment: 'BranchCompartment' = None
        
        # Initialize domain
        
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
    
    def D(self,):
        __e = FrameEvent("D",None)
        self.__kernel(__e)
    
    def E(self,):
        __e = FrameEvent("E",None)
        self.__kernel(__e)
    
    def F(self,):
        __e = FrameEvent("F",None)
        self.__kernel(__e)
    
    def OnBool(self,b: bool):
        parameters = {}
        parameters["b"] = b
        __e = FrameEvent("OnBool",parameters)
        self.__kernel(__e)
    
    def OnInt(self,i: int):
        parameters = {}
        parameters["i"] = i
        __e = FrameEvent("OnInt",parameters)
        self.__kernel(__e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $I
    
    def __branch_state_I(self, __e):
        if __e._message == "A":
            next_compartment = BranchCompartment('__branch_state_SimpleIf')
            self.__transition(next_compartment)
            return
        elif __e._message == "B":
            next_compartment = BranchCompartment('__branch_state_NegatedIf')
            self.__transition(next_compartment)
            return
        elif __e._message == "C":
            next_compartment = BranchCompartment('__branch_state_Precedence')
            self.__transition(next_compartment)
            return
        elif __e._message == "D":
            next_compartment = BranchCompartment('__branch_state_NestedIf')
            self.__transition(next_compartment)
            return
        elif __e._message == "E":
            next_compartment = BranchCompartment('__branch_state_GuardedTransition')
            self.__transition(next_compartment)
            return
        elif __e._message == "F":
            next_compartment = BranchCompartment('__branch_state_NestedGuardedTransition')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $SimpleIf
    
    def __branch_state_SimpleIf(self, __e):
        if __e._message == "OnBool":
            if  __e._parameters["b"]:
                self.log_do("then 1")
            else:
                pass
            
            if  __e._parameters["b"]:
                pass
            else:
                self.log_do("else 1")
            
            if  __e._parameters["b"]:
                self.log_do("then 2")
            else:
                self.log_do("else 2")
            
            if  __e._parameters["b"]:
                next_compartment = BranchCompartment('__branch_state_F1')
                self.__transition(next_compartment)
                return
            else:
                next_compartment = BranchCompartment('__branch_state_F2')
                self.__transition(next_compartment)
                return
            
            return
        elif __e._message == "OnInt":
            if  __e._parameters["i"] > 5:
                self.log_do("> 5")
            else:
                self.log_do("<= 5")
            
            if  __e._parameters["i"] < 10:
                self.log_do("< 10")
            else:
                self.log_do(">= 10")
            
            if  __e._parameters["i"] == 7:
                self.log_do("== 7")
                next_compartment = BranchCompartment('__branch_state_F1')
                self.__transition(next_compartment)
                return
            else:
                self.log_do("!= 7")
                next_compartment = BranchCompartment('__branch_state_F2')
                self.__transition(next_compartment)
                return
            
            return
    
    # ----------------------------------------
    # $NegatedIf
    
    def __branch_state_NegatedIf(self, __e):
        if __e._message == "OnBool":
            if  not (__e._parameters["b"]):
                self.log_do("then 1")
            else:
                pass
            
            if  not (__e._parameters["b"]):
                pass
            else:
                self.log_do("else 1")
            
            if  not (__e._parameters["b"]):
                self.log_do("then 2")
            else:
                self.log_do("else 2")
            
            if  not (__e._parameters["b"]):
                next_compartment = BranchCompartment('__branch_state_F1')
                self.__transition(next_compartment)
                return
            else:
                next_compartment = BranchCompartment('__branch_state_F2')
                self.__transition(next_compartment)
                return
            
            return
        elif __e._message == "OnInt":
            if  not (__e._parameters["i"] >= 5):
                self.log_do("< 5")
            else:
                self.log_do(">= 5")
            
            if  not (__e._parameters["i"] <= 10):
                self.log_do("> 10")
            else:
                self.log_do("<= 10")
            
            if  not (__e._parameters["i"] != 7):
                self.log_do("== 7")
                next_compartment = BranchCompartment('__branch_state_F1')
                self.__transition(next_compartment)
                return
            else:
                self.log_do("!= 7")
                next_compartment = BranchCompartment('__branch_state_F2')
                self.__transition(next_compartment)
                return
            
            return
    
    # ----------------------------------------
    # $Precedence
    
    def __branch_state_Precedence(self, __e):
        if __e._message == "OnInt":
            if  -__e._parameters["i"] >= 0 and -__e._parameters["i"] <= 5:
                self.log_do("then 1")
            else:
                self.log_do("else 1")
            
            if   not (__e._parameters["i"] >= -5 and __e._parameters["i"] <= 5) and (__e._parameters["i"] >= -10 and __e._parameters["i"] <= 10):
                self.log_do("then 2")
            else:
                self.log_do("else 2")
            
            if  __e._parameters["i"] >= 0 and __e._parameters["i"] <= 5 or __e._parameters["i"] >= 10 and __e._parameters["i"] <= 20:
                self.log_do("then 3")
            else:
                self.log_do("else 3")
            
            if  not ((__e._parameters["i"] < 0 or __e._parameters["i"] > 10) and __e._parameters["i"] + 5 < 20):
                self.log_do("then 4")
            else:
                self.log_do("else 4")
            
            return
    
    # ----------------------------------------
    # $NestedIf
    
    def __branch_state_NestedIf(self, __e):
        if __e._message == "OnInt":
            if  __e._parameters["i"] > 0:
                self.log_do("> 0")
                if  __e._parameters["i"] < 100:
                    self.log_do("< 100")
                    next_compartment = BranchCompartment('__branch_state_F1')
                    self.__transition(next_compartment)
                    return
                else:
                    self.log_do(">= 100")
                
            else:
                self.log_do("<= 0")
                if  __e._parameters["i"] > -10:
                    self.log_do("> -10")
                else:
                    self.log_do("<= -10")
                    next_compartment = BranchCompartment('__branch_state_F2')
                    self.__transition(next_compartment)
                    return
                
            
            return
    
    # ----------------------------------------
    # $GuardedTransition
    
    def __branch_state_GuardedTransition(self, __e):
        if __e._message == "OnInt":
            if  __e._parameters["i"] > 100:
                self.log_do("-> $F1")
                next_compartment = BranchCompartment('__branch_state_F1')
                self.__transition(next_compartment)
                return
            else:
                pass
            
            if  not (__e._parameters["i"] > 10):
                pass
            else:
                self.log_do("-> $F2")
                next_compartment = BranchCompartment('__branch_state_F2')
                self.__transition(next_compartment)
                return
            
            self.log_do("-> $F3")
            next_compartment = BranchCompartment('__branch_state_F3')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $NestedGuardedTransition
    
    def __branch_state_NestedGuardedTransition(self, __e):
        if __e._message == "OnInt":
            if  __e._parameters["i"] > 10:
                if  __e._parameters["i"] > 100:
                    self.log_do("-> $F1")
                    next_compartment = BranchCompartment('__branch_state_F1')
                    self.__transition(next_compartment)
                    return
                else:
                    pass
                
                if  __e._parameters["i"] > 50:
                    pass
                else:
                    self.log_do("-> $F2")
                    next_compartment = BranchCompartment('__branch_state_F2')
                    self.__transition(next_compartment)
                    return
                
            else:
                pass
            
            self.log_do("-> $F3")
            next_compartment = BranchCompartment('__branch_state_F3')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $F1
    
    def __branch_state_F1(self, __e):
        pass
        
    
    # ----------------------------------------
    # $F2
    
    def __branch_state_F2(self, __e):
        pass
        
    
    # ----------------------------------------
    # $F3
    
    def __branch_state_F3(self, __e):
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
        if self.__compartment.state == '__branch_state_I':
            self.__branch_state_I(__e)
        elif self.__compartment.state == '__branch_state_SimpleIf':
            self.__branch_state_SimpleIf(__e)
        elif self.__compartment.state == '__branch_state_NegatedIf':
            self.__branch_state_NegatedIf(__e)
        elif self.__compartment.state == '__branch_state_Precedence':
            self.__branch_state_Precedence(__e)
        elif self.__compartment.state == '__branch_state_NestedIf':
            self.__branch_state_NestedIf(__e)
        elif self.__compartment.state == '__branch_state_GuardedTransition':
            self.__branch_state_GuardedTransition(__e)
        elif self.__compartment.state == '__branch_state_NestedGuardedTransition':
            self.__branch_state_NestedGuardedTransition(__e)
        elif self.__compartment.state == '__branch_state_F1':
            self.__branch_state_F1(__e)
        elif self.__compartment.state == '__branch_state_F2':
            self.__branch_state_F2(__e)
        elif self.__compartment.state == '__branch_state_F3':
            self.__branch_state_F3(__e)
        
    def __transition(self, next_compartment: 'BranchCompartment'):
        self.__next_compartment = next_compartment
    
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
    