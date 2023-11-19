# emitted from framec_v0.11.0

import sys
from enum import Enum

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class MatchTests_Fruit(Enum):
    PEACH = 0
    pear = 2
    Banana = 3
    Watermelon = 4
    Lemon = 5


class MatchTests:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = '__matchtests_state_A'
        self.__compartment: 'MatchTestsCompartment' = MatchTestsCompartment(self.__state)
        self.__next_compartment: 'MatchTestsCompartment' = None
        
        # Initialize domain
        
        self.tape  = []
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__kernel(frame_event)
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $A
    
    def __matchtests_state_A(self, e):
        if e._message == ">":
            self.matchFruit_do(MatchTests_Fruit.PEACH)
            self.matchFruit_do(MatchTests_Fruit.pear)
            self.matchFruit_do(MatchTests_Fruit.Banana)
            self.matchFruit_do(MatchTests_Fruit.Lemon)
            self.matchString_do("!@#$%^&*()")
            self.matchString_do("a")
            self.matchString_do("")
            self.matchString_do(None)
            self.matchString_do("b")
            self.matchString_do("c")
            self.matchNumber_do(1001.5)
            self.matchNumber_do(0.12)
            self.matchNumber_do(0.5)
            self.matchNumber_do(0.111)
            self.matchNumber_do(1001)
            print(self.tape)
            return
    
    # ===================== Actions Block =================== #
    
    def matchFruit_do(self,x: MatchTests_Fruit):
        if (x == MatchTests_Fruit.Banana) or (x == MatchTests_Fruit.Watermelon):
            self.log_do("Matched Banana or Watermelon")
        elif (x == MatchTests_Fruit.PEACH):
            self.log_do("Matched PEACH")
        elif (x == MatchTests_Fruit.pear):
            self.log_do("Matched pear")
        elif (x == MatchTests_Fruit.Banana):
            self.log_do("Matched Banana")
        else:
            self.log_do("no enum match")
        
    
    def matchString_do(self,s):
        if ((s == "%") or (s == "^") or (s == "!@#$%^&*()")):
            self.log_do("matched " + s)
        elif ((s == "a") or (s == "b")):
            self.log_do("matched a|b")
        elif (isinstance(s, str) and len(s) == 0):
            self.log_do("matched empty string")
        elif (s is None):
            self.log_do("matched null")
        else:
            self.log_do("no string match")
        
        return
        
    
    def matchNumber_do(self,n):
        if (n == 1001.5) or (n == 0.12):
            self.log_do("Matched 1001.5 or 0.12")
        elif (n == 0.5):
            self.log_do("Matched .5")
        elif (n == 0.111):
            self.log_do("Matched .111")
        else:
            self.log_do("no number match")
        
        return
        
    
    def syntaxTests_do(self):
        if ((x == "a")):
            pass
        else:
            pass
        
        if ((x == "a")):
            pass
        else:
            pass
        
        if ((x == "a")):
            pass
        
        if ((x == "a")):
            foo()
        
        if ((x == "a")):
            foo()
        else:
            bar()
        
        if ((x == "a")):
            foo()
        else:
            bar()
        
        if ((x == "a")):
            pass
        else:
            pass
        
        if ((x == "a")):
                foo()
        else:
                bar()
        
        if ((x == "a")):
                foo()
        else:
                bar()
        
        return
        
    
    def log_do(self,msg):
        self.tape.append(msg)
    
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
        if self.__compartment.state == '__matchtests_state_A':
            self.__matchtests_state_A(e)
        
    def __transition(self, compartment: 'MatchTestsCompartment'):
        self.__next_compartment = compartment
    

# ===================== Compartment =================== #

class MatchTestsCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    