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


def make_adder(n):
    adder = lambda x: x + n
    return adder
    return

def main():
    add5 = make_adder(5)
    print("5 + 3 = " + str(add5(3)))
    add10 = make_adder(10)
    print("10 + 3 = " + str(add10(3)))
    return

if __name__ == '__main__':
    main()
