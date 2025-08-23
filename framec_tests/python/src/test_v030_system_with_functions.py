#Emitted from framec_v0.30.0


class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


def main():
    sys = TestSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys.start()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    utility("test")
    return

def utility(msg):# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Utility: " + msg)
    return
#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    sys = TestSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    sys.start()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    utility("test")
    return
def utility(msg):# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Utility: " + msg)
    return
class TestSystem:
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        # Constructor implementation will be added here
    
    # Interface methods will be added here
    
    # State machine will be added here
    
    # System runtime (__kernel, __router, __transition) will be added here
    

if __name__ == '__main__':
    main()