#Emitted from framec_v0.30.0


class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


def main():
    hsm = ParentDispatchTest()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.test1()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.test2()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.test3()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.next()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.test4()
    return
#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    hsm = ParentDispatchTest()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.test1()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.test2()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.test3()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.next()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    hsm.test4()
    return
class ParentDispatchTest:
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        # Constructor implementation will be added here
    
    # Interface methods will be added here
    
    # State machine will be added here
    
    # System runtime (__kernel, __router, __transition) will be added here
    

if __name__ == '__main__':
    main()