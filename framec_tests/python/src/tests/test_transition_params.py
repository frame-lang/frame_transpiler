from transition_params.transition_params import TransitParams


class TransitParamsController(TransitParams):

    def log_do(self, msg: str):
        return self.tape.append(msg)


def return_state_name(state):
    return f'__transitparams_state_{state}'


class TestTransitParams:

    def test_enter(self):
        sm = TransitParamsController()
        sm.Next()
        assert sm.tape == ["hi A"]

    def test_enter_and_exit(self):
        sm = TransitParamsController()
        sm.Next()
        sm.tape.clear()
        sm.Next()
        assert sm.tape == ["bye A", "hi B", "42"]
        sm.tape.clear()
        sm.Next()
        assert sm.tape == ["True", "bye B", "hi again A"]

    def change_state(self):
        sm = TransitParamsController()
        assert sm.state_info() == return_state_name("Init")
        sm.Change()
        assert sm.state_info() == return_state_name("A")
        sm.Change()
        assert sm.state_info() == return_state_name("B")
        sm.Change()
        assert sm.state_info() == return_state_name("A")
        assert len(sm.tape) == 0

    def change_and_transition(self):
        sm = TransitParamsController()
        sm.Change()
        assert sm.state_info() == return_state_name("A")
        assert len(sm.tape) == 0
        sm.Next()
        assert sm.state_info() == return_state_name("B")
        assert sm.tape == ["bye A", "hi B", "42"]
        sm.tape.clear()
        sm.Change()
        assert sm.state_info() == return_state_name("A")
        assert len(sm.tape) == 0
        sm.Change()
        sm.Next()
        assert sm.state_info() == return_state_name("A")
        assert sm.tape == ["True", "bye B", "hi again A"]