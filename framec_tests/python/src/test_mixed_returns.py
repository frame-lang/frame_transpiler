#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    analyzer = TextAnalyzer()
    result1 = analyzer.analyze("")
    print("Empty: " + result1)
    result2 = analyzer.analyze("hello")
    print("hello: " + result2)
    result3 = analyzer.analyze("HELLO WORLD")
    print("HELLO WORLD: " + result3)
    result4 = analyzer.analyze("Frame v0.20 is great!")
    print("Frame v0.20 is great!: " + result4)
    return

class TextAnalyzer:
    
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        
         # Create and initialize start state compartment.
        
        next_compartment = None
        next_compartment = TextAnalyzerCompartment('__textanalyzer_state_Analyzing', next_compartment)
        self.__compartment = next_compartment
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Initialize domain
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Interface Block ================== #
    
    def analyze(self,text: str):
        parameters = {}
        parameters["text"] = text
        self.return_stack.append(None)
        __e = FrameEvent("analyze",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Analyzing
    
    def __textanalyzer_state_Analyzing(self, __e, compartment):
        if __e._message == "analyze":
            if __e._parameters["text"] == "":
                self.return_stack[-1] = "empty input"
                return
            category = self.categorizeText_do(__e._parameters["text"])
            print("Category from action: " + category)
            if category == "short":
                self.return_stack[-1] = "short text: " + __e._parameters["text"]
            elif category == "caps":
                self.return_stack[-1] = "LOUD TEXT: " + __e._parameters["text"]
            else:
                self.return_stack[-1] = "normal text: " + __e._parameters["text"]
            return
    
    # ===================== Actions Block =================== #
    
    def categorizeText_do(self,text: str):
        
        if self.len_do(text) < 10:
            return "short"
        if text == text.upper() and text != text.lower():
            return "caps"
        return "normal"
        return
        
    
    def len_do(self,s: str):
        
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
            self.__router(FrameEvent( "<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else: # there is a forwarded event
                if next_compartment.forward_event._message == "$>":
                    # forwarded event is enter event
                    self.__router(next_compartment.forward_event)
                else:
                    # forwarded event is not enter event
                    # send normal enter event
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
                    # and now forward event to new, intialized state
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
                
    
    def __router(self, __e, compartment=None):
        target_compartment = compartment or self.__compartment
        if target_compartment.state == '__textanalyzer_state_Analyzing':
            self.__textanalyzer_state_Analyzing(__e, target_compartment)
        
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
    

# ===================== Compartment =================== #

class TextAnalyzerCompartment:

    def __init__(self,state,parent_compartment):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
        self.parent_compartment = parent_compartment
    
if __name__ == '__main__':
    main()
