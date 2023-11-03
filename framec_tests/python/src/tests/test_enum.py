from enum_case.enum_case import EnumTest, EnumTest_Days

class EnumTestController(EnumTest):
    #
    # def entered_do(self, msg: str, val:int):
    #     self.days.append(f'{msg}={val}')


    def entered_do(self, msg: str, day):
        self.days.append(f'{msg}={day.value}')

    def left_do(self, msg: str, day):
        self.days.append(f'{msg}={day.value}')


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

# def test_enum_value_modification():

#     sm = EnumTestController()
#     assert EnumTest_Days.SUNDAY.value == 0
#     assert EnumTest_Days.MONDAY.value == 1

#     # Modify an enum value
#     EnumTest_Days.SUNDAY.value = 42
#     # new_sunday._value_ = 42 
#     assert EnumTest_Days.SUNDAY.value == 42
#     sm.entered_do("Modified Sunday", EnumTest_Days.SUNDAY.value)
#     assert "Modified Sunday=42" in sm.days


def test_attempt_to_modify_enum():
    try:
        EnumTest_Days.SUNDAY.value = 42
    except AttributeError:
        pass
    else:
        # If no exception is raised, the test should fail
        assert False, "Enum modification should raise an AttributeError"