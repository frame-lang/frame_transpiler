import pytest
from var_scope.var_scope import VarScope

class VarScopeController(VarScope):

    def log_do(self, s: str):
        self.tape.append(s)

    def do_nn(self):
        self.nn("|nn|[d]")


    def do_ny(self):
        self.ny("|ny|[d]")
    

    def do_yn(self):
        self.yn("|yn|[d]", "|yn|[x]")
    

    def do_yy( self):
        self.yy("|yy|[d]", "|yy|[x]")

def expected(state: str, msg: str, x: str):
    return [
        "#.a",
        f'${state}[b]',
        f'${state}.c',
        f'|{msg}|[d]',
        f'|{msg}|.e',
        x
    ]

class TestVarScope:
    """
    There are five different kinds of variables in Frame. Variables lower in
    the following list shadow variables higher in the list. Frame uses a
    variety of sigils to disambiguate potentially shadowed variables, which
    are indicated in parentheses below.
      * domain variables (`#.v`)
      * state parameters (`$[v]`)
      * state variables (`$.v`)
      * event handler parameters (`||[v]`)
      * event handler variables (`||.v`)

    This module tests that variable shadowing and the disambiguation sigils
    work as expected.
    """
    def test_no_shadowing(self):
        sm = VarScopeController()
        sm.to_nn()
        sm.do_nn()
        assert sm.tape == expected("NN", "nn", "#.x")
    
    def test_all_shadowing_scenarios(self):
        sm = VarScopeController()
        sm.to_nn()
        sm.do_ny()
        assert sm.tape == expected("NN", "ny", "|ny|.x")
        sm.tape.clear()
        sm.do_yn()
        assert sm.tape == expected("NN", "yn", "|yn|[x]")
        sm.tape.clear()
        sm.do_yy()
        assert sm.tape == expected("NN", "yy", "|yy|.x")

        sm = VarScopeController()
        sm.to_ny()
        sm.do_nn()
        assert sm.tape == expected("NY", "nn", "$NY.x")
        sm.tape.clear()
        sm.do_ny()
        assert sm.tape == expected("NY", "ny", "|ny|.x")
        sm.tape.clear()
        sm.do_yn()
        assert sm.tape == expected("NY", "yn", "|yn|[x]")
        sm.tape.clear()
        sm.do_yy()
        assert sm.tape == expected("NY", "yy", "|yy|.x")

        sm = VarScopeController()
        sm.to_yn()
        sm.do_nn()
        assert sm.tape == expected("YN", "nn", "$YN[x]")
        sm.tape.clear()
        sm.do_ny()
        assert sm.tape == expected("YN", "ny", "|ny|.x")
        sm.tape.clear()
        sm.do_yn()
        assert sm.tape == expected("YN", "yn", "|yn|[x]")
        sm.tape.clear()
        sm.do_yy()
        assert sm.tape == expected("YN", "yy", "|yy|.x")

        sm = VarScopeController()
        sm.to_yy()
        sm.do_nn()
        assert sm.tape == expected("YY", "nn", "$YY.x")
        sm.tape.clear()
        sm.do_ny()
        assert sm.tape == expected("YY", "ny", "|ny|.x")
        sm.tape.clear()
        sm.do_yn()
        assert sm.tape == expected("YY", "yn", "|yn|[x]")
        sm.tape.clear()
        sm.do_yy()
        assert sm.tape == expected("YY", "yy", "|yy|.x")

    @pytest.mark.skip()
    def test_disambiguation(self):
        sm = VarScopeController()
        sm.to_nn()
        sm.sigils("foo")
        assert sm.tape == ["#.x", "foo", "|sigils|.x"]
        sm = VarScopeController()
        sm.to_ny()
        sm.sigils("foo")
        assert sm.tape == ["#.x", "$NY.x", "foo", "|sigils|.x"]
        sm = VarScopeController()
        sm.to_yn()
        sm.sigils("foo")
        assert sm.tape == ["#.x", "$YN[x]", "foo", "|sigils|.x"]
        sm = VarScopeController()
        sm.to_yy()
        sm.sigils("foo")
        assert sm.tape == ["#.x", "$YY[x]", "$YY.x", "foo", "|sigils|.x"]
    
    