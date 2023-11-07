# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class Naming:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__naming_state_Init
        self.__compartment: 'NamingCompartment' = NamingCompartment(self.__state)
        self.__next_compartment: 'NamingCompartment' = None
        
        # Initialize domain
        
        self.snake_domain_var : int = 300
        self.CamelDomainVar : int = 550
        self.domainVar123 : int = 150
        self.snake_log  = []
        self.CamelLog  = []
        self.log123  = []
        self.finalLog  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def snake_event(self,snake_param: int):
        parameters = {}
        parameters["snake_param"] = snake_param

        e = FrameEvent("snake_event",parameters)
        self.__kernel(e)
    
    def CamelEvent(self,CamelParam: int):
        parameters = {}
        parameters["CamelParam"] = CamelParam

        e = FrameEvent("CamelEvent",parameters)
        self.__kernel(e)
    
    def event123(self,param123: int):
        parameters = {}
        parameters["param123"] = param123

        e = FrameEvent("event123",parameters)
        self.__kernel(e)
    
    def call(self,event: str,param: int):
        parameters = {}
        parameters["event"] = event

        parameters["param"] = param

        e = FrameEvent("call",parameters)
        self.__kernel(e)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __naming_state_Init(self, e):
        if e._message == "snake_event":
            compartment = NamingCompartment(self.__naming_state_snake_state)
            compartment.state_args["snake_state_param"] = e._parameters["snake_param"]
            compartment.state_vars["snake_state_var"] = self.snake_domain_var + self.CamelDomainVar + self.domainVar123 + 100
            self.__transition(compartment)
            return
        elif e._message == "CamelEvent":
            compartment = NamingCompartment(self.__naming_state_CamelState)
            compartment.state_args["CamelStateParam"] = e._parameters["CamelParam"]
            compartment.state_vars["CamelStateVar"] = self.snake_domain_var + self.CamelDomainVar + self.domainVar123 + 200
            self.__transition(compartment)
            return
        elif e._message == "event123":
            compartment = NamingCompartment(self.__naming_state_state123)
            compartment.state_args["stateParam123"] = e._parameters["param123"]
            compartment.state_vars["stateVar123"] = self.snake_domain_var + self.CamelDomainVar + self.domainVar123 + 300
            self.__transition(compartment)
            return
        elif e._message == "call":
            if ((e._parameters["event"] == "snake_event")):
                self.snake_event(e._parameters["param"])
                return
            elif ((e._parameters["event"] == "CamelEvent")):
                self.CamelEvent(e._parameters["param"])
                return
            elif ((e._parameters["event"] == "event123")):
                self.event123(e._parameters["param"])
                return
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $snake_state
    
    def __naming_state_snake_state(self, e):
          #  1100
        if e._message == "snake_event":
            snake_local_var : int = self.__compartment.state_vars["snake_state_var"] + self.__compartment.state_args["snake_state_param"] + e._parameters["snake_param"]
            self.snake_action_do(snake_local_var)
            compartment = NamingCompartment(self.__naming_state_Final)
            compartment.state_args["result"] = snake_local_var
            self.__transition(compartment)
            return
        elif e._message == "CamelEvent":
            CamelLocalVar : int = self.__compartment.state_vars["snake_state_var"] + self.__compartment.state_args["snake_state_param"] + e._parameters["CamelParam"]
            self.CamelAction_do(CamelLocalVar)
            compartment = NamingCompartment(self.__naming_state_Final)
            compartment.state_args["result"] = CamelLocalVar
            self.__transition(compartment)
            return
        elif e._message == "event123":
            localVar123 : int = self.__compartment.state_vars["snake_state_var"] + self.__compartment.state_args["snake_state_param"] + e._parameters["param123"]
            self.action123_do(localVar123)
            compartment = NamingCompartment(self.__naming_state_Final)
            compartment.state_args["result"] = localVar123
            self.__transition(compartment)
            return
        elif e._message == "call":
            if ((e._parameters["event"] == "snake_event")):
                self.snake_event(e._parameters["param"])
                return
            elif ((e._parameters["event"] == "CamelEvent")):
                self.CamelEvent(e._parameters["param"])
                return
            elif ((e._parameters["event"] == "event123")):
                self.event123(e._parameters["param"])
                return
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $CamelState
    
    def __naming_state_CamelState(self, e):
          #  1200
        if e._message == "snake_event":
            snake_local_var : int = self.__compartment.state_vars["CamelStateVar"] + self.__compartment.state_args["CamelStateParam"] + e._parameters["snake_param"]
            self.snake_action_do(snake_local_var)
            compartment = NamingCompartment(self.__naming_state_Final)
            compartment.state_args["result"] = snake_local_var
            self.__transition(compartment)
            return
        elif e._message == "CamelEvent":
            CamelLocalVar : int = self.__compartment.state_vars["CamelStateVar"] + self.__compartment.state_args["CamelStateParam"] + e._parameters["CamelParam"]
            self.CamelAction_do(CamelLocalVar)
            compartment = NamingCompartment(self.__naming_state_Final)
            compartment.state_args["result"] = CamelLocalVar
            self.__transition(compartment)
            return
        elif e._message == "event123":
            localVar123 : int = self.__compartment.state_vars["CamelStateVar"] + self.__compartment.state_args["CamelStateParam"] + e._parameters["param123"]
            self.action123_do(localVar123)
            compartment = NamingCompartment(self.__naming_state_Final)
            compartment.state_args["result"] = localVar123
            self.__transition(compartment)
            return
        elif e._message == "call":
            if ((e._parameters["event"] == "snake_event")):
                self.snake_event(e._parameters["param"])
                return
            elif ((e._parameters["event"] == "CamelEvent")):
                self.CamelEvent(e._parameters["param"])
                return
            elif ((e._parameters["event"] == "event123")):
                self.event123(e._parameters["param"])
                return
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $state123
    
    def __naming_state_state123(self, e):
          #  1300
        if e._message == "snake_event":
            snake_local_var : int = self.__compartment.state_vars["stateVar123"] + self.__compartment.state_args["stateParam123"] + e._parameters["snake_param"]
            self.snake_action_do(snake_local_var)
            compartment = NamingCompartment(self.__naming_state_Final)
            compartment.state_args["result"] = snake_local_var
            self.__transition(compartment)
            return
        elif e._message == "CamelEvent":
            CamelLocalVar : int = self.__compartment.state_vars["stateVar123"] + self.__compartment.state_args["stateParam123"] + e._parameters["CamelParam"]
            self.CamelAction_do(CamelLocalVar)
            compartment = NamingCompartment(self.__naming_state_Final)
            compartment.state_args["result"] = CamelLocalVar
            self.__transition(compartment)
            return
        elif e._message == "event123":
            localVar123 : int = self.__compartment.state_vars["stateVar123"] + self.__compartment.state_args["stateParam123"] + e._parameters["param123"]
            self.action123_do(localVar123)
            compartment = NamingCompartment(self.__naming_state_Final)
            compartment.state_args["result"] = localVar123
            self.__transition(compartment)
            return
        elif e._message == "call":
            if ((e._parameters["event"] == "snake_event")):
                self.snake_event(e._parameters["param"])
                return
            elif ((e._parameters["event"] == "CamelEvent")):
                self.CamelEvent(e._parameters["param"])
                return
            elif ((e._parameters["event"] == "event123")):
                self.event123(e._parameters["param"])
                return
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $Final
    
    def __naming_state_Final(self, e):
        if e._message == ">":
            self.logFinal_do((self.__compartment.state_args["result"]))
            compartment = NamingCompartment(self.__naming_state_Init)
            self.__transition(compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def snake_action_do(self,snake_param: int):
        raise NotImplementedError
    def CamelAction_do(self,CamelParam: int):
        raise NotImplementedError
    def action123_do(self,param123: int):
        raise NotImplementedError
    def logFinal_do(self,r: int):
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
        if self.__compartment.state.__name__ == '__naming_state_Init':
            self.__naming_state_Init(e)
        elif self.__compartment.state.__name__ == '__naming_state_snake_state':
            self.__naming_state_snake_state(e)
        elif self.__compartment.state.__name__ == '__naming_state_CamelState':
            self.__naming_state_CamelState(e)
        elif self.__compartment.state.__name__ == '__naming_state_state123':
            self.__naming_state_state123(e)
        elif self.__compartment.state.__name__ == '__naming_state_Final':
            self.__naming_state_Final(e)
        
    def __transition(self, compartment: 'NamingCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class NamingCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    
# ********************

#class NamingController(Naming):
	#def __init__(self,):
	    #super().__init__()

    #def snake_action_do(self,snake_param: int):
        #pass

    #def CamelAction_do(self,CamelParam: int):
        #pass

    #def action123_do(self,param123: int):
        #pass

    #def logFinal_do(self,r: int):
        #pass

# ********************

