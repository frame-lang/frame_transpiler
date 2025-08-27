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
    simple_var = "test"# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print(simple_var)
    obj_var = TestSystem()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    obj_var.some_operation()
    return
class TestSystem:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]
    
    # ==================== Operations Block ================== #

if __name__ == '__main__':
    main()
