from enum_case.enum_case import EnumTest

class EnumTestController(EnumTest):

    def entered_do(self, msg: str, val:int):
        self.days.append(f'{msg}={val}')

    def left_do(self, msg: str, val:int):
        self.days.append(f'{msg}={val}')


def test_initial_state():
    sm = EnumTestController()
    assert sm.state_info() == '__enumtest_state_SUN'

def test_state_transition_():
    sm = EnumTestController()
    sm.A()
    assert sm.state_info() == '__enumtest_state_SUN'
    sm.B()
    assert sm.state_info() == '__enumtest_state_MON'
    assert "SUNDAY=0" in sm.days
    sm.A()
    sm.B()
    assert sm.state_info() == '__enumtest_state_TUE'
    assert "MONDAY=1" in sm.days
    sm.A()
    sm.B()
    assert sm.state_info() == '__enumtest_state_WED'
    assert "TUESDAY=2" in sm.days
    sm.A()
    sm.B()
    print(sm.state_info())
    assert sm.state_info() == '__enumtest_state_THR'
    assert "WEDNESDAY=3" in sm.days
    sm.A()
    sm.B()
    print(sm.state_info())
    assert sm.state_info() == '__enumtest_state_FRI'
    assert "THURSDAY=4" in sm.days
    sm.A()
    sm.B()
    assert sm.state_info() == '__enumtest_state_SUN'
    assert "FRIDAY=5" in sm.days
    assert "SUNDAY=0" in sm.days


