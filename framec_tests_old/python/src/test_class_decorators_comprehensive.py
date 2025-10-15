#Emitted from framec_v0.30.0

from dataclasses import dataclass
from typing import ClassVar

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


def test_decorators():
    p1 = Point()
    p1.x = 3
    p1.y = 4
    print("Point: (" + str(p1.x) + ", " + str(p1.y) + ")")
    p2 = Point3D()
    p2.x = 1
    p2.y = 2
    p2.z = 3
    print("Point3D: (" + str(p2.x) + ", " + str(p2.y) + ", " + str(p2.z) + ")")
    print("All decorator tests passed!")
    return


@dataclass
class Point:
    
    def __init__(self):
        self.x = 0
        self.y = 0



@dataclass
class Point3D:
    
    def __init__(self):
        self.x = 0
        self.y = 0
        self.z = 0


# Module initialization

test_decorators()


