from branch.branch import Branch


class BranchController(Branch):

    def log_do(self, msg: str):
        self.tape.append(msg)

class TestBranch:

    def test_simple_if_bool(self):
        sm = BranchController()
        sm.A()
        sm.OnBool(True)
        assert sm.state_info() == "__branch_state_F1"
        assert sm.tape == ["then 1", "then 2"]
        sm = BranchController()
        sm.A()
        sm.OnBool(False)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == ["else 1", "else 2"]
    
    def test_simple_if_init(self):
        sm = BranchController()
        sm.A()
        sm.OnInt(7)
        assert sm.state_info() == "__branch_state_F1"
        assert sm.tape == ["> 5", "< 10", "== 7"]

        sm = BranchController()
        sm.A()
        sm.OnInt(-3)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == ["<= 5", "< 10", "!= 7"]

        sm = BranchController()
        sm.A()
        sm.OnInt(12)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == ["> 5", ">= 10", "!= 7"]

    def test_negated_if_bool(self):
        sm = BranchController()
        sm.B()
        sm.OnBool(True)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == ["else 1", "else 2"]  

        sm = BranchController()
        sm.B()
        sm.OnBool(False)
        assert sm.state_info() == "__branch_state_F1"
        assert sm.tape == ["then 1", "then 2"]  

    def test_negated_if_int(self):

        sm = BranchController()
        sm.B()
        sm.OnInt(7)
        assert sm.state_info() == "__branch_state_F1"
        assert sm.tape == [">= 5", "<= 10", "== 7"]

        sm = BranchController()
        sm.B()
        sm.OnInt(5)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == [">= 5", "<= 10", "!= 7"]

        sm = BranchController()
        sm.B()
        sm.OnInt(10)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == [">= 5", "<= 10", "!= 7"]

        sm = BranchController()
        sm.B()
        sm.OnInt(0)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == ["< 5", "<= 10", "!= 7"]

        sm = BranchController()
        sm.B()
        sm.OnInt(100)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == [">= 5", "> 10", "!= 7"]
    
    def test_operator_precedence(self):

        sm = BranchController()
        sm.C()
        sm.OnInt(0)
        assert sm.tape == ["then 1", "else 2", "then 3", "then 4"]
        sm.tape = []
        sm.OnInt(7)
        assert sm.tape == ["else 1", "then 2", "else 3", "then 4"] 
        sm.tape = []
        sm.OnInt(-3)
        assert sm.tape == ["then 1", "else 2", "else 3", "else 4"]
        sm.tape = []
        sm.OnInt(12)
        assert sm.tape == ["else 1", "else 2", "then 3", "else 4"]

    def test_nested_if(self):

        sm = BranchController()
        sm.D()
        sm.OnInt(50)
        assert sm.state_info() == "__branch_state_F1"
        assert sm.tape == ["> 0", "< 100"]

        sm = BranchController()
        sm.D()
        sm.OnInt(200) 
        assert sm.state_info() == "__branch_state_NestedIf"
        assert sm.tape == ["> 0", ">= 100"]

        sm = BranchController()
        sm.D()
        sm.OnInt(-5)
        assert sm.state_info() == "__branch_state_NestedIf"
        assert sm.tape == ["<= 0", "> -10"]

        sm = BranchController()
        sm.D()
        sm.OnInt(-10)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == ["<= 0", "<= -10"]

    def test_guarded_transition(self):
        """Test that a transition guarded by a conditional expression triggers an 
        early return from the handler."""

        sm = BranchController()
        sm.E()
        sm.OnInt(5)
        assert sm.state_info() == "__branch_state_F3"
        assert sm.tape == ["-> $F3"]

        sm = BranchController()
        sm.E()
        sm.OnInt(15)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == ["-> $F2"]

        sm = BranchController()
        sm.E()
        sm.OnInt(115)
        assert sm.state_info() == "__branch_state_F1"
        assert sm.tape == ["-> $F1"]

    def test_nested_guarded_transition(self):

        sm = BranchController()
        sm.F()
        sm.OnInt(5)
        assert sm.state_info() == "__branch_state_F3"
        assert sm.tape == ["-> $F3"]

        sm = BranchController()
        sm.F()
        sm.OnInt(15)
        assert sm.state_info() == "__branch_state_F2"
        assert sm.tape == ["-> $F2"]

        sm = BranchController()
        sm.F()
        sm.OnInt(65)
        assert sm.state_info() == "__branch_state_F3"
        assert sm.tape == ["-> $F3"]

        sm = BranchController()
        sm.F()
        sm.OnInt(115)
        assert sm.state_info() == "__branch_state_F1"
        assert sm.tape == ["-> $F1"]