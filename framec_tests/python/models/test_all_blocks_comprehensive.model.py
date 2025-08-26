#Emitted from framec_v0.30.0


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

class AllBlocksTest:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__allblockstest_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    
    # ==================== Operations Block ================== #
    
    def setup(self):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Operations: setup called")
    
    def process_data(self,value):# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Operations: processing " + value)
        return "processed_" + value
    # ==================== Interface Block ================== #
    
    def start_test(self,):
        self.return_stack.append(None)
        __e = FrameEvent("start_test",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def get_result(self,):
        self.return_stack.append(None)
        __e = FrameEvent("get_result",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __allblockstest_state_Start(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Machine: Start state entered")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            self.setup()# DEBUG: TransitionStmt
            
            next_compartment = Noneself.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Processing
    
    def __allblockstest_state_Processing(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Machine: Processing state entered")# DEBUG_EXPR_TYPE: Discriminant(5)
            
            result = self.process_data("test_data")# DEBUG_EXPR_TYPE: Discriminant(5)
            
            self.test_result = result# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Machine: result stored as " + self.test_result)# DEBUG: TransitionStmt
            
            next_compartment = Noneself.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Complete
    
    def __allblockstest_state_Complete(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Machine: Complete state entered")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Actions: calling complete_process")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            self.complete_process_do()
            return
        elif __e._message == "start_test":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Machine: start_test interface called")# DEBUG: TransitionStmt
            
            next_compartment = Noneself.__transition(next_compartment)
            return
        elif __e._message == "get_result":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Machine: get_result interface called")
            self.return_stack[-1] = self.test_result
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__allblockstest_state_Start(__e, None)
    def _sProcessing(self, __e):
        return self.__allblockstest_state_Processing(__e, None)
    def _sComplete(self, __e):
        return self.__allblockstest_state_Complete(__e, None)
    # ===================== Actions Block =================== #
    
    def complete_process_do(self):
        # DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Actions: complete_process called")# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Actions: stored result is " + self.test_result)
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
        if target_compartment.state == '__allblockstest_state_Start':
            self.__allblockstest_state_Start(__e, target_compartment)
        elif target_compartment.state == '__allblockstest_state_Processing':
            self.__allblockstest_state_Processing(__e, target_compartment)
        elif target_compartment.state == '__allblockstest_state_Complete':
            self.__allblockstest_state_Complete(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

