#Emitted from framec_v0.11.0



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
        
        self.__compartment: 'EnumTestCompartment' = EnumTestCompartment('__enumtest_state_SUN')
        self.__next_compartment: 'EnumTestCompartment' = None
        
        # Initialize domain
        
        self.days  = []
        
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
    
    # ===================== Machine Block =================== #
    
    # ----------------------------------------
    # $SUN
    
    def __enumtest_state_SUN(self, __e):
        if __e._message == "A":
            self.entered_do("SUNDAY",EnumTest_Days.SUNDAY)
            return
        elif __e._message == "B":
            self.left_do("MONDAY",EnumTest_Days.MONDAY)
            next_compartment = EnumTestCompartment('__enumtest_state_MON')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $MON
    
    def __enumtest_state_MON(self, __e):
        if __e._message == "A":
            self.entered_do("MONDAY",EnumTest_Days.MONDAY)
            return
        elif __e._message == "B":
            self.left_do("TUESDAY",EnumTest_Days.TUESDAY)
            next_compartment = EnumTestCompartment('__enumtest_state_TUE')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $TUE
    
    def __enumtest_state_TUE(self, __e):
        if __e._message == "A":
            self.entered_do("TUESDAY",EnumTest_Days.TUESDAY)
            return
        elif __e._message == "B":
            self.left_do("WEDNESDAY",EnumTest_Days.WEDNESDAY)
            next_compartment = EnumTestCompartment('__enumtest_state_WED')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $WED
    
    def __enumtest_state_WED(self, __e):
        if __e._message == "A":
            self.entered_do("WEDNESDAY",EnumTest_Days.WEDNESDAY)
            return
        elif __e._message == "B":
            self.left_do("THURSDAY",EnumTest_Days.THURSDAY)
            next_compartment = EnumTestCompartment('__enumtest_state_THR')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $THR
    
    def __enumtest_state_THR(self, __e):
        if __e._message == "A":
            self.entered_do("THURSDAY",EnumTest_Days.THURSDAY)
            return
        elif __e._message == "B":
            self.left_do("FRIDAY",EnumTest_Days.FRIDAY)
            next_compartment = EnumTestCompartment('__enumtest_state_FRI')
            self.__transition(next_compartment)
            return
    
    # ----------------------------------------
    # $FRI
    
    def __enumtest_state_FRI(self, __e):
        if __e._message == "A":
            self.entered_do("FRIDAY",EnumTest_Days.FRIDAY)
            return
        elif __e._message == "B":
            self.left_do("SUNDAY",EnumTest_Days.SUNDAY)
            next_compartment = EnumTestCompartment('__enumtest_state_SUN')
            self.__transition(next_compartment)
            return
    
    # ===================== Actions Block =================== #
    
    def entered_do(self,msg: str,val: int):
        raise NotImplementedError
    
    def left_do(self,msg: str,val: int):
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
        if self.__compartment.state == '__enumtest_state_SUN':
            self.__enumtest_state_SUN(__e)
        elif self.__compartment.state == '__enumtest_state_MON':
            self.__enumtest_state_MON(__e)
        elif self.__compartment.state == '__enumtest_state_TUE':
            self.__enumtest_state_TUE(__e)
        elif self.__compartment.state == '__enumtest_state_WED':
            self.__enumtest_state_WED(__e)
        elif self.__compartment.state == '__enumtest_state_THR':
            self.__enumtest_state_THR(__e)
        elif self.__compartment.state == '__enumtest_state_FRI':
            self.__enumtest_state_FRI(__e)
        
    def __transition(self, next_compartment: 'EnumTestCompartment'):
        self.__next_compartment = next_compartment
    
    def state_info(self):
        return self.__compartment.state
        

# ===================== Compartment =================== #

class EnumTestCompartment:

    def __init__(self,state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
    