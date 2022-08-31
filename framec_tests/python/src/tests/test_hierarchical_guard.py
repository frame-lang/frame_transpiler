from hierarchical_guard.hierarchical_guard import HierarchicalGuard

class HierarchicalGuardController(HierarchicalGuard):

    def log_do(self, msg: str):
        self.tape.append(msg)

def return_state_name(state: str) -> str:

    return f'__hierarchicalguard_state_{state}'

class TestHierarchicalGuard:
    """Test guarded transitions in hierarchical state machines."""

    def test_propagate_to_parent(self):
        """Test that basic conditional transitions work properly. In particular,
        that control propagates to a parent handler if a child handler does
        not transition and ends in a continue (`:>`)."""

        sm = HierarchicalGuardController()
        sm.A(0)
        sm.tape.clear()
        assert sm.state_info() == return_state_name("S0")
        sm.A(20)
        assert sm.state_info() == return_state_name("S2")
        assert sm.tape == ["S0.A"]

        sm = HierarchicalGuardController()
        sm.A(0)
        sm.tape.clear()
        assert sm.state_info() == return_state_name("S0")
        sm.A(-5)
        assert sm.state_info() == return_state_name("S0")
        assert sm.tape == ["S0.A", "S.A"]

        sm = HierarchicalGuardController()
        sm.A(0)
        sm.tape.clear()
        assert sm.state_info() == return_state_name("S0")
        sm.B(-5)
        assert sm.state_info() == return_state_name("S1")
        assert sm.tape == ["S0.B"]

        sm = HierarchicalGuardController()
        sm.A(0)
        sm.tape.clear()
        assert sm.state_info() == return_state_name("S0")
        sm.B(5)
        assert sm.state_info() == return_state_name("S2")
        assert sm.tape == ["S0.B", "S.B"]

    
    def test_propagate_multiple_levels(self):
        """Test that control propagates across across multiple levels if a
            transition is not initiated."""
        sm = HierarchicalGuardController()
        sm.B(0)
        sm.tape.clear()
        assert sm.state_info() == return_state_name("S2")
        sm.A(7)
        assert sm.state_info() == return_state_name("S3")
        assert sm.tape == ["S2.A", "S1.A"]

        sm = HierarchicalGuardController()
        sm.B(0)
        sm.tape.clear()
        assert sm.state_info() == return_state_name("S2")
        sm.A(-5)
        assert sm.state_info() == return_state_name("S0")
        assert sm.tape == ["S2.A", "S1.A", "S0.A", "S.A"]

    
    def test_propagate_skips_levels(self):
        """Test that propagation of control skips levels that do not contain a
        given handler."""

        sm = HierarchicalGuardController()
        sm.B(0)
        sm.tape.clear()
        assert sm.state_info() == return_state_name("S2")
        sm.B(-5)
        assert sm.state_info() == return_state_name("S1")
        assert sm.tape == ["S2.B", "S0.B"]

        sm = HierarchicalGuardController()
        sm.B(0)
        sm.tape.clear()
        assert sm.state_info() == return_state_name("S2")
        sm.B(5)
        assert sm.state_info() == return_state_name("S2")
        assert sm.tape == ["S2.B", "S0.B", "S.B"]

    def test_conditional_return(self):
        """Test that conditional returns prevent propagation to parents."""

        sm = HierarchicalGuardController()
        sm.B(20)
        sm.tape.clear()
        assert sm.state_info() == return_state_name("S3")
        sm.A(5)
        assert sm.state_info() == return_state_name("S3")
        assert sm.tape == ["S3.A", "stop"]

        sm = HierarchicalGuardController()
        sm.B(20)
        sm.tape.clear()
        assert sm.state_info() ==  return_state_name("S3")
        sm.A(-5)
        assert sm.state_info() ==  return_state_name("S0")
        assert sm.tape == ["S3.A", "continue", "S.A"]

        sm = HierarchicalGuardController()
        sm.B(20)
        sm.tape.clear()
        assert sm.state_info() ==  return_state_name("S3")
        sm.B(-5)
        assert sm.state_info() ==  return_state_name("S3")
        assert sm.tape == ["S3.B", "stop"]

        sm = HierarchicalGuardController()
        sm.B(20)
        sm.tape.clear()
        assert sm.state_info() ==  return_state_name("S3")
        sm.B(5)
        assert sm.state_info() ==  return_state_name("S2")
        assert sm.tape == ["S3.B", "continue", "S.B"]