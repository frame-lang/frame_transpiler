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


def main():
    print("=== Basic LEGB Test ===")
    module_var = "MODULE"
    print(module_var)
    test_function()
    print("Back in main")
    print(module_var)
    return

def test_function():
    print("=== Function Scope ===")
    func_var = "FUNCTION"
    print(func_var)
    if True:
        block_var = "BLOCK"
        print(block_var)
        print(func_var)
    print(func_var)
    return

if __name__ == '__main__':
    main()
