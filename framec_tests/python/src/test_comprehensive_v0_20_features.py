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


def main():
    print("=== Frame v0.20 Comprehensive Feature Test ===")
    processor = AdvancedProcessor()
    results = [processor.processData(""),processor.processData("test"),processor.processData("ERROR"),processor.processData("Hello World"),processor.processData("very long text that exceeds the normal length limits")]
    print("\n=== Results ===")
    for result in results:
        print("Result: " + result)
    print("\n=== State Management Test ===")
    processor.reset()
    processor.configure("debug")
    processor.processData("debug test")
    return
class AdvancedProcessor:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__advancedprocessor_state_Idle', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def processData(self,input):
        parameters = {}
        parameters["input"] = input
        self.return_stack.append(None)
        __e = FrameEvent("processData",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def reset(self,):
        self.return_stack.append(None)
        __e = FrameEvent("reset",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def configure(self,mode):
        parameters = {}
        parameters["mode"] = mode
        self.return_stack.append(None)
        __e = FrameEvent("configure",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Idle
    
    def __advancedprocessor_state_Idle(self, __e, compartment):
        if __e._message == "$>":
            print("Processor ready in Idle state")
            return
        elif __e._message == "processData":
            if __e._parameters["input"] == "":
                self.return_stack[-1] = "error: empty input"
                return
            next_compartment = FrameCompartment('__advancedprocessor_state_Processing', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "configure":
            if __e._parameters["mode"] == "debug":
                print("Enabling debug mode")
                next_compartment = FrameCompartment('__advancedprocessor_state_Debug', None, None, None, None)
                self.__transition(next_compartment)
            elif __e._parameters["mode"] == "fast":
                print("Enabling fast mode")
                next_compartment = FrameCompartment('__advancedprocessor_state_FastProcessing', None, None, None, None)
                self.__transition(next_compartment)
            else:
                print("Unknown mode: " + __e._parameters["mode"])
            return
        elif __e._message == "reset":
            print("Already in idle state")
            return
    
    
    # ----------------------------------------
    # $Processing
    
    def __advancedprocessor_state_Processing(self, __e, compartment):
        if __e._message == "$>":
            print("Processing: " + (compartment.state_args["data"]))
            result = self.processText_do(compartment.state_args["data"])
            if result == "error":
                self.return_stack[-1] = "processing failed"
                next_compartment = FrameCompartment('__advancedprocessor_state_Idle', None, None, None, None)
                self.__transition(next_compartment)
            elif result == "warning":
                self.return_stack[-1] = "processed with warnings"
                next_compartment = FrameCompartment('__advancedprocessor_state_Idle', None, None, None, None)
                self.__transition(next_compartment)
            else:
                self.return_stack[-1] = "success: " + result
                next_compartment = FrameCompartment('__advancedprocessor_state_Idle', None, None, None, None)
                self.__transition(next_compartment)
            return
        elif __e._message == "reset":
            print("Resetting from processing state")
            next_compartment = FrameCompartment('__advancedprocessor_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Debug
    
    def __advancedprocessor_state_Debug(self, __e, compartment):
        if __e._message == "$>":
            print("Debug mode active")
            return
        elif __e._message == "processData":
            print("DEBUG: Processing '" + __e._parameters["input"] + "'")
            if __e._parameters["input"] == "debug test":
                self.return_stack[-1] = "debug: test successful"
                return
            self.return_stack[-1] = "debug: " + __e._parameters["input"]
            return
        elif __e._message == "reset":
            print("Exiting debug mode")
            next_compartment = FrameCompartment('__advancedprocessor_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $FastProcessing
    
    def __advancedprocessor_state_FastProcessing(self, __e, compartment):
        if __e._message == "processData":
            self.return_stack[-1] = "fast: " + __e._parameters["input"]
            next_compartment = FrameCompartment('__advancedprocessor_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "reset":
            next_compartment = FrameCompartment('__advancedprocessor_state_Idle', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sIdle(self, __e):
        return self.__advancedprocessor_state_Idle(__e, None)
    def _sProcessing(self, __e):
        return self.__advancedprocessor_state_Processing(__e, None)
    def _sDebug(self, __e):
        return self.__advancedprocessor_state_Debug(__e, None)
    def _sFastProcessing(self, __e):
        return self.__advancedprocessor_state_FastProcessing(__e, None)
    # ===================== Actions Block =================== #
    
    def processText_do(self,text):
        
        if text == "ERROR":
            return "error"
        if self.len_do(text) > 50:
            return "warning"
        if text == "test":
            return "validated"
        return "processed"
        return
        
    
    def len_do(self,s):
        
        count = 0
        for c in s:
            count = count + 1
        return count
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
        if target_compartment.state == '__advancedprocessor_state_Idle':
            self.__advancedprocessor_state_Idle(__e, target_compartment)
        elif target_compartment.state == '__advancedprocessor_state_Processing':
            self.__advancedprocessor_state_Processing(__e, target_compartment)
        elif target_compartment.state == '__advancedprocessor_state_Debug':
            self.__advancedprocessor_state_Debug(__e, target_compartment)
        elif target_compartment.state == '__advancedprocessor_state_FastProcessing':
            self.__advancedprocessor_state_FastProcessing(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
