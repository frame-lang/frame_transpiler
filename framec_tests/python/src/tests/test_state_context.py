from state_context.state_context import StateContextSm

class StateContextController(StateContextSm):

    def log_do(self, name: str, val: int):
        self.tape.append(f'{name}={val}')

def return_state_name(state: str) -> str:

    return f'__statecontextsm_state_{state}'


class TestStateContextSm:
    """Tests the interaction of several features (state variables, state parameters, event parameters,
    event variables, return values) that are implemented via state contexts"""
    def test_initial_state(self):

        sm = StateContextController()
        r = sm.Inc()
        assert r == 4
        sm.LogState()
        assert sm.tape == ["w=3", "w=4", "w=4"]
    
    def test_transition(self):
        sm = StateContextController()
        sm.Inc()
        sm.Inc()
        sm.tape.clear()

        sm.Start()
        assert sm.tape == ["a=3", "b=5", "x=15"]
        sm.tape.clear()

        sm.Inc()
        r = sm.Inc()
        assert r == 17
        assert sm.tape == ["x=16", "x=17"]
        sm.tape.clear()

        sm.Next(3)
        assert sm.tape == ["c=10", "x=27", "a=30", "y=17", "z=47"]
        sm.tape.clear()

        sm.Inc()
        sm.Inc()
        r = sm.Inc()
        assert r == 50
        assert sm.tape == ["z=48", "z=49", "z=50"]

    
    def test_change_state(self):

        sm = StateContextController()
        sm.Inc()
        sm.Inc()
        sm.Start()
        sm.tape.clear()

        sm.Inc()
        assert sm.tape == ["x=16"]
        sm.tape.clear()

        sm.Change(10)
        sm.LogState()
        assert sm.tape == ["y=26", "z=0"]
        sm.tape.clear()

        sm.Inc()
        sm.Change(100)
        sm.LogState()
        assert sm.state_info() == return_state_name("Init")
        assert sm.tape == ["z=1", "tmp=127", "w=0"]
