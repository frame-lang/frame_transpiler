from auto_inc_dec.auto_inc_dec import AutoIncDec

class AutoIncDecController(AutoIncDec):

    def print_it_do(self,val):
        print(val)

def test_state_transition():
    sm = AutoIncDecController()
    assert sm.a == 0
    assert sm.b == 0
    assert sm.c == 0
    sm.inc()
    assert sm.state_info() == '__autoincdec_state_S1'
    sm.dec()
    assert sm.state_info() == '__autoincdec_state_S1'

def test_increment():
    sm = AutoIncDecController()
    sm.inc()
    assert sm.a == 2  
    assert sm.b == 0  
    assert sm.c == 2  

def test_decrement():
    sm = AutoIncDecController()
    sm.dec()
    assert sm.a == -2 
    assert sm.b == 0  
    assert sm.c == -2  

def test_multiple_operations():
    sm = AutoIncDecController()
    sm.inc()
    sm.inc()
    sm.dec()
    sm.dec()
    sm.inc()
    sm.dec()
    assert sm.a == 0
    assert sm.b == 2
    assert sm.c == 0

