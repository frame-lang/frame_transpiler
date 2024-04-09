#Emitted from framec_v0.11.2



from framelang.framelang import FrameEvent



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class Naming:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = NamingCompartment('__naming_state_Init', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        
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
        self.return_stack.append(None)
        __e = FrameEvent("snake_event",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def CamelEvent(self,CamelParam: int):
        parameters = {}
        parameters["CamelParam"] = CamelParam
        self.return_stack.append(None)
        __e = FrameEvent("CamelEvent",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def event123(self,param123: int):
        parameters = {}
        parameters["param123"] = param123
        self.return_stack.append(None)
        __e = FrameEvent("event123",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def call(self,event: str,param: int):
        parameters = {}
        parameters["event"] = event
        parameters["param"] = param
        self.return_stack.append(None)
        __e = FrameEvent("call",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $Init
    
    def __naming_state_Init(self, __e, compartment):
        if __e._message == "snake_event":
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_snake_state', next_compartment)
            next_compartment.state_args["snake_state_param"] = __e._parameters["snake_param"]
            next_compartment.state_vars["snake_state_var"] = self.snake_domain_var + self.CamelDomainVar + self.domainVar123 + 100
            self.__transition(next_compartment)
            return
        elif __e._message == "CamelEvent":
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_CamelState', next_compartment)
            next_compartment.state_args["CamelStateParam"] = __e._parameters["CamelParam"]
            next_compartment.state_vars["CamelStateVar"] = self.snake_domain_var + self.CamelDomainVar + self.domainVar123 + 200
            self.__transition(next_compartment)
            return
        elif __e._message == "event123":
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_state123', next_compartment)
            next_compartment.state_args["stateParam123"] = __e._parameters["param123"]
            next_compartment.state_vars["stateVar123"] = self.snake_domain_var + self.CamelDomainVar + self.domainVar123 + 300
            self.__transition(next_compartment)
            return
        elif __e._message == "call":
            if ((__e._parameters["event"] == "snake_event")):
                self.snake_event(__e._parameters["param"])
                return
            elif ((__e._parameters["event"] == "CamelEvent")):
                self.CamelEvent(__e._parameters["param"])
                return
            elif ((__e._parameters["event"] == "event123")):
                self.event123(__e._parameters["param"])
                return
            
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $snake_state
    
    def __naming_state_snake_state(self, __e, compartment):
          #  1100
        if __e._message == "snake_event":
            snake_local_var: int = compartment.state_vars["snake_state_var"] + compartment.state_args["snake_state_param"] + __e._parameters["snake_param"]
            self.snake_action_do(snake_local_var)
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Final', next_compartment)
            next_compartment.state_args["result"] = snake_local_var
            self.__transition(next_compartment)
            return
        elif __e._message == "CamelEvent":
            CamelLocalVar: int = compartment.state_vars["snake_state_var"] + compartment.state_args["snake_state_param"] + __e._parameters["CamelParam"]
            self.CamelAction_do(CamelLocalVar)
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Final', next_compartment)
            next_compartment.state_args["result"] = CamelLocalVar
            self.__transition(next_compartment)
            return
        elif __e._message == "event123":
            localVar123: int = compartment.state_vars["snake_state_var"] + compartment.state_args["snake_state_param"] + __e._parameters["param123"]
            self.action123_do(localVar123)
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Final', next_compartment)
            next_compartment.state_args["result"] = localVar123
            self.__transition(next_compartment)
            return
        elif __e._message == "call":
            if ((__e._parameters["event"] == "snake_event")):
                self.snake_event(__e._parameters["param"])
                return
            elif ((__e._parameters["event"] == "CamelEvent")):
                self.CamelEvent(__e._parameters["param"])
                return
            elif ((__e._parameters["event"] == "event123")):
                self.event123(__e._parameters["param"])
                return
            
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $CamelState
    
    def __naming_state_CamelState(self, __e, compartment):
          #  1200
        if __e._message == "snake_event":
            snake_local_var: int = compartment.state_vars["CamelStateVar"] + compartment.state_args["CamelStateParam"] + __e._parameters["snake_param"]
            self.snake_action_do(snake_local_var)
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Final', next_compartment)
            next_compartment.state_args["result"] = snake_local_var
            self.__transition(next_compartment)
            return
        elif __e._message == "CamelEvent":
            CamelLocalVar: int = compartment.state_vars["CamelStateVar"] + compartment.state_args["CamelStateParam"] + __e._parameters["CamelParam"]
            self.CamelAction_do(CamelLocalVar)
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Final', next_compartment)
            next_compartment.state_args["result"] = CamelLocalVar
            self.__transition(next_compartment)
            return
        elif __e._message == "event123":
            localVar123: int = compartment.state_vars["CamelStateVar"] + compartment.state_args["CamelStateParam"] + __e._parameters["param123"]
            self.action123_do(localVar123)
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Final', next_compartment)
            next_compartment.state_args["result"] = localVar123
            self.__transition(next_compartment)
            return
        elif __e._message == "call":
            if ((__e._parameters["event"] == "snake_event")):
                self.snake_event(__e._parameters["param"])
                return
            elif ((__e._parameters["event"] == "CamelEvent")):
                self.CamelEvent(__e._parameters["param"])
                return
            elif ((__e._parameters["event"] == "event123")):
                self.event123(__e._parameters["param"])
                return
            
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $state123
    
    def __naming_state_state123(self, __e, compartment):
          #  1300
        if __e._message == "snake_event":
            snake_local_var: int = compartment.state_vars["stateVar123"] + compartment.state_args["stateParam123"] + __e._parameters["snake_param"]
            self.snake_action_do(snake_local_var)
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Final', next_compartment)
            next_compartment.state_args["result"] = snake_local_var
            self.__transition(next_compartment)
            return
        elif __e._message == "CamelEvent":
            CamelLocalVar: int = compartment.state_vars["stateVar123"] + compartment.state_args["stateParam123"] + __e._parameters["CamelParam"]
            self.CamelAction_do(CamelLocalVar)
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Final', next_compartment)
            next_compartment.state_args["result"] = CamelLocalVar
            self.__transition(next_compartment)
            return
        elif __e._message == "event123":
            localVar123: int = compartment.state_vars["stateVar123"] + compartment.state_args["stateParam123"] + __e._parameters["param123"]
            self.action123_do(localVar123)
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Final', next_compartment)
            next_compartment.state_args["result"] = localVar123
            self.__transition(next_compartment)
            return
        elif __e._message == "call":
            if ((__e._parameters["event"] == "snake_event")):
                self.snake_event(__e._parameters["param"])
                return
            elif ((__e._parameters["event"] == "CamelEvent")):
                self.CamelEvent(__e._parameters["param"])
                return
            elif ((__e._parameters["event"] == "event123")):
                self.event123(__e._parameters["param"])
                return
            
            else:
                pass
            
            return
    
    # ----------------------------------------
    # $Final
    
    def __naming_state_Final(self, __e, compartment):
        if __e._message == ">":
            self.logFinal_do((compartment.state_args["result"]))
            next_compartment = None
            next_compartment = NamingCompartment('__naming_state_Init', next_compartment)
            self.__transition(next_compartment)
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
        if self.__compartment.state == '__naming_state_Init':
            self.__naming_state_Init(__e, self.__compartment)
        elif self.__compartment.state == '__naming_state_snake_state':
            self.__naming_state_snake_state(__e, self.__compartment)
        elif self.__compartment.state == '__naming_state_CamelState':
            self.__naming_state_CamelState(__e, self.__compartment)
        elif self.__compartment.state == '__naming_state_state123':
            self.__naming_state_state123(__e, self.__compartment)
        elif self.__compartment.state == '__naming_state_Final':
            self.__naming_state_Final(__e, self.__compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class NamingCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    