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


class Module1:
    @staticmethod
    def process(value):
        return (value * 2) + 1


    @staticmethod
    def helper(base):
        result = 0
        for i in range(base):
            result = result + (i * 2)
        return result


    @staticmethod
    def compute_sum(numbers):
        total = 0
        for num in numbers:
            total = total + num
        return total


