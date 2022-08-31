from state_context_stack.state_context_stack import StateContextStack


class StateContextStackController(StateContextStack):

    def log_do(self, msg: str):
        return self.tape.append(msg)


def return_state_name(msg):

    return f'__statecontextstack_state_{msg}'


class TestStateContextStack:
    """
    Tests the state stack feature when states have associated contexts.

    Most features of state contexts are not supported by state stacks. In particular, state
    parameters and enter/exit parameters are not supported. The reason is that when transitioning
    to a popped state, the state is not known statically, so there is no way for the programmer to
    know what arguments must be passed.
    
    However, state variables are supported by the state stack feature. The interaction of those
    features is tested here.
    
    Additionally, the basic functionality of state stacks are tested again here since pushing and
    popping with state contexts is a different code path than pushing and popping without."""

    def test_push_pop(self):
        """
         Test that a pop restores a pushed state.
        """

        sm = StateContextStackController()
        assert sm.state_info() == return_state_name("A")
        sm.push()
        sm.to_b()
        assert sm.state_info() == return_state_name("B")
        sm.pop()
        assert sm.state_info() == return_state_name("A")
    
    def test_multiple_push_pop(self):
        """
        Test that multiple states can be pushed and subsequently restored by pops, LIFO style.
        """
        
        sm = StateContextStackController()
        assert sm.state_info() ==  return_state_name("A")
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
        assert sm.state_info() ==  return_state_name("C")
        sm.to_a()
        assert sm.state_info() ==  return_state_name("A")
        sm.pop()
        assert sm.state_info() ==  return_state_name("B")
        sm.pop()
        assert sm.state_info() ==  return_state_name("A")
        sm.pop()
        assert sm.state_info() ==  return_state_name("A")
        sm.pop()
        assert sm.state_info() ==  return_state_name("C")
        sm.to_b()
        sm.push()
        sm.to_c()
        sm.push() # stack top-to-bottom: C, B, A
        sm.to_a()
        sm.to_b()
        assert sm.state_info() ==  return_state_name("B")
        sm.pop()
        assert sm.state_info() ==  return_state_name("C")
        sm.pop()
        assert sm.state_info() ==  return_state_name("B")
        sm.pop()
        assert sm.state_info() ==  return_state_name("A")
    
    def test_pop_transition_events(self):
        """Test that pop transitions trigger enter/exit events."""
        sm = StateContextStackController()
        sm.to_b()
        sm.push()
        sm.to_a()
        sm.push()
        sm.to_c()
        sm.push() # stack top-to-bottom: C, A, B
        sm.to_a()
        sm.tape.clear()
        assert sm.state_info() ==  return_state_name("A")
        sm.pop()
        assert sm.state_info() ==  return_state_name("C")
        assert sm.tape ==  ["A:<", "C:>"]
        sm.tape.clear()
        sm.pop()
        sm.pop()
        assert sm.state_info() ==  return_state_name("B")
        assert sm.tape ==  ["C:<", "A:>", "A:<", "B:>"]

    def test_pop_change_state_no_events(self):
        """Test that pop change-states do not trigger enter/exit events."""
        sm = StateContextStackController()
        sm.to_b()
        sm.push()
        sm.to_a()
        sm.push()
        sm.to_c()
        sm.push() # stack top-to-bottom: C, A, B
        sm.to_a()
        sm.tape.clear()
        assert sm.state_info() ==  return_state_name("A")
        sm.pop_change()
        assert sm.state_info() ==  return_state_name("C")
        assert len(sm.tape) == 0
        sm.pop()
        sm.pop_change()
        assert sm.state_info() ==  return_state_name("B")
        assert sm.tape ==  ["C:<", "A:>"]

    def test_pop_restores_state_variables(self):
        """
        Test that state variables are restored after pop.
        """
        sm = StateContextStackController()
        sm.inc()
        sm.inc()
        sm.push()
        assert sm.state_info() == return_state_name("A")
        assert sm.value() == 2
        sm.to_b()
        sm.inc()
        sm.push()
        assert sm.state_info() == return_state_name("B")
        assert sm.value() == 5
        sm.to_c()
        sm.inc()
        sm.inc()
        sm.inc()
        sm.push()
        assert sm.state_info() == return_state_name("C")
        assert sm.value() == 30
        sm.to_a()
        sm.inc()
        assert sm.state_info() == return_state_name("A")
        assert sm.value() == 1
        sm.pop()
        assert sm.state_info() == return_state_name("C")
        assert sm.value() == 30
        sm.pop()
        assert sm.state_info() ==  return_state_name("B")
        assert sm.value() ==  5
        sm.to_a()
        sm.inc()
        sm.inc()
        sm.inc()
        sm.push()
        assert sm.state_info() ==  return_state_name("A")
        assert sm.value() ==  3
        sm.to_c()
        sm.inc()
        assert sm.state_info() ==  return_state_name("C")
        assert sm.value() ==  10
        sm.pop()
        assert sm.state_info() ==  return_state_name("A")
        assert sm.value() ==  3
        sm.pop()
        assert sm.state_info() ==  return_state_name("A")
        assert sm.value() ==  2

    def push_stores_state_variable_snapshot(self):
        """Test that push stores a snapshot of the current values of state variables. Any changes to
        state variables after a push should not be reflected after that state is popped."""

        sm = StateContextStackController()
        sm.inc()
        sm.inc()
        sm.push()
        assert sm.state_info() == return_state_name("A")
        assert sm.value() == 2
        sm.inc()
        sm.inc()
        assert sm.value() == 4

        sm.to_b()
        sm.inc()
        sm.push()
        assert sm.state_info() ==  return_state_name("B")
        assert sm.value() == 5
        sm.inc()
        sm.inc()
        assert sm.value() == 15 # these changes should be forgotten

        sm.to_c()
        sm.inc()
        sm.inc()
        sm.inc()
        sm.push()
        assert sm.state_info() ==  return_state_name("C")
        assert sm.value() == 30
        sm.inc()
        assert sm.value() == 40 # forgotten

        sm.to_a()
        sm.inc()
        assert sm.state_info() ==  return_state_name("A")
        assert sm.value() == 1

        sm.pop()
        assert sm.state_info() ==  return_state_name("C")
        assert sm.value() == 30

        sm.pop()
        assert sm.state_info() ==  return_state_name("B")
        assert sm.value() == 5

        sm.to_a()
        sm.inc()
        sm.inc()
        sm.inc()
        sm.push()
        assert sm.state_info() ==  return_state_name("A")
        assert sm.value() == 3
        sm.inc()
        assert sm.value() == 4 # forgotten

        sm.to_c()
        sm.inc()
        assert sm.state_info() ==  return_state_name("C")
        assert sm.value() == 10

        sm.pop()
        assert sm.state_info() == return_state_name("A")
        assert sm.value() == 3

        sm.pop()
        assert sm.state_info() ==  return_state_name("A")
        assert sm.value() == 2




