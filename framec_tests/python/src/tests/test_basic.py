from basic.basic import Basic


class BasicController(Basic):

    def entered_do(self, msg: str):
        self.entry_log.append(msg)

    def left_do(self, msg: str):
        self.exit_log.append(msg)


class TestBasic:

    def test_intial_enter_event(self):
        """Test that the enter event is sent for entering the initial state on startup."""

        sm = BasicController()
        assert sm.entry_log == ["S0"]

    def test_transition_enter_events(self):
        """Test that enter events are sent to the new state on transition."""
        sm = BasicController()
        sm.entry_log = []
        sm.A()
        sm.B()
        assert sm.entry_log == ["S1", "S0"]
    
    def test_transition_exit_events(self):
        """Test that exit events are sent to the old state on transition."""

        sm = BasicController()
        sm.A()
        sm.B()
        assert sm.exit_log == ["S0", "S1"]
    
    def test_current_state(self):
        """Test that the state of the machine is updated correctly."""

        sm = BasicController()
        assert sm.state_info() == "__basic_state_S0"
        sm.A()
        assert sm.state_info() == "__basic_state_S1"
        sm.B()
        assert sm.state_info() == "__basic_state_S0"
