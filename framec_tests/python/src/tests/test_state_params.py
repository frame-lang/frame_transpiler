from state_params.state_params import StateParams

class StateParamsController(StateParams):

    def got_param_do(self, name: str, val: int):
        self.param_log.append(f'{name}={val}')


class TestStateParams:

    def test_single_parameter(self):
        sm = StateParamsController()
        sm.Next()
        sm.Log()
        assert sm.param_log == ["val=1"]

    def test_multiple_parameters(self):
        sm = StateParamsController()
        sm.Next()
        sm.Next()
        sm.Log()
        assert sm.param_log == ["left=1", "right=2"]

    def test_several_passes(self):
        sm = StateParamsController()
        sm.Next() # val=1
        sm.Next() # left=1, right=2
        sm.Next() # val=3
        sm.Log()
        sm.Prev() # left=4, right=3
        sm.Log()
        sm.Prev() # val=12
        sm.Log()
        assert sm.param_log == ["val=3", "left=4", "right=3", "val=12"]
