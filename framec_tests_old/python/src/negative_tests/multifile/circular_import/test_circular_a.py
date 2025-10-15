import ModuleB

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


# MODULE ModuleA start
class ModuleA:
    @staticmethod
    def functionA():
        # FUNC functionA: empty=false, vars={}, ctx=["ModuleA"]
        # DEBUG: func=functionA, vars={}, name=Some("ModuleA")
        return "A calls B: " + ModuleA.ModuleB.functionB()

