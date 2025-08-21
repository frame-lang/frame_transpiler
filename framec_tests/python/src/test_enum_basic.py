#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


class CalendarSystem_Days(Enum):
    SUNDAY = 0
    monday = 1
    Tuesday = 2
    WEDNESDAY = 3
    tHuRsDaY = 4
    FRIDAY = 5
    SATURDAY = 6


class CalendarSystem:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
        self.__compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        
    
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        pass
    

# ===================== Compartment =================== #

class CalendarSystemCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    