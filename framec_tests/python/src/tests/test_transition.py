from transition.transition import TransitionSm

class TransitionSmController(TransitionSm):

    def enter_do(self, state: str):
        self.enters.append(state)
    
    def exit_do(self, state: str):
        self.exits.append(state)
    
    def clear_all(self):
        self.enters.clear()
        self.exits.clear()

def return_state_name(state):

    return f'__transitionsm_state_{state}'

class TestTransitionEvents:

    """
    Frame supports two different operations for changing the current state of the machine:
    "change-state" (`->>`) which simply changes to the new state, and "transition" (`->`), which
    also sends an exit event to the old state and an enter event to the new state.
    """

    def test_transition_events(self):
        """
        Test that transition works and triggers enter and exit events.
        """
        sm = TransitionSmController()
        sm.clear_all()
        sm.transit()
        assert sm.state_info() == return_state_name('S1')
        assert sm.exits == ["S0"]
        assert sm.enters == ["S1"]

    def test_change_state_no_events(self):
        """Test that change-state works and does not trigger events."""

        sm = TransitionSmController()
        sm.clear_all()
        sm.change()
        assert sm.state_info() == return_state_name("S0")
        sm.transit()
        assert sm.state_info() == return_state_name("S1")
        sm.change()
        assert sm.state_info() == return_state_name("S2")
        sm.transit()
        assert sm.state_info() == return_state_name("S3")
        sm.transit()
        assert sm.state_info() == return_state_name("S0")
        assert sm.enters == ['S1','S3','S4']
        assert sm.exits == ['S0','S2','S3']
    




