from state_vars.state_vars import StateVars

class StateVarsController(StateVars):

    def __init__(self):
        super().__init__()

def return_state_name(state):
    return f'__statevars_state_{state}'


class TestStateVars:

    def test_single_variable(self):

        sm = StateVarsController()
        assert sm.state_info() == return_state_name('A')
        assert sm.compartment_info().state_vars['x'] == 0
        sm.X() # increment x
        sm.X() # increment x
        assert sm.compartment_info().state_vars['x'] == 2

    
    def test_multiple_variables(self):

        sm = StateVarsController()
        sm.Y() # transition to B
        assert sm.state_info() == return_state_name("B")
        assert sm.compartment_info().state_vars['y'] == 10
        assert sm.compartment_info().state_vars['z'] == 100
        sm.Y(); # increment y
        sm.Y(); # increment y
        sm.Z(); # increment z
        sm.Y(); # increment y
        assert sm.compartment_info().state_vars['y'] == 13
        assert sm.compartment_info().state_vars['z'] == 101
    
    def test_variables_are_reset(self):

        sm = StateVarsController()
        sm.X() # increment x
        sm.X(); # increment x
        assert sm.compartment_info().state_vars["x"] == 2
        sm.Z(); # transition to B
        sm.Z(); # increment z
        sm.Y(); # increment y
        sm.Z(); # increment z
        assert sm.compartment_info().state_vars['y'] == 11
        assert sm.compartment_info().state_vars['z'] == 102
        sm.X() # transition to A
        assert sm.compartment_info().state_vars["x"] == 0
        sm.Y() # transition to B
        assert sm.compartment_info().state_vars['y'] == 10
        assert sm.compartment_info().state_vars['z'] == 100



