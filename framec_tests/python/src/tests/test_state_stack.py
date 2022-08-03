from state_stack.state_stack import StateStack

class StateStackController(StateStack):

    def log_do(self, msg: str):
        self.tape.append(msg)


def return_state_name(state):
    return f'__statestack_state_{state}'


class TestStateStack:
    """
    Tests the basic functionality of the state stack feature. This test case does not include any
    features that require a state context.
    """

    def test_push_pop(self):
        sm = StateStackController()
        assert sm.state_info() == return_state_name("A")
        sm.push()
        sm.to_b()
        assert sm.state_info() == return_state_name("B")
        sm.pop()
        assert sm.state_info() == return_state_name("A")
    

    def test_multiple_push_pops(self):
        """Test that multiple states can be pushed and subsequently restored by pops, LIFO style."""
        sm = StateStackController()
        assert sm.state_info() == return_state_name("A")
        sm.push()
        sm.to_c()
        sm.push()
        sm.to_a()
        sm.push()
        sm.push()
        sm.to_c() # no push
        sm.to_b()
        sm.push()
        sm.to_c()
        sm.push() # stack top-to-bottom: C, B, A, A, C, A
        sm.to_a()
        assert sm.state_info() == return_state_name("A")
        sm.pop()
        assert sm.state_info() == return_state_name("C")
        sm.to_a()
        assert sm.state_info() == return_state_name("A")
        sm.pop()
        assert sm.state_info() == return_state_name("B")
        sm.pop()
        assert sm.state_info() == return_state_name("A")
        sm.pop()
        assert sm.state_info() == return_state_name("A")
        sm.pop()
        assert sm.state_info() == return_state_name("C")
        sm.to_b()
        sm.push()
        sm.to_c()
        sm.push() # stack top-to-bottom: C, B, A
        sm.to_a()
        sm.to_b()
        assert sm.state_info() == return_state_name("B")
        sm.pop()
        assert sm.state_info() == return_state_name("C")
        sm.pop()
        assert sm.state_info() == return_state_name("B")
        sm.pop()
        assert sm.state_info() == return_state_name("A")
    


    def test_pop_transition_events(self):
        """Test that pop transitions trigger enter/exit events."""
        sm = StateStackController()
        sm.to_b()
        sm.push()
        sm.to_a()
        sm.push()
        sm.to_c()
        sm.push() # stack top-to-bottom: C, A, B
        sm.to_a()
        sm.tape.clear()
        assert sm.state_info() == return_state_name("A")
        sm.pop()
        assert sm.state_info() == return_state_name("C")
        assert sm.tape == ["A:<", "C:>"]
        sm.tape.clear()
        sm.pop()
        sm.pop()
        assert sm.state_info() == return_state_name("B")
        assert sm.tape == ["C:<", "A:>", "A:<", "B:>"]
    

    def test_pop_change_state_no_events(self):
        """Test that pop change-states do not trigger enter/exit events."""   
        sm = StateStackController()
        sm.to_b()
        sm.push()
        sm.to_a()
        sm.push()
        sm.to_c()
        sm.push() # stack top-to-bottom: C, A, B
        sm.to_a()
        sm.tape.clear()
        assert sm.state_info() == return_state_name("A")
        sm.pop_change()
        assert sm.state_info() == return_state_name("C")
        assert len(sm.tape) == 0
        sm.pop()
        sm.pop_change()
        assert sm.state_info() == return_state_name("B")
        assert sm.tape == ["C:<", "A:>"]
    