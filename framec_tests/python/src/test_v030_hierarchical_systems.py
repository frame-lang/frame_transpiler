#Emitted from framec_v0.30.0

from enum import Enum

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None):
        self.state = state
        self.forward_event = forward_event
        self.exit_args = exit_args
        self.enter_args = enter_args
        self.parent_compartment = parent_compartment

class Parent:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__parent_state_Root', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def start(self,):
        self.return_stack.append(None)
        __e = FrameEvent("start",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def process(self,data):
        parameters = {}
        parameters["data"] = data
        self.return_stack.append(None)
        __e = FrameEvent("process",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Root
    
    def __parent_state_Root(self, __e, compartment):
        if __e._message == "start":
            next_compartment = FrameCompartment('__parent_state_Active', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "process":
            print("Parent processing: " + __e._parameters["data"])
            return
    
    
    # ----------------------------------------
    # $Active
    
    def __parent_state_Active(self, __e, compartment):
        if __e._message == "process":
            print("Parent Active processing: " + __e._parameters["data"])
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sRoot(self, __e):
        return self.__parent_state_Root(__e, None)
    def _sActive(self, __e):
        return self.__parent_state_Active(__e, None)
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent("<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else:
                # forwarded event
                if next_compartment.forward_event._message == "$>":
                    self.__router(next_compartment.forward_event)
                else:
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
    
    def __router(self, __e, compartment=None):
        target_compartment = compartment or self.__compartment
        if target_compartment.state == '__parent_state_Root':
            self.__parent_state_Root(__e, target_compartment)
        elif target_compartment.state == '__parent_state_Active':
            self.__parent_state_Active(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class AnotherParent:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__anotherparent_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def begin(self,):
        self.return_stack.append(None)
        __e = FrameEvent("begin",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def handle(self,msg):
        parameters = {}
        parameters["msg"] = msg
        self.return_stack.append(None)
        __e = FrameEvent("handle",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __anotherparent_state_Start(self, __e, compartment):
        if __e._message == "begin":
            next_compartment = FrameCompartment('__anotherparent_state_Running', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Running
    
    def __anotherparent_state_Running(self, __e, compartment):
        if __e._message == "handle":
            print("Running: " + __e._parameters["msg"])
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__anotherparent_state_Start(__e, None)
    def _sRunning(self, __e):
        return self.__anotherparent_state_Running(__e, None)
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent("<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else:
                # forwarded event
                if next_compartment.forward_event._message == "$>":
                    self.__router(next_compartment.forward_event)
                else:
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
    
    def __router(self, __e, compartment=None):
        target_compartment = compartment or self.__compartment
        if target_compartment.state == '__anotherparent_state_Start':
            self.__anotherparent_state_Start(__e, target_compartment)
        elif target_compartment.state == '__anotherparent_state_Running':
            self.__anotherparent_state_Running(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

