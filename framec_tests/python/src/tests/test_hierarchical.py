from hierarchical.hierarchical import Hierarchical


class HierarchicalController(Hierarchical):

    def enter_do(self, msg: str):
        self.enters.append(msg)

    def exit_do(self, msg: str):
        self.exits.append(msg)

    def log_do(self, msg: str):
        self.tape.append(msg)

def return_state_name(state: str) -> str:

    return f'__hierarchical_state_{state}'


class TestHierarchical:
    """Test hierarchical event handling and state transitions."""
    
    def test_enter_continue(self):
        """Test that a continue (`:>`) in a child enter handler calls the parent enter handler."""
        sm = HierarchicalController()
        sm.enters.clear()
        sm.A()
        assert sm.enters == ["S0", "S"]
        sm.enters.clear()
        sm.C()
        assert sm.enters == ["S2", "S0", "S"]
    
    def test_exit_continue(self):
        """Test that a continue (`:>`) in a child exit handler calls the parent exit handler."""
        sm = HierarchicalController()
        sm.A()
        sm.exits.clear()
        sm.C()
        assert sm.exits == ["S0", "S"]
        sm.exits.clear()
        sm.A()
        assert sm.exits == ["S2", "S0", "S"]

    def test_enter_return(self):
        """Test that a return (`^`) in a child enter handler *does not* call the parent enter handler."""

        sm = HierarchicalController()
        sm.enters.clear()
        sm.B()
        assert sm.enters == ["S1"]

        sm = HierarchicalController()
        sm.A()
        sm.A()
        assert sm.state_info() == return_state_name("T")
        sm.enters.clear()
        sm.C()
        assert sm.enters == ["S3", "S1"]

    def test_exit_return(self):
        """Test that a return (`^`) in a child exit handler *does not* call the parent exit handler."""

        sm = HierarchicalController()
        sm.B()
        assert sm.state_info() == return_state_name("S1")
        sm.exits.clear()
        sm.A()
        assert sm.exits == ["S1"]

        sm = HierarchicalController()
        sm.A()
        sm.A()
        sm.C()
        assert sm.state_info() == return_state_name("S3")
        sm.exits.clear()
        sm.B()
        assert sm.exits == ["S3", "S1"]
    
    def test_current_state_simple(self):
        """Test that location in a hierarchical state is represented 
        correctly. In this test, all state transitions are performed by the immediately matching handler."""

        sm = HierarchicalController()
        assert sm.state_info() == return_state_name("S")
        sm.A()
        assert sm.state_info() == return_state_name("S0")
        sm.A()
        assert sm.state_info() == return_state_name("T")
        sm.C()
        assert sm.state_info() == return_state_name("S3")
        sm.B()
        assert sm.state_info() == return_state_name("S2")

    def test_current_state_with_propagation(self):
        """
        Test that location in a hierarchical state is represented correctly. In this test, several
        state transitions propagate message handling to parents, either by implicit fall-through or
        explicit continues.
        """

        sm = HierarchicalController()
        assert sm.state_info() == return_state_name("S")
        sm.A()
        assert sm.state_info() == return_state_name("S0")
        sm.B()
        assert sm.state_info() == return_state_name("S1")
        sm.B()
        assert sm.state_info() == return_state_name("S1")
        sm.C()
        assert sm.state_info() == return_state_name("S1")
        sm.A()
        assert sm.state_info() == return_state_name("S0")
        sm.C()
        assert sm.state_info() == return_state_name("S2")
        sm.B()
        assert sm.state_info() == return_state_name("S1")

    def test_override_parent_handler(self):
        """Test that a handler in a child overrides the parent handler if the child handler ends with
        a return"""

        sm = HierarchicalController()
        sm.A()
        sm.tape.clear()
        sm.A()
        assert sm.state_info() == return_state_name("T")
        assert sm.tape == ["S0.A"]
        sm.C()
        sm.tape.clear()
        sm.B()
        assert sm.state_info() == return_state_name("S2")
        assert sm.tape == ["S3.B"]

    def test_before_parent_handler(self):
        """Test that a handler in a child propagates control to the parent handler if the child
        handler ends with a continue."""

        sm = HierarchicalController()
        sm.A()
        sm.tape.clear()
        sm.B()
        assert sm.state_info() == return_state_name("S1")
        assert sm.tape == ["S0.B", "S.B"]
        sm.tape.clear()
        sm.exits.clear()
        sm.enters.clear()
        sm.B()
        assert sm.state_info() == return_state_name("S1")
        assert sm.tape == ["S1.B", "S.B"]
        assert sm.exits == ["S1"]
        assert sm.enters == ["S1"]
        sm = HierarchicalController()
        sm.A()
        sm.C()
        assert sm.state_info() == return_state_name("S2")
        sm.tape.clear()
        sm.exits.clear()
        sm.enters.clear()
        sm.B()
        assert sm.state_info() == return_state_name("S1")
        assert sm.tape == ["S2.B", "S0.B", "S.B"]
        assert sm.exits == ["S2", "S0", "S"]
        assert sm.enters == ["S1"] 

    
    def test_defer_to_parent_handler(self):
        """Test that missing event handlers in children automatically propagate to parents."""
        sm = HierarchicalController()
        sm.B()
        assert sm.state_info() == return_state_name("S1")
        sm.tape.clear()
        sm.A()
        assert sm.state_info() == return_state_name("S0")
        assert sm.tape == ["S.A"]
        sm.A()
        sm.C()
        assert sm.state_info() == return_state_name("S3")
        sm.tape.clear()
        sm.A()
        assert sm.state_info() == return_state_name("S0")
        assert sm.tape == ["S.A"]

    def test_before_missing_handler(self):
        """Test that propagating control to a parent handler that doesn't handle the current message
        is a no-op.
        """
        sm = HierarchicalController()
        sm.B()
        assert sm.state_info() == return_state_name("S1")
        sm.tape.clear()
        sm.exits.clear()
        sm.enters.clear()
        sm.C()
        assert sm.state_info() == return_state_name("S1")
        assert sm.tape == ["S1.C"]
        assert len(sm.exits) == 0
        assert len(sm.enters) == 0
    
    def test_continue_after_transition_ignored(self):
        """Test that a continue after a transition statement is ignored."""
        
        sm = HierarchicalController()
        sm.A()
        sm.C()
        assert sm.state_info() == return_state_name("S2")
        sm.enters.clear()
        sm.tape.clear()
        sm.C()
        assert sm.state_info() == return_state_name("T")
        assert sm.enters == ["T"]
        assert sm.tape == ["S2.C"]
