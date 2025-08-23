#Emitted from framec_v0.30.0


class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


def main():
    calc = Calculator()
    sum = calc.add(5,3)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("5 + 3 = " + str(sum))
    category = calc.categorizeNumber(42)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("42 is: " + category)
    return
#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    calc = Calculator()
    sum = calc.add(5,3)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("5 + 3 = " + str(sum))
    category = calc.categorizeNumber(42)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("42 is: " + category)
    return
class Calculator:
    
    # ==================== System Factory =================== #
    
    def __init__(self):
        # Constructor implementation will be added here
    
    # Interface methods will be added here
    
    # State machine will be added here
    
    # System runtime (__kernel, __router, __transition) will be added here
    

if __name__ == '__main__':
    main()