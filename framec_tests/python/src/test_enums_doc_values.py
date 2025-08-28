#Emitted from framec_v0.30.0

from enum import Enum

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


class CalendarSystem_Days(Enum):
    SUNDAY = 0
    monday = 1
    Tuesday = 2
    WEDNESDAY = 3
    tHuRsDaY = 4
    FRIDAY = 5
    SATURDAY = 6
    SUNDAY = 7
class CalendarSystem:
    def __init__(self):
        self.__compartment = None
        self.return_stack = [None]

