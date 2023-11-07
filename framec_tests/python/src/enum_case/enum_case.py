# emitted from framec_v0.11.0



from framelang.framelang import FrameEvent
from enum import Enum



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters
        self._return = None


class EnumTest_Days(Enum):
    SUNDAY = 0
    MONDAY = 1
    TUESDAY = 2
    WEDNESDAY = 3
    THURSDAY = 4
    FRIDAY = 5


class EnumTest:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        
        self.__state = self.__enumtest_state_SUN
        self.__compartment: 'EnumTestCompartment' = EnumTestCompartment(self.__state)
        self.__next_compartment: 'EnumTestCompartment' = None
        
        # Initialize domain
        
        self.days  = []
        
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
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $SUN
    
    def __enumtest_state_SUN(self, e):
        if e._message == "A":
            self.entered_do("SUNDAY",EnumTest_Days.SUNDAY)
            return
        elif e._message == "B":
            self.left_do("MONDAY",EnumTest_Days.MONDAY)
            compartment = EnumTestCompartment(self.__enumtest_state_MON)
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $MON
    
    def __enumtest_state_MON(self, e):
        if e._message == "A":
            self.entered_do("MONDAY",EnumTest_Days.MONDAY)
            return
        elif e._message == "B":
            self.left_do("TUESDAY",EnumTest_Days.TUESDAY)
            compartment = EnumTestCompartment(self.__enumtest_state_TUE)
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $TUE
    
    def __enumtest_state_TUE(self, e):
        if e._message == "A":
            self.entered_do("TUESDAY",EnumTest_Days.TUESDAY)
            return
        elif e._message == "B":
            self.left_do("WEDNESDAY",EnumTest_Days.WEDNESDAY)
            compartment = EnumTestCompartment(self.__enumtest_state_WED)
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $WED
    
    def __enumtest_state_WED(self, e):
        if e._message == "A":
            self.entered_do("WEDNESDAY",EnumTest_Days.WEDNESDAY)
            return
        elif e._message == "B":
            self.left_do("THURSDAY",EnumTest_Days.THURSDAY)
            compartment = EnumTestCompartment(self.__enumtest_state_THR)
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $THR
    
    def __enumtest_state_THR(self, e):
        if e._message == "A":
            self.entered_do("THURSDAY",EnumTest_Days.THURSDAY)
            return
        elif e._message == "B":
            self.left_do("FRIDAY",EnumTest_Days.FRIDAY)
            compartment = EnumTestCompartment(self.__enumtest_state_FRI)
            self.__transition(compartment)
            return
    
    # ----------------------------------------
    # $FRI
    
    def __enumtest_state_FRI(self, e):
        if e._message == "A":
            self.entered_do("FRIDAY",EnumTest_Days.FRIDAY)
            return
        elif e._message == "B":
            self.left_do("SUNDAY",EnumTest_Days.SUNDAY)
            compartment = EnumTestCompartment(self.__enumtest_state_SUN)
            self.__transition(compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def entered_do(self,msg: str,val: int):
        raise NotImplementedError
    def left_do(self,msg: str,val: int):
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
        if self.__compartment.state.__name__ == '__enumtest_state_SUN':
            self.__enumtest_state_SUN(e)
        elif self.__compartment.state.__name__ == '__enumtest_state_MON':
            self.__enumtest_state_MON(e)
        elif self.__compartment.state.__name__ == '__enumtest_state_TUE':
            self.__enumtest_state_TUE(e)
        elif self.__compartment.state.__name__ == '__enumtest_state_WED':
            self.__enumtest_state_WED(e)
        elif self.__compartment.state.__name__ == '__enumtest_state_THR':
            self.__enumtest_state_THR(e)
        elif self.__compartment.state.__name__ == '__enumtest_state_FRI':
            self.__enumtest_state_FRI(e)
        
    def __transition(self, compartment: 'EnumTestCompartment'):
        self.__next_compartment = compartment
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class EnumTestCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    
# ********************

#class EnumTestController(EnumTest):
	#def __init__(self,):
	    #super().__init__()

    #def entered_do(self,msg: str,val: int):
        #pass

    #def left_do(self,msg: str,val: int):
        #pass

# ********************

