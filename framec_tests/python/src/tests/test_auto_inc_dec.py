from auto_inc_dec.auto_inc_dec import AutoIncDec
import math

class AutoIncDecController(AutoIncDec):

    def print_it_do(self,val):
        print(val)

# Test case to verify the state transition.
def test_state_transition():
    sm = AutoIncDecController()
    assert sm.a == 0
    assert sm.b == 0
    assert sm.c == 0
    sm.pre()
    assert sm.state_info() == '__autoincdec_state_Inc'
    sm.trans()
    assert sm.state_info() == '__autoincdec_state_Dec'

# Test case to verify incrementing behavior.
def test_increment():
    sm = AutoIncDecController()
    sm.pre()
    assert sm.a == 1 
    assert sm.b == 1
    sm.post()
    assert sm.c == 1

# Test case to verify decrementing behavior.
def test_decrement():
    sm = AutoIncDecController()
    sm.trans()
    sm.pre()
    assert sm.a == -1 
    assert sm.b == -1
    sm.post()
    assert sm.c == -1

# Test case to verify multiple operations.
def test_multiple_operations():
    sm = AutoIncDecController()
    sm.pre()
    sm.post()
    sm.trans()
    sm.pre()
    sm.post()
    sm.trans()
    sm.pre()
    sm.post()
    assert sm.a == 2
    assert sm.b == 1
    assert sm.c == 1

# Test case to verify behavior with negative values.
def test_negative_values():
    sm = AutoIncDecController()
    sm.trans()
    sm.pre()
    sm.post()
    assert sm.a < 0
    assert sm.b < 0
    assert sm.c < 0

# Test case to verify behavior with decimal and float types.
def test_decimal_and_float_types():
    sm = AutoIncDecController()
    sm.a = 1.5
    sm.pre()
    assert sm.a == 2.5
    sm.trans()
    sm.pre() 
    assert sm.a == 1.5

# Test case to verify behavior with NaN (Not-a-Number) value.
def test_nan():
    sm = AutoIncDecController()
    sm.a = float('nan')
    sm.pre()
    assert math.isnan(sm.a) 

# Test case to verify behavior with negative infinity.
def test_negative_infinite():
    sm = AutoIncDecController()
    sm.a = float('-inf')
    sm.post()
    assert sm.a == float('-inf') 
