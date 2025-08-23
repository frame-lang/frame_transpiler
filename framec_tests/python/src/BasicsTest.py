#Emitted from framec_v0.30.0


class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters


def main():# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("=== Frame v0.20 Basics Test ===")
    x = 0
    name = "Spock"
    array = [4][2]int{{10, 11}, {20, 21}, {30, 31}, {40, 41}}# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Variable x: " + str(x))# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Name: " + name)# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Array initialized")# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Basics test completed")
    return

if __name__ == '__main__':
    main()