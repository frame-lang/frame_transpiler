# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class Match:
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__match_state_Init
        self.__compartment: 'MatchCompartment' = MatchCompartment(self.__state)
        self.__next_compartment: 'MatchCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def Empty(self,):
        e = FrameEvent("Empty",None)
        self.__mux(e)
    
    def Simple(self,):
        e = FrameEvent("Simple",None)
        self.__mux(e)
    
    def Multi(self,):
        e = FrameEvent("Multi",None)
        self.__mux(e)
    
    def Nested(self,):
        e = FrameEvent("Nested",None)
        self.__mux(e)
    
    def Child(self,):
        e = FrameEvent("Child",None)
        self.__mux(e)
    
    def OnInt(self,i: int):
        parameters = {}
        parameters["i"] = i

        e = FrameEvent("OnInt",parameters)
        self.__mux(e)
    
    def Onstring(self,s: str):
        parameters = {}
        parameters["s"] = s

        e = FrameEvent("Onstring",parameters)
        self.__mux(e)
    
    # ===================== Machine Block =================== #
    
    def __match_state_Init(self, e):
        if e._message == "Empty":
            compartment = MatchCompartment(self.__match_state_EmptyMatch)
            self.__transition(compartment)
            return
        elif e._message == "Simple":
            compartment = MatchCompartment(self.__match_state_SimpleMatch)
            self.__transition(compartment)
            return
        elif e._message == "Multi":
            compartment = MatchCompartment(self.__match_state_MultiMatch)
            self.__transition(compartment)
            return
        elif e._message == "Nested":
            compartment = MatchCompartment(self.__match_state_NestedMatch)
            self.__transition(compartment)
            return
        elif e._message == "Child":
            compartment = MatchCompartment(self.__match_state_ChildMatch)
            self.__transition(compartment)
            return
    
    def __match_state_EmptyMatch(self, e):
        if e._message == "Onstring":
            if ((e._parameters["s"] == "") or (e._parameters["s"] == "foo")):
                self.log_do("empty")
            else:
                self.log_do("?")
            
            return
      #  TODO: matching only the empty string is broken
    
    
    def __match_state_SimpleMatch(self, e):
        if e._message == "OnInt":
            if (e._parameters["i"] == 0):
                self.log_do("0")
            elif (e._parameters["i"] == 42):
                self.log_do("42")
            elif (e._parameters["i"] == 42):
                self.log_do("!!!")
            elif (e._parameters["i"] == -200):
                self.log_do("-200")
            else:
                self.log_do("?")
            
            return
        elif e._message == "Onstring":
            if ((e._parameters["s"] == "hello")):
                self.log_do("hello")
            elif ((e._parameters["s"] == "hello")):
                self.log_do("!!!")
            elif ((e._parameters["s"] == "goodbye")):
                self.log_do("goodbye")
            elif ((e._parameters["s"] == "Testing 1, 2, 3...")):
                self.log_do("testing")
            elif ((e._parameters["s"] == "$10!")):
                self.log_do("money")
            else:
                self.log_do("?")
            
            return
    
    def __match_state_MultiMatch(self, e):
        if e._message == "OnInt":
            if (e._parameters["i"] == 3) or (e._parameters["i"] == -7):
                self.log_do("3|-7")
            elif (e._parameters["i"] == -4) or (e._parameters["i"] == 5) or (e._parameters["i"] == 6):
                self.log_do("-4|5|6")
            else:
                self.log_do("?")
            
            return
        elif e._message == "Onstring":
            if ((e._parameters["s"] == "$10") or (e._parameters["s"] == "12.5%") or (e._parameters["s"] == "@#*!")):
                self.log_do("symbols")
            elif ((e._parameters["s"] == " ") or (e._parameters["s"] == "  ") or (e._parameters["s"] == "\t") or (e._parameters["s"] == "\n")):
                self.log_do("whitespace")
            else:
                self.log_do("?")
            
            return
    
    def __match_state_NestedMatch(self, e):
        if e._message == "OnInt":
            if  e._parameters["i"] > 0:
                if (e._parameters["i"] == 1) or (e._parameters["i"] == 2) or (e._parameters["i"] == 3):
                    self.log_do("1-3")
                    if (e._parameters["i"] == 1):
                        self.log_do("1")
                    elif (e._parameters["i"] == 2):
                        self.log_do("2")
                    else:
                        self.log_do("3")
                    
                elif (e._parameters["i"] == 4) or (e._parameters["i"] == 5):
                    self.log_do("4-5")
                    if  e._parameters["i"] == 4:
                        self.log_do("4")
                    else:
                        self.log_do("5")
                    
                else:
                    self.log_do("too big")
                
            else:
                self.log_do("too small")
            
            return
        elif e._message == "Onstring":
            if ((e._parameters["s"] == "hello") or (e._parameters["s"] == "hola") or (e._parameters["s"] == "bonjour")):
                self.log_do("greeting")
                if ((e._parameters["s"] == "hello")):
                    self.log_do("English")
                elif ((e._parameters["s"] == "hola")):
                    self.log_do("Spanish")
                else:
                    self.log_do("French")
                
            elif ((e._parameters["s"] == "goodbye") or (e._parameters["s"] == "adios") or (e._parameters["s"] == "au revoir")):
                self.log_do("farewell")
                if ((e._parameters["s"] == "goodbye")):
                    self.log_do("English")
                elif ((e._parameters["s"] == "adios")):
                    self.log_do("Spanish")
                else:
                    self.log_do("French")
                
            else:
                self.log_do("?")
            
            return
    
    def __match_state_ChildMatch(self, e):
        if e._message == "OnInt":
            if (e._parameters["i"] == 0):
                compartment = MatchCompartment(self.__match_state_Final)
                self.__transition(compartment)
                return
            elif (e._parameters["i"] == 3):
                self.log_do("3")
            elif (e._parameters["i"] == 4):
                self.log_do("4")
                
                return
            elif (e._parameters["i"] == 42):
                self.log_do("42 in child")
            elif (e._parameters["i"] == 5):
                self.log_do("5")
                compartment = MatchCompartment(self.__match_state_Final)
                self.__transition(compartment)
                return
            else:
                self.log_do("no match in child")
            
        elif e._message == "Onstring":
            if ((e._parameters["s"] == "hello")):
                self.log_do("hello in child")
            elif ((e._parameters["s"] == "goodbye")):
                compartment = MatchCompartment(self.__match_state_Final)
                self.__transition(compartment)
                return
            elif ((e._parameters["s"] == "Testing 1, 2, 3...")):
                self.log_do("testing in child")
                
                return
            else:
                self.log_do("no match in child")
            
        self.__match_state_SimpleMatch(e)
        
    
    def __match_state_Final(self, e):
        pass
        
    
    # ===================== Actions Block =================== #
    
    def log_do(self,msg: str):
        raise NotImplementedError
    # =============== Machinery and Mechanisms ============== #
    
    def __mux(self, e):
        
        self.__router(e)
        
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            if(next_compartment.forward_event is not None and 
               next_compartment.forward_event._message == ">"):
                self.__router(FrameEvent( "<", self.__compartment.exit_args))
                self.__compartment = next_compartment
                self.__router(next_compartment.forward_event)
            else:
                self.__do_transition(next_compartment)
                if next_compartment.forward_event is not None:
                    self.__router(next_compartment.forward_event)
            next_compartment.forward_event = None
    
    def __router(self, e):
        if self.__compartment.state.__name__ == '__match_state_Init':
            self.__match_state_Init(e)
        elif self.__compartment.state.__name__ == '__match_state_EmptyMatch':
            self.__match_state_EmptyMatch(e)
        elif self.__compartment.state.__name__ == '__match_state_SimpleMatch':
            self.__match_state_SimpleMatch(e)
        elif self.__compartment.state.__name__ == '__match_state_MultiMatch':
            self.__match_state_MultiMatch(e)
        elif self.__compartment.state.__name__ == '__match_state_NestedMatch':
            self.__match_state_NestedMatch(e)
        elif self.__compartment.state.__name__ == '__match_state_ChildMatch':
            self.__match_state_ChildMatch(e)
        elif self.__compartment.state.__name__ == '__match_state_Final':
            self.__match_state_Final(e)
        
    def __transition(self, compartment: 'MatchCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'MatchCompartment'):
        self.__router(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__router(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class MatchCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    


# ********************

#class MatchController(Match):
	#def __init__(self,):
	    #super().__init__()

    #def log_do(self,msg: str):
        #pass

# ********************

