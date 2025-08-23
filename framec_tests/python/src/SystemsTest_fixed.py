#Emitted from framec_v0.30.0


class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


def main():
    sys = NoParameters()
    # Trigger enter event to start the system
    sys._sStart(FrameEvent("$>", []))
    return
class NoParameters:
    def __init__(self):
        self.__state = self._sStart
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Start
    
    def __noparameters_state_Start(self, __e, compartment):
        if __e._message == "$>":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("NoParameters started")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sStart(self, __e):
        return self.__noparameters_state_Start(__e, None)

if __name__ == '__main__':
    main()
