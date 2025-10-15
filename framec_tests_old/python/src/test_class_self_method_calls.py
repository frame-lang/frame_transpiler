# Emitted from framec_v0.81.2


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


class Calculator:


    def add(self, a, b):
        return a + b


    def multiply(self, a, b):
        return a * b


    def calculate(self, a, b, op):
        if op == "add":
            return self.add(a, b)
        else:
            return self.multiply(a, b)
        return



def main():
    calc = Calculator()
    sum = calc.calculate(5, 3, "add")
    print("5 + 3 = " + str(sum))
    product = calc.calculate(5, 3, "multiply")
    print("5 * 3 = " + str(product))


if __name__ == '__main__':
    main()
