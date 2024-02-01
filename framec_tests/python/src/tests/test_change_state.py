from change_state.change_state import ChangeStateSm

def return_state_name(state):

    return f'__changestatesm_state_{state}'

class TestChangeState:


    def test_change_states(self):
        """Simple test of change state functionality."""

        sm = ChangeStateSm()

        sm.change()
        assert sm.state_info() == return_state_name("S1")
        sm.change()
        assert sm.state_info() == return_state_name("S2")
        sm.change()
        assert sm.state_info() == return_state_name("S3")
        sm.change()
        assert sm.state_info() == return_state_name("S4")
        sm.change()
        assert sm.state_info() == return_state_name("S0")
    




