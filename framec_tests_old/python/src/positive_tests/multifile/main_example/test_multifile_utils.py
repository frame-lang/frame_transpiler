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


class MathUtils:
    PI = 3.14159

    @staticmethod
    def add(a, b):
        return a + b


    @staticmethod
    def multiply(a, b):
        return a * b


    @staticmethod
    def circleArea(radius):
        return (MathUtils.PI * radius) * radius


    @staticmethod
    def isEven(number):
        return (number % 2) == 0


    @staticmethod
    def factorial(n):
        if n <= 1:
            return 1
        return n * factorial(n - 1)


