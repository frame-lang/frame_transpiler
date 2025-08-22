#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    sys1 = NoParamsSystem()
    sys2 = StartStateParameters("hello")
    sys3 = StartStateEnterParameters("world")
    sys4 = DomainVariables(1,2)
    sys5 = AllParameterTypes("hello","world",1,2)
    return
class NoParamsSystem:
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        # Constructor implementation will be added here
    
    # State machine will be added here
    
    # System runtime (__kernel, __router, __transition) will be added here
    
if __name__ == '__main__':
    main()
