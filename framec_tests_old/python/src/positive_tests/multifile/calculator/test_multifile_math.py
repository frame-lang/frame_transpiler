# Emitted from framec_v0.81.6


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


class MathModule:
    operations_count = 0

    @staticmethod
    def add(a, b):
        MathModule.operations_count = MathModule.operations_count + 1
        return a + b


    @staticmethod
    def multiply(a, b):
        MathModule.operations_count = MathModule.operations_count + 1
        return a * b


    @staticmethod
    def power(base, exp):
        MathModule.operations_count = MathModule.operations_count + 1
        return base ** exp


    @staticmethod
    def getOperationsCount():
        return MathModule.operations_count


