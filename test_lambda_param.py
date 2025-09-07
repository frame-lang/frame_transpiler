#Emitted from framec_v0.30.0


class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None, state_vars=None, state_args=None):
        self.state = state
        self.forward_event = forward_event
        self.exit_args = exit_args
        self.enter_args = enter_args
        self.parent_compartment = parent_compartment
        self.state_vars = state_vars or {}
        self.state_args = state_args or {}


def apply_operation(op,x,y):
    return op(x,y)
    return

def test_lambda_as_param():
    add = lambda a, b: a + b
    mul = lambda a, b: a * b
    result1 = apply_operation(add,5,3)
    print("Add result: " + str(result1))
    result2 = apply_operation(mul,5,3)
    print("Mul result: " + str(result2))
    return

def main():
    test_lambda_as_param()
    return

if __name__ == '__main__':
    main()
