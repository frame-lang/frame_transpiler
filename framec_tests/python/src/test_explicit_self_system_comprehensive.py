#Emitted from framec_v0.30.0

from enum import Enum

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None, state_vars=None, state_args=None):
        self.state = state
        self.forward_event = forward_event
        self.exit_args = exit_args
        self.enter_args = enter_args
        self.parent_compartment = parent_compartment
        self.state_vars = state_vars or {}
        self.state_args = state_args or {}


def main():
    sys = ComprehensiveSystem()
    sys.processTask("task1")
    sys.calculate(10,20)
    return
class ComprehensiveSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__comprehensivesystem_state_Ready', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.taskCount: int = 0
        self.total: int = 0
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def publicOp(self):
        print("Public operation")
    
    def helperOp(self,value: int):
        return value * 2
    # ==================== Interface Block ================== #
    
    def processTask(self,taskName: str):
        parameters = {}
        parameters["taskName"] = taskName
        self.return_stack.append(None)
        __e = FrameEvent("processTask",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def calculate(self,a: int,b: int):
        parameters = {}
        parameters["a"] = a
        parameters["b"] = b
        self.return_stack.append(None)
        __e = FrameEvent("calculate",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Ready
    
    def __comprehensivesystem_state_Ready(self, __e, compartment):
        if __e._message == "processTask":
            self._logTask(__e._parameters["taskName"])
            
            self.publicOp()
            
            self.taskCount = self.taskCount + 1
            result = self._computeHash(__e._parameters["taskName"])
            print("Hash: " + str(result))
            self._processInternal()
            
            return
        elif __e._message == "calculate":
            doubled = self.helperOp(__e._parameters["a"])
            self._updateTotal(doubled + __e._parameters["b"])
            
            self.return_stack[-1] = self.total
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sReady(self, __e):
        return self.__comprehensivesystem_state_Ready(__e, None)
    # ===================== Actions Block =================== #
    
    def _logTask(self,name: str):
        
        print("Logging task: " + name)
        return
        
    
    def _computeHash(self,input: str):
        
        return len(input) * 42
        return
        
    
    def _processInternal(self):
        
        self._internalHelper()
        
        self.publicOp()
        
        return
        
    
    def _internalHelper(self):
        
        print("Internal helper called")
        return
        
    
    def _updateTotal(self,value: int):
        
        self.total = self.total + value
        print("Total updated to: " + str(self.total))
        return
        
    
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
        if target_compartment.state == '__comprehensivesystem_state_Ready':
            self.__comprehensivesystem_state_Ready(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class SecondarySystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__secondarysystem_state_Start', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def test(self,):
        self.return_stack.append(None)
        __e = FrameEvent("test",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __secondarysystem_state_Start(self, __e, compartment):
        if __e._message == "test":
            self._ownAction()
            
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__secondarysystem_state_Start(__e, None)
    # ===================== Actions Block =================== #
    
    def _ownAction(self):
        
        print("SecondarySystem action")
        return
        
    
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
        if target_compartment.state == '__secondarysystem_state_Start':
            self.__secondarysystem_state_Start(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
