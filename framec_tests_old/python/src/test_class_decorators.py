from dataclasses import dataclass, field
from typing import ClassVar

# Emitted from framec_v0.76.0


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


class Point:


    def __init__(self, x, y):
        self.x = x
        self.y = y
        return



class ImmutablePoint:


    def __init__(self, x, y):
        self.x = x
        self.y = y
        return



class OrderedPoint:


    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z
        return



class CustomPoint:

    class_counter = 0

    def __init__(self, x, y):
        self.x = x
        self.y = y
        CustomPoint.class_counter = CustomPoint.class_counter + 1
        return


    def distance_to_origin(self):
        return ((self.x ** 2) + (self.y ** 2)) ** 0.5



def test_decorators():
    p1 = Point(3, 4)
    p2 = ImmutablePoint(5, 12)
    p3 = OrderedPoint(1, 2, 3)
    p4 = CustomPoint(6, 8)
    print(f"Point: ({p1.x}, {p1.y})")
    print(f"ImmutablePoint: ({p2.x}, {p2.y})")
    print(f"OrderedPoint: ({p3.x}, {p3.y}, {p3.z})")
    print(f"CustomPoint: ({p4.x}, {p4.y})")
    print(f"CustomPoint instances: {CustomPoint.class_counter}")
    print(f"Distance: {p4.distance_to_origin()}")


def my_custom_decorator(cls):
    print(f"Decorating class {cls.__name__}")
    return cls



test_decorators()
