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


def main():
    grader = GradeSystem()
    scores = [95,85,75,65,55,-5,105]# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== Grade System Tests ===")
    for score in scores:
        grade = grader.calculateGrade(score)# DEBUG_EXPR_TYPE: Discriminant(4)
        
        print("Score " + str(score) + " -> Grade: " + grade)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("\n=== Category Tests ===")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    grader.testCategories()
    return
class GradeSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__gradesystem_state_Processing', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def calculateGrade(self,score: int):
        parameters = {}
        parameters["score"] = score
        self.return_stack.append(None)
        __e = FrameEvent("calculateGrade",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def testCategories(self,):
        self.return_stack.append(None)
        __e = FrameEvent("testCategories",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Processing
    
    def __gradesystem_state_Processing(self, __e, compartment):
        if __e._message == "calculateGrade":
            if __e._parameters["score"] < 0:
                self.return_stack[-1] = "Invalid: negative score"
                return
            elif __e._parameters["score"] > 100:
                self.return_stack[-1] = "Invalid: score too high"
                return
            if __e._parameters["score"] >= 90:
                if __e._parameters["score"] >= 95:
                    self.return_stack[-1] = "A+"
                    return
                elif __e._parameters["score"] >= 93:
                    self.return_stack[-1] = "A"
                    return
                else:
                    self.return_stack[-1] = "A-"
                    return
            elif __e._parameters["score"] >= 80:
                if __e._parameters["score"] >= 87:
                    self.return_stack[-1] = "B+"
                    return
                elif __e._parameters["score"] >= 83:
                    self.return_stack[-1] = "B"
                    return
                else:
                    self.return_stack[-1] = "B-"
                    return
            elif __e._parameters["score"] >= 70:
                if __e._parameters["score"] >= 77:
                    self.return_stack[-1] = "C+"
                    return
                elif __e._parameters["score"] >= 73:
                    self.return_stack[-1] = "C"
                    return
                else:
                    self.return_stack[-1] = "C-"
                    return
            elif __e._parameters["score"] >= 60:
                self.return_stack[-1] = "D"
                return
            else:
                self.return_stack[-1] = "F"
                return
            return
        elif __e._message == "testCategories":
            testValue = 42
            if testValue < 25:# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("Low value")
                return
            elif testValue < 50:# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("Medium value: " + str(testValue))
                return
            elif testValue < 75:# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("High value")
                return
            else:# DEBUG_EXPR_TYPE: Discriminant(4)
                
                print("Very high value")
                return
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sProcessing(self, __e):
        return self.__gradesystem_state_Processing(__e, None)
    
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
        if target_compartment.state == '__gradesystem_state_Processing':
            self.__gradesystem_state_Processing(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()