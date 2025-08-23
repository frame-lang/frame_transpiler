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
    grader = GradeProcessor()
    grade = grader.processScore(85)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Grade: " + grade)
    return
class GradeProcessor:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__gradeprocessor_state_Start', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def processScore(self,score: int):
        parameters = {}
        parameters["score"] = score
        self.return_stack.append(None)
        __e = FrameEvent("processScore",parameters)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __gradeprocessor_state_Start(self, __e, compartment):
        if __e._message == "processScore":
            if __e._parameters["score"] < 0:
                return "Invalid"
            elif __e._parameters["score"] > 100:
                return "Invalid"
            if __e._parameters["score"] >= 90:
                return "A"
            elif __e._parameters["score"] >= 80:
                return "B"
            elif __e._parameters["score"] >= 70:
                return "C"
            elif __e._parameters["score"] >= 60:
                return "D"
            else:
                return "F"
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__gradeprocessor_state_Start(__e, None)
    
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
        if target_compartment.state == '__gradeprocessor_state_Start':
            self.__gradeprocessor_state_Start(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
