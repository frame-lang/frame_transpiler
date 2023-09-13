# emitted from framec_v0.11.0
# get include files at https://github.com/frame-lang/frame-ancillary-files
from framelang.framelang import FrameEvent

class AutoIncDec:
    
    def __init__(self):
        
         # Create and intialize start state compartment.
        self.__state = self.__autoincdec_state_Inc
        self.__compartment: 'AutoIncDecCompartment' = AutoIncDecCompartment(self.__state)
        self.__next_compartment: 'AutoIncDecCompartment' = None
        
        # Initialize domain
        
        self.a : int = 0
        self.b : int = 0
        self.c : int = 0
        
        # Send system start event
        frame_event = FrameEvent(">", None)
        self.__mux(frame_event)
    
    # ===================== Interface Block =================== #
    
    def pre(self,):
        e = FrameEvent("pre",None)
        self.__mux(e)
    
    def post(self,):
        e = FrameEvent("post",None)
        self.__mux(e)
    
    def trans(self,):
        e = FrameEvent("trans",None)
        self.__mux(e)
    
    # ====================== Multiplexer ==================== #
    
    def __mux(self, e):
        if self.__compartment.state.__name__ == '__autoincdec_state_Inc':
            self.__autoincdec_state_Inc(e)
        elif self.__compartment.state.__name__ == '__autoincdec_state_Dec':
            self.__autoincdec_state_Dec(e)
        
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
    
    def __autoincdec_state_Inc(self, e):
        if e._message == "pre":
            
            self.a = self.a + 1
            self.b = self.a
            print(self.b)
            
            return
        
        elif e._message == "post":
            self.c = self.a
            self.a = self.a + 1
            print(self.c)
            
            return
        
        elif e._message == "trans":
            compartment = AutoIncDecCompartment(self.__autoincdec_state_Dec)
            self.__transition(compartment)
            
            return
        
    def __autoincdec_state_Dec(self, e):
        if e._message == "pre":
            
            self.a = self.a - 1
            self.b = self.a
            print(self.b)
            
            return
        
        elif e._message == "post":
            self.c = self.a
            self.a = self.a - 1
            print(self.c)
            
            return
        
        elif e._message == "trans":
            compartment = AutoIncDecCompartment(self.__autoincdec_state_Inc)
            self.__transition(compartment)
            
            return
        
    
    # ===================== Actions Block =================== #
    
    
    
    def print_it_do(self,val):
        print(val)
    
    
    # =============== Machinery and Mechanisms ============== #
    
    def __transition(self, compartment: 'AutoIncDecCompartment'):
        self.__next_compartment = compartment
    
    def  __do_transition(self, next_compartment: 'AutoIncDecCompartment'):
        self.__mux(FrameEvent("<", self.__compartment.exit_args))
        self.__compartment = next_compartment
        self.__mux(FrameEvent(">", self.__compartment.enter_args))
    
    def state_info(self):
        return self.__compartment.state.__name__
        

# ===================== Compartment =================== #

class AutoIncDecCompartment:

    def __init__(self, state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = FrameEvent(None, None)
    


# ********************

#class AutoIncDecController(AutoIncDec):
	#def __init__(self,):
	    #super().__init__()

    #def print_it_do(self,val):

# ********************

