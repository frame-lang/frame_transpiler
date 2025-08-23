#Emitted from framec_v0.30.0


class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


def main():
    calculator = Calculator()
    result1 = calculator.getDefault()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Default value: " + result1)
    result2 = calculator.calculate(10,5)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("10 + 5 = " + str(result2))
    result3 = calculator.divide(10,0)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("10 / 0 = " + result3)
    result4 = calculator.divide(10,2)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("10 / 2 = " + str(result4))
    return
#Emitted from framec_v0.30.0



class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

def main():
    calculator = Calculator()
    result1 = calculator.getDefault()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Default value: " + result1)
    result2 = calculator.calculate(10,5)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("10 + 5 = " + str(result2))
    result3 = calculator.divide(10,0)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("10 / 0 = " + result3)
    result4 = calculator.divide(10,2)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("10 / 2 = " + str(result4))
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